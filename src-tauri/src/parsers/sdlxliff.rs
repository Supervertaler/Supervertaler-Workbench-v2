use crate::parsers::xliff::{ContentPart, XliffFile, XliffSegment, get_attr, guess_tag_type, coalesce_parts};
use quick_xml::events::Event;
use quick_xml::reader::Reader;

/// Parse an SDLXLIFF file (Trados Studio format).
/// SDLXLIFF is XLIFF 1.2 with SDL-specific namespaces and extensions.
/// Key differences from plain XLIFF:
/// - Uses <seg-source> alongside <source>
/// - Has <sdl:seg-defs> for confirmation status
/// - Trans-unit IDs are UUIDs
/// - Contains <g> tags for inline formatting
/// - Contains <x/> tags for standalone placeholders
pub fn parse_sdlxliff(content: &str) -> Result<XliffFile, String> {
    let mut reader = Reader::from_str(content);
    reader.config_mut().trim_text(false);

    let mut source_language = String::new();
    let mut target_language = String::new();
    let mut original = String::new();
    let mut segments: Vec<XliffSegment> = Vec::new();

    let mut current_id = String::new();
    let mut in_trans_unit = false;
    let mut in_seg_source = false;
    let mut in_source = false;
    let mut in_target = false;
    let mut has_seg_source = false;

    let mut source_text = String::new();
    let mut target_text = String::new();
    let mut source_parts: Vec<ContentPart> = Vec::new();
    let mut target_parts: Vec<ContentPart> = Vec::new();

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
                        source_text.clear();
                        target_text.clear();
                        source_parts.clear();
                        target_parts.clear();
                        has_seg_source = false;
                        for attr in e.attributes().flatten() {
                            if attr.key.local_name().as_ref() == b"id" {
                                current_id = attr
                                    .unescape_value()
                                    .unwrap_or_default()
                                    .to_string();
                            }
                        }
                    }
                    b"seg-source" if in_trans_unit => {
                        in_seg_source = true;
                        has_seg_source = true;
                        source_text.clear();
                        source_parts.clear();
                    }
                    b"source" if in_trans_unit && !in_seg_source && !in_target => {
                        if !has_seg_source {
                            in_source = true;
                        }
                    }
                    b"target" if in_trans_unit && !in_seg_source && !in_source => {
                        in_target = true;
                    }
                    // <g> tags in SDLXLIFF wrap inline formatting
                    b"g" if in_seg_source || in_source || in_target => {
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
                        if in_seg_source || in_source {
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
                            let source = source_text.trim().to_string();
                            if !source.is_empty() {
                                segments.push(XliffSegment {
                                    id: current_id.clone(),
                                    source,
                                    target: target_text.trim().to_string(),
                                    source_parts: coalesce_parts(&source_parts),
                                    target_parts: coalesce_parts(&target_parts),
                                });
                            }
                            in_trans_unit = false;
                        }
                    }
                    b"seg-source" => {
                        in_seg_source = false;
                    }
                    b"source" => {
                        in_source = false;
                    }
                    b"target" => {
                        in_target = false;
                    }
                    b"g" if in_seg_source || in_source || in_target => {
                        let part = ContentPart::TagClose {
                            id: String::new(),
                            tag_type: "formatting".to_string(),
                            display: "</g>".to_string(),
                        };
                        if in_seg_source || in_source {
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
                if in_seg_source || in_source {
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
                    b"target" if in_trans_unit && !in_seg_source => {
                        target_text.clear();
                        target_parts.clear();
                    }
                    b"x" if in_seg_source || in_source || in_target => {
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
                        if in_seg_source || in_source {
                            source_parts.push(part);
                        } else if in_target {
                            target_parts.push(part);
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(format!("SDLXLIFF parse error: {}", e)),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sdlxliff_with_seg_source() {
        let sdlxliff = r#"<?xml version="1.0" encoding="UTF-8"?>
<xliff version="1.2" xmlns="urn:oasis:names:tc:xliff:document:1.2"
       xmlns:sdl="http://sdl.com/FileTypes/SdlXliff/1.0">
  <file original="test.docx" source-language="en-US" datatype="x-sdlfilterframework2">
    <body>
      <trans-unit id="abc-123">
        <source>Hello world</source>
        <seg-source>Hello world</seg-source>
        <target>Hallo wereld</target>
      </trans-unit>
    </body>
  </file>
</xliff>"#;

        let result = parse_sdlxliff(sdlxliff).unwrap();
        assert_eq!(result.source_language, "en-US");
        assert_eq!(result.segments.len(), 1);
        assert_eq!(result.segments[0].source, "Hello world");
        assert_eq!(result.segments[0].target, "Hallo wereld");
    }

    #[test]
    fn test_parse_sdlxliff_with_g_tags() {
        let sdlxliff = r#"<?xml version="1.0" encoding="UTF-8"?>
<xliff version="1.2" xmlns="urn:oasis:names:tc:xliff:document:1.2">
  <file original="test.docx" source-language="en-US" target-language="nl-NL" datatype="x-sdlfilterframework2">
    <body>
      <trans-unit id="1">
        <source>Hello <g id="1">bold</g> world</source>
        <seg-source>Hello <g id="1">bold</g> world</seg-source>
        <target>Hallo <g id="1">vet</g> wereld</target>
      </trans-unit>
    </body>
  </file>
</xliff>"#;

        let result = parse_sdlxliff(sdlxliff).unwrap();
        let seg = &result.segments[0];
        assert_eq!(seg.source, "Hello bold world");
        assert_eq!(seg.target, "Hallo vet wereld");
        // source_parts: "Hello ", TagOpen, "bold", TagClose, " world"
        assert_eq!(seg.source_parts.len(), 5);
        assert_eq!(seg.target_parts.len(), 5);
    }

    #[test]
    fn test_parse_sdlxliff_with_x_tags() {
        let sdlxliff = r#"<?xml version="1.0" encoding="UTF-8"?>
<xliff version="1.2" xmlns="urn:oasis:names:tc:xliff:document:1.2">
  <file original="test.docx" source-language="en-US" target-language="nl-NL" datatype="x-sdlfilterframework2">
    <body>
      <trans-unit id="1">
        <source>Before<x id="1"/>After</source>
        <seg-source>Before<x id="1"/>After</seg-source>
        <target>Voor<x id="1"/>Na</target>
      </trans-unit>
    </body>
  </file>
</xliff>"#;

        let result = parse_sdlxliff(sdlxliff).unwrap();
        let seg = &result.segments[0];
        assert_eq!(seg.source, "BeforeAfter");
        assert_eq!(seg.source_parts.len(), 3);
        match &seg.source_parts[1] {
            ContentPart::Standalone { id, .. } => assert_eq!(id, "1"),
            _ => panic!("Expected Standalone"),
        }
    }
}
