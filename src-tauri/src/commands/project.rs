use crate::parsers::{xliff, sdlxliff};
use crate::parsers::xliff::ContentPart;
use quick_xml::events::{Event, BytesStart, BytesEnd, BytesText};
use quick_xml::reader::Reader;
use quick_xml::writer::Writer;
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use std::sync::Mutex;
use tauri::State;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectData {
    pub path: String,
    pub name: String,
    pub source_language: String,
    pub target_language: String,
    pub segment_count: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SegmentData {
    pub id: u64,
    pub segment_number: u32,
    pub source_text: String,
    pub target_text: String,
    pub status: String,
    pub match_percentage: Option<f32>,
    pub match_origin: Option<String>,
    pub source_parts: Vec<ContentPart>,
    pub target_parts: Vec<ContentPart>,
}

/// Holds the currently loaded project's segments in memory.
pub struct AppState {
    pub segments: Mutex<Vec<SegmentData>>,
    pub project: Mutex<Option<ProjectData>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            segments: Mutex::new(Vec::new()),
            project: Mutex::new(None),
        }
    }
}

#[tauri::command]
pub async fn load_project(path: String, state: State<'_, AppState>) -> Result<ProjectData, String> {
    let raw_content = std::fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read file: {}", e))?;
    // Strip UTF-8 BOM if present
    let content = raw_content.strip_prefix('\u{feff}').unwrap_or(&raw_content);

    let extension = std::path::Path::new(&path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    let xliff_data = match extension.as_str() {
        "sdlxliff" => sdlxliff::parse_sdlxliff(&content)?,
        "xliff" | "xlf" | "mqxliff" | "mqxlz" => xliff::parse_xliff(&content)?,
        _ => return Err(format!("Unsupported file format: .{}", extension)),
    };

    println!("[Supervertaler] Parsed {} segments from {} (lang: {} → {})",
        xliff_data.segments.len(), extension, xliff_data.source_language, xliff_data.target_language);
    if let Some(first) = xliff_data.segments.first() {
        println!("[Supervertaler] First segment: src={:?}, tgt={:?}",
            &first.source[..first.source.len().min(80)],
            &first.target[..first.target.len().min(80)]);
    }

    let segments: Vec<SegmentData> = xliff_data
        .segments
        .iter()
        .enumerate()
        .map(|(i, seg)| {
            let status = if seg.target.is_empty() {
                "new".to_string()
            } else {
                "translated".to_string()
            };
            SegmentData {
                id: (i + 1) as u64,
                segment_number: (i + 1) as u32,
                source_text: seg.source.clone(),
                target_text: seg.target.clone(),
                status,
                match_percentage: None,
                match_origin: None,
                source_parts: seg.source_parts.clone(),
                target_parts: seg.target_parts.clone(),
            }
        })
        .collect();

    let project = ProjectData {
        path: path.clone(),
        name: std::path::Path::new(&path)
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default(),
        source_language: xliff_data.source_language,
        target_language: xliff_data.target_language,
        segment_count: segments.len(),
    };

    // Store in app state
    *state.segments.lock().unwrap() = segments;
    *state.project.lock().unwrap() = Some(project.clone());

    Ok(project)
}

#[tauri::command]
pub async fn get_segments(state: State<'_, AppState>) -> Result<Vec<SegmentData>, String> {
    let segments = state.segments.lock().unwrap().clone();
    Ok(segments)
}

/// Save updated target text back to the original bilingual file.
///
/// Strategy: re-read the original XML, walk through it event-by-event,
/// and when we encounter a <target> inside a <trans-unit>, replace its
/// content with the updated text from AppState. Everything else is
/// copied through verbatim, preserving all structure, namespaces,
/// attributes, and formatting.
#[tauri::command]
pub async fn save_project(state: State<'_, AppState>) -> Result<(), String> {
    let project = state.project.lock().unwrap().clone()
        .ok_or("No project loaded")?;
    let segments = state.segments.lock().unwrap().clone();

    let path = &project.path;
    let extension = std::path::Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    let segment_targets: Vec<&SegmentData> = segments.iter().collect();

    // Debug: log parts info for first few segments
    for (i, seg) in segments.iter().take(3).enumerate() {
        let tag_count = seg.target_parts.iter().filter(|p| !matches!(p, ContentPart::Text { .. })).count();
        println!("[Supervertaler] Segment {} target_parts: {} total, {} tags, text={:?}",
            i + 1, seg.target_parts.len(), tag_count,
            &seg.target_text[..seg.target_text.len().min(50)]);
    }

    // Read the original file
    let raw_content = std::fs::read_to_string(path)
        .map_err(|e| format!("Failed to read file for saving: {}", e))?;
    let content = raw_content.strip_prefix('\u{feff}').unwrap_or(&raw_content);

    let is_sdlxliff = extension == "sdlxliff";

    // Write updated content
    let updated = rewrite_xliff_targets(content, &segment_targets, is_sdlxliff)?;

    // Write back, preserving BOM if original had one
    let mut output = String::new();
    if raw_content.starts_with('\u{feff}') {
        output.push('\u{feff}');
    }
    output.push_str(&updated);

    std::fs::write(path, output.as_bytes())
        .map_err(|e| format!("Failed to write file: {}", e))?;

    println!("[Supervertaler] Saved {} segments to {}", segments.len(), path);
    Ok(())
}

/// Rewrite target elements in an XLIFF file with updated translations.
///
/// Walks the XML event-by-event. Inside each <trans-unit>, tracks the
/// segment index and replaces <target> content. For segments with
/// ContentParts (inline tags), reconstructs the target with tags.
/// For plain text segments, writes the text directly.
fn rewrite_xliff_targets(
    content: &str,
    segments: &[&SegmentData],
    is_sdlxliff: bool,
) -> Result<String, String> {
    let mut reader = Reader::from_str(content);
    reader.config_mut().trim_text(false);

    let mut writer = Writer::new(Cursor::new(Vec::new()));

    let mut segment_index: usize = 0; // Tracks which segment we're on
    let mut in_trans_unit = false;
    let mut in_target = false;
    let mut in_seg_source = false;
    let mut skip_target_content = false;
    let mut target_depth: u32 = 0;
    let mut has_source_content = false; // Track if this trans-unit has actual content

    // For SDLXLIFF: track if we've seen source content to skip empty trans-units
    let mut current_source_text = String::new();
    let mut in_source = false;

    // MemoQ minorversions: pass through verbatim, don't treat as segments
    let mut minor_version_depth: u32 = 0;

    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let local_name = e.local_name();
                // Track entry into mq:minorversions — pass through verbatim
                if local_name.as_ref() == b"minorversions" || local_name.as_ref() == b"historical-unit" {
                    minor_version_depth += 1;
                }
                if minor_version_depth > 0 {
                    writer.write_event(Event::Start(e.clone()))
                        .map_err(|e| format!("Write error: {}", e))?;
                    buf.clear();
                    continue;
                }
                match local_name.as_ref() {
                    b"trans-unit" => {
                        in_trans_unit = true;
                        has_source_content = false;
                        current_source_text.clear();
                        writer.write_event(Event::Start(e.clone()))
                            .map_err(|e| format!("Write error: {}", e))?;
                        continue;
                    }
                    b"seg-source" if in_trans_unit && is_sdlxliff => {
                        in_seg_source = true;
                        writer.write_event(Event::Start(e.clone()))
                            .map_err(|e| format!("Write error: {}", e))?;
                        continue;
                    }
                    b"source" if in_trans_unit && !in_seg_source && !in_target => {
                        in_source = true;
                        writer.write_event(Event::Start(e.clone()))
                            .map_err(|e| format!("Write error: {}", e))?;
                        continue;
                    }
                    b"target" if in_trans_unit && !in_seg_source && !in_source => {
                        in_target = true;
                        target_depth = 1;

                        // Check if this trans-unit maps to a segment
                        // (SDLXLIFF skips empty trans-units in parsing)
                        if is_sdlxliff && !has_source_content {
                            // This trans-unit was skipped during parsing — pass through unchanged
                            skip_target_content = false;
                            writer.write_event(Event::Start(e.clone()))
                                .map_err(|e| format!("Write error: {}", e))?;
                            continue;
                        }

                        if segment_index < segments.len() {
                            let seg = segments[segment_index];
                            skip_target_content = true;

                            // Write the <target> opening tag
                            writer.write_event(Event::Start(e.clone()))
                                .map_err(|e| format!("Write error: {}", e))?;

                            // Write the updated content
                            if !seg.target_parts.is_empty()
                                && seg.target_parts.iter().any(|p| !matches!(p, ContentPart::Text { .. }))
                            {
                                // Has inline tags — reconstruct with parts
                                write_content_parts(&mut writer, &seg.target_parts)?;
                            } else {
                                // Plain text — write directly
                                let text = BytesText::new(&seg.target_text);
                                writer.write_event(Event::Text(text))
                                    .map_err(|e| format!("Write error: {}", e))?;
                            }
                            continue;
                        } else {
                            // More trans-units in file than segments — pass through
                            skip_target_content = false;
                            writer.write_event(Event::Start(e.clone()))
                                .map_err(|e| format!("Write error: {}", e))?;
                            continue;
                        }
                    }
                    _ => {
                        if in_target && skip_target_content {
                            target_depth += 1;
                            continue; // Skip original target content
                        }
                        if in_source || in_seg_source {
                            current_source_text.push_str("x"); // Mark as having content
                        }
                    }
                }
                writer.write_event(Event::Start(e.clone()))
                    .map_err(|e| format!("Write error: {}", e))?;
            }
            Ok(Event::End(ref e)) => {
                let local_name = e.local_name();
                // Track exit from mq:minorversions — pass through verbatim
                if local_name.as_ref() == b"minorversions" || local_name.as_ref() == b"historical-unit" {
                    if minor_version_depth > 0 {
                        minor_version_depth -= 1;
                    }
                    writer.write_event(Event::End(e.clone()))
                        .map_err(|e| format!("Write error: {}", e))?;
                    buf.clear();
                    continue;
                }
                if minor_version_depth > 0 {
                    writer.write_event(Event::End(e.clone()))
                        .map_err(|e| format!("Write error: {}", e))?;
                    buf.clear();
                    continue;
                }
                match local_name.as_ref() {
                    b"trans-unit" => {
                        if in_trans_unit {
                            // Only increment segment index for trans-units that had content
                            if !is_sdlxliff || has_source_content {
                                segment_index += 1;
                            }
                            in_trans_unit = false;
                        }
                        writer.write_event(Event::End(e.clone()))
                            .map_err(|e| format!("Write error: {}", e))?;
                        continue;
                    }
                    b"seg-source" if in_seg_source => {
                        in_seg_source = false;
                        has_source_content = !current_source_text.trim().is_empty();
                        writer.write_event(Event::End(e.clone()))
                            .map_err(|e| format!("Write error: {}", e))?;
                        continue;
                    }
                    b"source" if in_source => {
                        in_source = false;
                        if !is_sdlxliff {
                            has_source_content = !current_source_text.trim().is_empty();
                        }
                        writer.write_event(Event::End(e.clone()))
                            .map_err(|e| format!("Write error: {}", e))?;
                        continue;
                    }
                    b"target" if in_target => {
                        target_depth -= 1;
                        if target_depth == 0 {
                            in_target = false;
                            skip_target_content = false;
                            writer.write_event(Event::End(e.clone()))
                                .map_err(|e| format!("Write error: {}", e))?;
                            continue;
                        }
                        if skip_target_content {
                            continue; // Skip nested target end tags
                        }
                    }
                    _ => {
                        if in_target && skip_target_content {
                            target_depth = target_depth.saturating_sub(1);
                            continue; // Skip original target content
                        }
                    }
                }
                writer.write_event(Event::End(e.clone()))
                    .map_err(|e| format!("Write error: {}", e))?;
            }
            Ok(Event::Text(ref e)) => {
                if minor_version_depth > 0 {
                    writer.write_event(Event::Text(e.clone()))
                        .map_err(|e| format!("Write error: {}", e))?;
                    buf.clear();
                    continue;
                }
                if in_target && skip_target_content {
                    continue; // Skip — we already wrote the updated content
                }
                if in_source || in_seg_source {
                    let text = e.unescape().unwrap_or_default().to_string();
                    current_source_text.push_str(&text);
                    has_source_content = has_source_content || !text.trim().is_empty();
                }
                writer.write_event(Event::Text(e.clone()))
                    .map_err(|e| format!("Write error: {}", e))?;
            }
            Ok(Event::Empty(ref e)) => {
                if minor_version_depth > 0 {
                    writer.write_event(Event::Empty(e.clone()))
                        .map_err(|e| format!("Write error: {}", e))?;
                    buf.clear();
                    continue;
                }
                let local_name = e.local_name();
                match local_name.as_ref() {
                    b"target" if in_trans_unit && !in_seg_source && !in_source => {
                        // Self-closing <target/> — need to check if we have a translation now
                        if (!is_sdlxliff || has_source_content) && segment_index < segments.len() {
                            let seg = segments[segment_index];
                            if !seg.target_text.is_empty() {
                                // We have a translation — expand to <target>text</target>
                                // Copy attributes from the empty element
                                let mut new_start = BytesStart::new(
                                    std::str::from_utf8(e.name().into_inner())
                                        .unwrap_or("target"),
                                );
                                for attr in e.attributes().flatten() {
                                    new_start.push_attribute(attr);
                                }
                                writer.write_event(Event::Start(new_start))
                                    .map_err(|err| format!("Write error: {}", err))?;

                                if !seg.target_parts.is_empty()
                                    && seg.target_parts.iter().any(|p| !matches!(p, ContentPart::Text { .. }))
                                {
                                    write_content_parts(&mut writer, &seg.target_parts)?;
                                } else {
                                    let text = BytesText::new(&seg.target_text);
                                    writer.write_event(Event::Text(text))
                                        .map_err(|err| format!("Write error: {}", err))?;
                                }

                                let end = BytesEnd::new(
                                    std::str::from_utf8(e.name().into_inner())
                                        .unwrap_or("target"),
                                );
                                writer.write_event(Event::End(end))
                                    .map_err(|err| format!("Write error: {}", err))?;
                                continue;
                            }
                        }
                        // No translation — keep as self-closing
                        writer.write_event(Event::Empty(e.clone()))
                            .map_err(|err| format!("Write error: {}", err))?;
                        continue;
                    }
                    _ => {
                        if in_target && skip_target_content {
                            continue; // Skip original target content
                        }
                    }
                }
                writer.write_event(Event::Empty(e.clone()))
                    .map_err(|e| format!("Write error: {}", e))?;
            }
            Ok(Event::CData(ref e)) => {
                if minor_version_depth > 0 {
                    writer.write_event(Event::CData(e.clone()))
                        .map_err(|e| format!("Write error: {}", e))?;
                    buf.clear();
                    continue;
                }
                if in_target && skip_target_content {
                    continue;
                }
                writer.write_event(Event::CData(e.clone()))
                    .map_err(|e| format!("Write error: {}", e))?;
            }
            Ok(Event::Decl(ref e)) => {
                writer.write_event(Event::Decl(e.clone()))
                    .map_err(|e| format!("Write error: {}", e))?;
            }
            Ok(Event::PI(ref e)) => {
                writer.write_event(Event::PI(e.clone()))
                    .map_err(|e| format!("Write error: {}", e))?;
            }
            Ok(Event::Comment(ref e)) => {
                writer.write_event(Event::Comment(e.clone()))
                    .map_err(|e| format!("Write error: {}", e))?;
            }
            Ok(Event::DocType(ref e)) => {
                writer.write_event(Event::DocType(e.clone()))
                    .map_err(|e| format!("Write error: {}", e))?;
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(format!("XML parse error during save: {}", e)),
        }
        buf.clear();
    }

    let result = writer.into_inner().into_inner();
    String::from_utf8(result).map_err(|e| format!("UTF-8 encoding error: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::xliff::parse_xliff;

    fn make_segment(id: u64, source: &str, target: &str, source_parts: Vec<ContentPart>, target_parts: Vec<ContentPart>) -> SegmentData {
        SegmentData {
            id,
            segment_number: id as u32,
            source_text: source.to_string(),
            target_text: target.to_string(),
            status: "translated".to_string(),
            match_percentage: None,
            match_origin: None,
            source_parts,
            target_parts,
        }
    }

    #[test]
    fn test_save_plain_text_roundtrip() {
        let xliff = r#"<?xml version="1.0" encoding="UTF-8"?>
<xliff version="1.2" xmlns="urn:oasis:names:tc:xliff:document:1.2">
  <file original="test.docx" source-language="en-US" target-language="nl-NL" datatype="plaintext">
    <body>
      <trans-unit id="1">
        <source>Hello world</source>
        <target>Hallo wereld</target>
      </trans-unit>
      <trans-unit id="2">
        <source>Goodbye</source>
        <target>Tot ziens</target>
      </trans-unit>
    </body>
  </file>
</xliff>"#;

        let seg1 = make_segment(1, "Hello world", "Hallo wereld UPDATED", vec![], vec![]);
        let seg2 = make_segment(2, "Goodbye", "Tot ziens", vec![], vec![]);
        let segments: Vec<&SegmentData> = vec![&seg1, &seg2];

        let result = rewrite_xliff_targets(xliff, &segments, false).unwrap();

        // Verify the updated target is in the output
        assert!(result.contains("Hallo wereld UPDATED"), "Updated target text not found");
        assert!(result.contains("Tot ziens"), "Second segment target should be preserved");
        // Verify source is unchanged
        assert!(result.contains("<source>Hello world</source>"), "Source should be unchanged");
    }

    #[test]
    fn test_save_self_closing_target_expanded() {
        let xliff = r#"<?xml version="1.0" encoding="UTF-8"?>
<xliff version="1.2" xmlns="urn:oasis:names:tc:xliff:document:1.2">
  <file original="test.docx" source-language="en-US" target-language="nl-NL" datatype="plaintext">
    <body>
      <trans-unit id="1">
        <source>Hello</source>
        <target/>
      </trans-unit>
    </body>
  </file>
</xliff>"#;

        let seg = make_segment(1, "Hello", "Hallo", vec![], vec![]);
        let segments: Vec<&SegmentData> = vec![&seg];

        let result = rewrite_xliff_targets(xliff, &segments, false).unwrap();

        // Self-closing <target/> should be expanded to <target>Hallo</target>
        assert!(result.contains("<target>Hallo</target>"), "Self-closing target should be expanded: {}", result);
        assert!(!result.contains("<target/>"), "Self-closing target should no longer exist");
    }

    #[test]
    fn test_save_with_inline_tags_roundtrip() {
        let xliff = r#"<?xml version="1.0" encoding="UTF-8"?>
<xliff version="1.2" xmlns="urn:oasis:names:tc:xliff:document:1.2">
  <file original="test.docx" source-language="en-US" target-language="nl-NL" datatype="plaintext">
    <body>
      <trans-unit id="1">
        <source>Click <bpt id="1">{\b}</bpt>here<ept id="1">{\b0}</ept> to continue</source>
        <target>Klik <bpt id="1">{\b}</bpt>hier<ept id="1">{\b0}</ept> om verder te gaan</target>
      </trans-unit>
    </body>
  </file>
</xliff>"#;

        // Parse to get the content parts
        let parsed = parse_xliff(xliff).unwrap();
        let parsed_seg = &parsed.segments[0];

        let seg = make_segment(
            1,
            &parsed_seg.source,
            &parsed_seg.target,
            parsed_seg.source_parts.clone(),
            parsed_seg.target_parts.clone(),
        );
        let segments: Vec<&SegmentData> = vec![&seg];

        let result = rewrite_xliff_targets(xliff, &segments, false).unwrap();

        // The target should contain the inline tags
        assert!(result.contains("<bpt id=\"1\">{\\b}</bpt>"), "bpt tag should be preserved in output: {}", result);
        assert!(result.contains("<ept id=\"1\">{\\b0}</ept>"), "ept tag should be preserved in output: {}", result);
        assert!(result.contains("hier"), "Target text should be preserved");
    }

    #[test]
    fn test_save_preserves_xml_declaration() {
        let xliff = r#"<?xml version="1.0" encoding="UTF-8"?>
<xliff version="1.2" xmlns="urn:oasis:names:tc:xliff:document:1.2">
  <file original="test.docx" source-language="en-US" target-language="nl-NL" datatype="plaintext">
    <body>
      <trans-unit id="1">
        <source>Test</source>
        <target>Test NL</target>
      </trans-unit>
    </body>
  </file>
</xliff>"#;

        let seg = make_segment(1, "Test", "Test NL", vec![], vec![]);
        let segments: Vec<&SegmentData> = vec![&seg];

        let result = rewrite_xliff_targets(xliff, &segments, false).unwrap();
        assert!(result.starts_with("<?xml"), "XML declaration should be preserved");
    }

    #[test]
    fn test_save_mqxliff_preserves_target_tags() {
        // Simulates a MemoQ MQXLIFF file with bpt/ept in target + minorversions
        let xliff = r#"<?xml version="1.0" encoding="UTF-8"?>
<xliff version="1.2" xmlns="urn:oasis:names:tc:xliff:document:1.2" xmlns:mq="MQXliff">
  <file original="test.docx" source-language="nl-be" target-language="en-gb" datatype="plaintext">
    <body>
      <trans-unit id="1" mq:status="ManuallyConfirmed">
        <source xml:space="preserve"><bpt id="1" ctype="bold">{}</bpt>TITEL<ept id="1">{}</ept></source>
        <target xml:space="preserve"><bpt id="1" ctype="bold">{}</bpt>TITLE<ept id="1">{}</ept></target>
        <mq:minorversions>
          <mq:historical-unit mq:status="NotStarted">
            <source xml:space="preserve"><bpt id="1" ctype="bold">{}</bpt>TITEL<ept id="1">{}</ept></source>
            <target xml:space="preserve"></target>
          </mq:historical-unit>
        </mq:minorversions>
      </trans-unit>
    </body>
  </file>
</xliff>"#;

        // Parse to get parts
        let parsed = parse_xliff(xliff).unwrap();
        assert_eq!(parsed.segments.len(), 1, "Should have exactly 1 segment");
        let seg = &parsed.segments[0];

        // Verify parser extracted tags from target
        let has_target_tags = seg.target_parts.iter().any(|p| !matches!(p, ContentPart::Text { .. }));
        assert!(has_target_tags, "Parser should extract tags from target. Parts: {:?}", seg.target_parts);

        // Now save with those parts
        let segment = make_segment(
            1, &seg.source, &seg.target,
            seg.source_parts.clone(), seg.target_parts.clone(),
        );
        let segments: Vec<&SegmentData> = vec![&segment];
        let result = rewrite_xliff_targets(xliff, &segments, false).unwrap();

        // Target must still contain bpt/ept
        assert!(result.contains("<bpt id=\"1\""), "bpt tag must be in saved target: {}", result);
        assert!(result.contains("<ept id=\"1\""), "ept tag must be in saved target: {}", result);
    }
}

/// Write ContentParts (text + inline tags) into the XML writer.
fn write_content_parts(
    writer: &mut Writer<Cursor<Vec<u8>>>,
    parts: &[ContentPart],
) -> Result<(), String> {
    for part in parts {
        match part {
            ContentPart::Text { text } => {
                let bytes = BytesText::new(text);
                writer.write_event(Event::Text(bytes))
                    .map_err(|e| format!("Write error: {}", e))?;
            }
            ContentPart::TagOpen { display, .. }
            | ContentPart::TagClose { display, .. }
            | ContentPart::Standalone { display, .. } => {
                // Write the original tag markup as raw bytes
                // The display field contains the original tag content
                // For now, write as raw text — this preserves the original structure
                writer.get_mut().get_mut().extend_from_slice(display.as_bytes());
            }
        }
    }
    Ok(())
}
