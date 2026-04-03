use serde_json::Value;

#[tauri::command]
pub async fn get_settings() -> Result<Value, String> {
    // TODO: Read settings.json from app data directory
    Ok(serde_json::json!({}))
}

#[tauri::command]
pub async fn save_settings(settings: Value) -> Result<(), String> {
    // TODO: Write settings.json to app data directory
    let _ = settings;
    Ok(())
}
