use std::collections::HashMap;

// return the list of socials and their enabled status
// eg { "facebook": true, "twitter": false, "instagram": true }
#[tauri::command]
fn get_enabled_socials() -> Result<serde_json::Value, String> {
    let mut socials = HashMap::new();
    socials.insert("twitter", true);
    socials.insert("discord", false);
    socials.insert("threads", true);
    Ok(serde_json::to_value(socials).unwrap())
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_enabled_socials])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
