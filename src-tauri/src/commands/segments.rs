use super::project::AppState;
use crate::parsers::xliff::ContentPart;
use tauri::State;

#[tauri::command]
pub async fn save_segment(
    segment_id: u64,
    target: String,
    target_parts: Option<Vec<ContentPart>>,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut segments = state.segments.lock().unwrap();
    if let Some(seg) = segments.iter_mut().find(|s| s.id == segment_id) {
        seg.target_text = target;
        if let Some(parts) = target_parts {
            seg.target_parts = parts;
        }
        if seg.status == "new" {
            seg.status = "draft".to_string();
        }
    }
    Ok(())
}
