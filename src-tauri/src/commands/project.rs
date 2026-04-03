use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectData {
    pub path: String,
    pub name: String,
    pub source_language: String,
    pub target_language: String,
    pub segment_count: usize,
}

#[tauri::command]
pub async fn load_project(path: String) -> Result<ProjectData, String> {
    // TODO: Parse XLIFF/SDLXLIFF/SDLPPX and load segments
    Ok(ProjectData {
        path: path.clone(),
        name: std::path::Path::new(&path)
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default(),
        source_language: String::new(),
        target_language: String::new(),
        segment_count: 0,
    })
}
