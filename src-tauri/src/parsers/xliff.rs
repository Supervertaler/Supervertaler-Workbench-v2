use quick_xml::events::Event;
use quick_xml::reader::Reader;
use serde::{Deserialize, Serialize};

/// A segment of content that is either plain text or a tag marker.
/// Used to render inline tags (bold, italic, placeholders, etc.) in the grid.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ContentPart {
    /// Plain text run
    #[serde(rename = "text")]
    Text { text: String },
    /// Opening tag of a paired element (e.g., <bpt>, <g>)
    #[serde(rename = "tag_open")]
    TagOpen {
        id: String,
        tag_type: String,
        display: String,
    },
    /// Closing tag of a paired element (e.g., <ept>, </g>)
    #[serde(rename = "tag_close")]
    TagClose {
        id: String,
        tag_type: String,
        display: String,
    },
    /// Self-closing / standalone tag (e.g., <x/>, <ph>, <it>)
    #[serde(rename = "standalone")]
    Standalone {
        id: String,
        tag_type: String,
        display: String,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct XliffFile {
    pub source_language: String,
    pub target_language: String,
    pub original: String,
    pub segments: Vec<XliffSegment>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct XliffSegment {
    pub id: String,
    pub source: String,
    pub target: String,
    pub source_parts: Vec<ContentPart>,
    pub target_parts: Vec<ContentPart>,
}

/// Guess the formatting type from bpt/ept inner text or ctype attribute.
pub fn guess_tag_type(inner_text: &str, ctype: &str) -> String {
    let lower = inner_text.to_lowercase();
    let ctype_lower = ctype.to_lowercase();

    if ctype_lower.contains("bold") || lower.contains("\\b") || lower.contains("<b>") || lower.contains("<b ") || lower.contains("<strong") {
        "bold".to_string()
    } else if ctype_lower.contains("italic") || lower.contains("\\i") || lower.contains("<i>") || lower.contains("<i ") || lower.contains("<em") {
        "italic".to_string()
    } else if ctype_lower.contains("underline") || lower.contains("\\ul") || lower.contains("<u>") || lower.contains("<u ") {
        "underline".to_string()
    } else if ctype_lower.contains("superscript") || lower.contains("<sup") {
        "superscript".to_string()
    } else if ctype_lower.contains("subscript") || lower.contains("<sub") {
        "subscript".to_string()
    } else if ctype_lower.contains("link") || lower.contains("<a ") || lower.contains("href") {
        "link".to_string()
    } else if ctype_lower.contains("font") || lower.contains("\\f") || lower.contains("<span") || lower.contains("\\cf") {
        "formatting".to_string()
    } else if !ctype.is_empty() {
        ctype_lower
    } else if inner_text.is_empty() {
        "formatting".to_string()
    } else {
        "formatting".to_string()
    }
}

/// Get attribute value from a quick-xml event
pub fn get_attr(e: &quick_xml::events::BytesStart, name: &[u8]) -> String {
    for attr in e.attributes().flatten() {
        if attr.key.local_name().as_ref() == name {
            return attr.unescape_value().unwrap_or_default().to_string();
        }
    }
    String::new()
}

/// Parse an XLIFF 1.2 file and extract translation units.
/// Handles inline tags: bpt (begin paired tag), ept (end paired tag),
/// ph (placeholder), it (isolated tag), g (group), x (standalone).
pub fn parse_xliff(content: &str) -> Result<XliffFile, String> {
    let mut reader = Reader::from_str(content);
    reader.config_mut().trim_text(false); // Don't trim — we need whitespace

    let mut source_language = String::new();
    let mut target_language = String::new();
    let mut original = String::new();
    let mut segments: Vec<XliffSegment> = Vec::new();

    let mut current_id = String::new();
    let mut in_trans_unit = false;
    let mut in_source = false;
    let mut in_target = false;

    // For collecting content parts and inline tag text
    let mut source_parts: Vec<ContentPart> = Vec::new();
    let mut target_parts: Vec<ContentPart> = Vec::new();
    let mut source_text = String::new();
    let mut target_text = String::new();

    // Track when we're inside an inline tag element to capture its inner text
    let mut in_inline_tag: Option<InlineTagContext> = None;
    let mut inline_tag_text = String::new();

    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let local_name = e.local_name();
                match local_name.as_ref() {
                    b"file" => {
                        for attr in e.attributes().flatten() {
                            match attr.key.local_name().as_ref() {
                                b"source-language" => {
                                    source_language = attr
                                        .unescape_value()
                                        .unwrap_or_default()
                                        .to_string();
                                }
                                b"target-language" => {
                                    target_language = attr
                                        .unescape_value()
                                        .unwrap_or_default()
                                        .to_string();
                                }
                                b"original" => {
                                    original = attr
                                        .unescape_value()
                                        .unwrap_or_default()
                                        .to_string();
                                }
                                _ => {}
                            }
                        }
                    }
                    b"trans-unit" => {
                        in_trans_unit = true;
                        current_id.clear();
                        source_parts.clear();
                        target_parts.clear();
                        source_text.clear();
                        target_text.clear();
                        for attr in e.attributes().flatten() {
                            if attr.key.local_name().as_ref() == b"id" {
                                current_id = attr
                                    .unescape_value()
                                    .unwrap_or_default()
                                    .to_string();
                            }
                        }
                    }
                    b"source" if in_trans_unit && !in_source && !in_target => {
                        in_source = true;
                    }
                    b"target" if in_trans_unit && !in_source && !in_target => {
                        in_target = true;
                    }
                    // Inline tags inside source/target
                    b"bpt" if in_source || in_target => {
                        let id = get_attr(e, b"id");
                        let ctype = get_attr(e, b"ctype");
                        in_inline_tag = Some(InlineTagContext::Bpt { id, ctype });
                        inline_tag_text.clear();
                    }
                    b"ept" if in_source || in_target => {
                        let id = get_attr(e, b"id");
                        let rid = get_attr(e, b"rid");
                        in_inline_tag = Some(InlineTagContext::Ept { id, rid });
                        inline_tag_text.clear();
                    }
                    b"ph" if in_source || in_target => {
                        let id = get_attr(e, b"id");
                        let ctype = get_attr(e, b"ctype");
                        in_inline_tag = Some(InlineTagContext::Ph { id, ctype });
                        inline_tag_text.clear();
                    }
                    b"it" if in_source || in_target => {
                        let id = get_attr(e, b"id");
                        let ctype = get_attr(e, b"ctype");
                        let pos = get_attr(e, b"pos");
                        in_inline_tag = Some(InlineTagContext::It { id, ctype, pos });
                        inline_tag_text.clear();
                    }
                    b"g" if in_source || in_target => {
                        // <g> wraps content — emit TagOpen now
                        let id = get_attr(e, b"id");
                        let ctype = get_attr(e, b"ctype");
                        let tag_type = if !ctype.is_empty() {
                            guess_tag_type("", &ctype)
                        } else {
                            "formatting".to_string()
                        };
                        let part = ContentPart::TagOpen {
                            id: id.clone(),
                            tag_type,
                            display: format!("<g id=\"{}\">", id),
                        };
                        if in_source {
                            source_parts.push(part);
                        } else if in_target {
                            target_parts.push(part);
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::End(ref e)) => {
                let local_name = e.local_name();
                match local_name.as_ref() {
                    b"trans-unit" => {
                        if in_trans_unit {
                            segments.push(XliffSegment {
                                id: current_id.clone(),
                                source: source_text.trim().to_string(),
                                target: target_text.trim().to_string(),
                                source_parts: coalesce_parts(&source_parts),
                                target_parts: coalesce_parts(&target_parts),
                            });
                            in_trans_unit = false;
                        }
                    }
                    b"source" if in_source => {
                        in_source = false;
                    }
                    b"target" if in_target => {
                        in_target = false;
                    }
                    b"bpt" if in_inline_tag.is_some() => {
                        if let Some(InlineTagContext::Bpt { id, ctype }) = in_inline_tag.take() {
                            let tag_type = guess_tag_type(&inline_tag_text, &ctype);
                            let display = if inline_tag_text.is_empty() {
                                format!("<bpt id=\"{}\">", id)
                            } else {
                                inline_tag_text.clone()
                            };
                            let part = ContentPart::TagOpen { id, tag_type, display };
                            if in_source {
                                source_parts.push(part);
                            } else if in_target {
                                target_parts.push(part);
                            }
                        }
                        inline_tag_text.clear();
                    }
                    b"ept" if in_inline_tag.is_some() => {
                        if let Some(InlineTagContext::Ept { id, rid }) = in_inline_tag.take() {
                            let tag_type = guess_tag_type(&inline_tag_text, "");
                            let display = if inline_tag_text.is_empty() {
                                format!("<ept id=\"{}\">", id)
                            } else {
                                inline_tag_text.clone()
                            };
                            // Use rid (reference to bpt) as pairing ID when available,
                            // so {1} opens and {/1} closes — not {/3}
                            let pair_id = if !rid.is_empty() { rid } else { id };
                            let part = ContentPart::TagClose { id: pair_id, tag_type, display };
                            if in_source {
                                source_parts.push(part);
                            } else if in_target {
                                target_parts.push(part);
                            }
                        }
                        inline_tag_text.clear();
                    }
                    b"ph" if in_inline_tag.is_some() => {
                        if let Some(InlineTagContext::Ph { id, ctype }) = in_inline_tag.take() {
                            let tag_type = guess_tag_type(&inline_tag_text, &ctype);
                            let display = if inline_tag_text.is_empty() {
                                format!("<ph id=\"{}\"/>", id)
                            } else {
                                inline_tag_text.clone()
                            };
                            let part = ContentPart::Standalone { id, tag_type, display };
                            if in_source {
                                source_parts.push(part);
                            } else if in_target {
                                target_parts.push(part);
                            }
                        }
                        inline_tag_text.clear();
                    }
                    b"it" if in_inline_tag.is_some() => {
                        if let Some(InlineTagContext::It { id, ctype, pos }) = in_inline_tag.take() {
                            let tag_type = guess_tag_type(&inline_tag_text, &ctype);
                            let display = if inline_tag_text.is_empty() {
                                format!("<it id=\"{}\" pos=\"{}\"/>", id, pos)
                            } else {
                                inline_tag_text.clone()
                            };
                            let part = if pos == "close" {
                                ContentPart::TagClose { id, tag_type, display }
                            } else {
                                ContentPart::TagOpen { id, tag_type, display }
                            };
                            if in_source {
                                source_parts.push(part);
                            } else if in_target {
                                target_parts.push(part);
                            }
                        }
                        inline_tag_text.clear();
                    }
                    b"g" if (in_source || in_target) => {
                        let part = ContentPart::TagClose {
                            id: String::new(),
                            tag_type: "formatting".to_string(),
                            display: "</g>".to_string(),
                        };
                        if in_source {
                            source_parts.push(part);
                        } else if in_target {
                            target_parts.push(part);
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::Text(ref e)) => {
                let text = e.unescape().unwrap_or_default().to_string();
                if in_inline_tag.is_some() {
                    // Text inside a bpt/ept/ph/it element — capture for display
                    inline_tag_text.push_str(&text);
                } else if in_source {
                    source_text.push_str(&text);
                    source_parts.push(ContentPart::Text { text });
                } else if in_target {
                    target_text.push_str(&text);
                    target_parts.push(ContentPart::Text { text });
                }
            }
            Ok(Event::Empty(ref e)) => {
                let local_name = e.local_name();
                match local_name.as_ref() {
                    b"target" if in_trans_unit && !in_source => {
                        // Self-closing <target/> means empty target
                        target_text.clear();
                        target_parts.clear();
                    }
                    b"x" if in_source || in_target => {
                        let id = get_attr(e, b"id");
                        let ctype = get_attr(e, b"ctype");
                        let tag_type = if !ctype.is_empty() {
                            guess_tag_type("", &ctype)
                        } else {
                            "placeholder".to_string()
                        };
                        let part = ContentPart::Standalone {
                            id: id.clone(),
                            tag_type,
                            display: format!("<x id=\"{}\"/>", id),
                        };
                        if in_source {
                            source_parts.push(part);
                        } else if in_target {
                            target_parts.push(part);
                        }
                    }
                    b"ph" if (in_source || in_target) && in_inline_tag.is_none() => {
                        let id = get_attr(e, b"id");
                        let ctype = get_attr(e, b"ctype");
                        let tag_type = if !ctype.is_empty() {
                            guess_tag_type("", &ctype)
                        } else {
                            "placeholder".to_string()
                        };
                        let part = ContentPart::Standalone {
                            id: id.clone(),
                            tag_type,
                            display: format!("<ph id=\"{}\"/>", id),
                        };
                        if in_source {
                            source_parts.push(part);
                        } else if in_target {
                            target_parts.push(part);
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(format!("XML parse error: {}", e)),
            _ => {}
        }
        buf.clear();
    }

    Ok(XliffFile {
        source_language,
        target_language,
        original,
        segments,
    })
}

/// Context for the inline tag currently being parsed.
enum InlineTagContext {
    Bpt { id: String, ctype: String },
    Ept { id: String, rid: String },
    Ph { id: String, ctype: String },
    It { id: String, ctype: String, pos: String },
}

/// Merge adjacent Text parts to avoid fragmentation.
pub fn coalesce_parts(parts: &[ContentPart]) -> Vec<ContentPart> {
    let mut result: Vec<ContentPart> = Vec::new();
    for part in parts {
        match part {
            ContentPart::Text { text } => {
                if let Some(ContentPart::Text { text: ref mut prev }) = result.last_mut() {
                    prev.push_str(text);
                } else {
                    result.push(part.clone());
                }
            }
            _ => result.push(part.clone()),
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_xliff() {
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
        <target/>
      </trans-unit>
    </body>
  </file>
</xliff>"#;

        let result = parse_xliff(xliff).unwrap();
        assert_eq!(result.source_language, "en-US");
        assert_eq!(result.target_language, "nl-NL");
        assert_eq!(result.segments.len(), 2);
        assert_eq!(result.segments[0].source, "Hello world");
        assert_eq!(result.segments[0].target, "Hallo wereld");
        assert_eq!(result.segments[1].source, "Goodbye");
        assert_eq!(result.segments[1].target, "");
    }

    #[test]
    fn test_parse_xliff_with_bpt_ept() {
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

        let result = parse_xliff(xliff).unwrap();
        assert_eq!(result.segments.len(), 1);
        let seg = &result.segments[0];
        // Plain text should NOT include tag content
        assert_eq!(seg.source, "Click here to continue");
        assert_eq!(seg.target, "Klik hier om verder te gaan");
        // Should have 5 parts: text, tag_open, text, tag_close, text
        assert_eq!(seg.source_parts.len(), 5);
        match &seg.source_parts[1] {
            ContentPart::TagOpen { id, tag_type, .. } => {
                assert_eq!(id, "1");
                assert_eq!(tag_type, "bold");
            }
            _ => panic!("Expected TagOpen"),
        }
        match &seg.source_parts[3] {
            ContentPart::TagClose { id, .. } => {
                assert_eq!(id, "1");
            }
            _ => panic!("Expected TagClose"),
        }
    }

    #[test]
    fn test_ept_rid_uses_bpt_id_for_pairing() {
        // In XLIFF, ept can have its own id but rid references the bpt it closes.
        // e.g. <bpt id="1">, <bpt id="2">, <ept id="3" rid="2">, <ept id="4" rid="1">
        // Should display as {1}, {2}, {/2}, {/1} — NOT {/3}, {/4}
        let xliff = r#"<?xml version="1.0" encoding="UTF-8"?>
<xliff version="1.2" xmlns="urn:oasis:names:tc:xliff:document:1.2">
  <file original="test.docx" source-language="en-US" target-language="nl-NL" datatype="plaintext">
    <body>
      <trans-unit id="1">
        <source><bpt id="1" ctype="bold">{}</bpt><bpt id="2" rid="1">{}</bpt>Definitielijst Personeelshandboek<ept id="3" rid="1">{}</ept><ept id="4" rid="1">{}</ept></source>
        <target/>
      </trans-unit>
    </body>
  </file>
</xliff>"#;

        let result = parse_xliff(xliff).unwrap();
        let seg = &result.segments[0];

        // The ept tags should use rid as their pairing id
        let close_ids: Vec<&str> = seg.source_parts.iter().filter_map(|p| {
            match p {
                ContentPart::TagClose { id, .. } => Some(id.as_str()),
                _ => None,
            }
        }).collect();
        // Both epts have rid="1", so both should show id "1"
        assert_eq!(close_ids, vec!["1", "1"]);
    }

    #[test]
    fn test_parse_xliff_with_ph() {
        let xliff = r#"<?xml version="1.0" encoding="UTF-8"?>
<xliff version="1.2" xmlns="urn:oasis:names:tc:xliff:document:1.2">
  <file original="test.docx" source-language="en-US" target-language="nl-NL" datatype="plaintext">
    <body>
      <trans-unit id="1">
        <source>Line one<ph id="1">&lt;br/&gt;</ph>Line two</source>
        <target>Regel een<ph id="1">&lt;br/&gt;</ph>Regel twee</target>
      </trans-unit>
    </body>
  </file>
</xliff>"#;

        let result = parse_xliff(xliff).unwrap();
        let seg = &result.segments[0];
        assert_eq!(seg.source, "Line oneLine two");
        assert_eq!(seg.source_parts.len(), 3);
        match &seg.source_parts[1] {
            ContentPart::Standalone { id, .. } => {
                assert_eq!(id, "1");
            }
            _ => panic!("Expected Standalone"),
        }
    }

    #[test]
    fn test_parse_xliff_with_g_tags() {
        let xliff = r#"<?xml version="1.0" encoding="UTF-8"?>
<xliff version="1.2" xmlns="urn:oasis:names:tc:xliff:document:1.2">
  <file original="test.docx" source-language="en-US" target-language="nl-NL" datatype="plaintext">
    <body>
      <trans-unit id="1">
        <source>Hello <g id="1">bold text</g> end</source>
        <target>Hallo <g id="1">vette tekst</g> einde</target>
      </trans-unit>
    </body>
  </file>
</xliff>"#;

        let result = parse_xliff(xliff).unwrap();
        let seg = &result.segments[0];
        assert_eq!(seg.source, "Hello bold text end");
        // Parts: "Hello ", TagOpen, "bold text", TagClose, " end"
        assert_eq!(seg.source_parts.len(), 5);
    }
}
