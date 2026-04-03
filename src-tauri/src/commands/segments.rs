use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Segment {
    pub id: u64,
    pub segment_number: u32,
    pub source_text: String,
    pub target_text: String,
    pub status: String,
    pub match_percentage: Option<f32>,
    pub match_origin: Option<String>,
}

#[tauri::command]
pub async fn get_segments() -> Result<Vec<Segment>, String> {
    // TODO: Return segments from the currently loaded project
    Ok(vec![])
}

#[tauri::command]
pub async fn save_segment(segment_id: u64, target: String) -> Result<(), String> {
    // TODO: Save segment target text to the project file
    let _ = (segment_id, target);
    Ok(())
}
