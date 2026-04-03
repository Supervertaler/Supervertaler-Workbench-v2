use crate::parsers::{xliff, sdlxliff};
use crate::parsers::xliff::ContentPart;
use serde::{Deserialize, Serialize};
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
