use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TermMatch {
    pub id: u64,
    pub source_term: String,
    pub target_term: String,
    pub priority: u8,
    pub forbidden: bool,
    pub notes: Option<String>,
}

#[tauri::command]
pub async fn lookup_terms(source_text: String) -> Result<Vec<TermMatch>, String> {
    // TODO: Query termbase SQLite for matching terms in the source text
    let _ = source_text;
    Ok(vec![])
}
