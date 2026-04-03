use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TmMatch {
    pub id: u64,
    pub source_text: String,
    pub target_text: String,
    pub match_percentage: f32,
    pub origin: String,
}

#[tauri::command]
pub async fn get_tm_matches(source: String, threshold: f32) -> Result<Vec<TmMatch>, String> {
    // TODO: Query SQLite TM with fuzzy matching (Levenshtein via strsim)
    let _ = (source, threshold);
    Ok(vec![])
}
