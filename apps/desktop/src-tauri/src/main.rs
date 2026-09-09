#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;

#[tauri::command]
fn process_iris_directive(query: String) -> Result<String, String> {
    let lower_query = query.to_lower();
    
    // HARD GUARDRAIL: Strict deletion/modification filter string blacklist
    let destructive_tokens = ["delete", "remove", "destroy", "uninstall", "format", "recycle", "del", "rmdir"];
    if destructive_tokens.iter().any(|&token| lower_query.contains(token)) {
        return Err("Access Denied: Destructive protocols blacklisted.".to_string());
    }

    // SAFE COMPLIANT AUTOMATIONS
    if lower_query.contains("youtube") {
        let _ = Command::new("cmd").args(["/C", "start https://youtube.com"]).spawn();
        return Ok("Launching YouTube".to_string());
    } else if lower_query.contains("google") {
        let _ = Command::new("cmd").args(["/C", "start https://google.com"]).spawn();
        return Ok("Navigating to Google".to_string());
    } else if lower_query.contains("notepad") {
        let _ = Command::new("cmd").args(["/C", "notepad.exe"]).spawn();
        return Ok("Opening Scratchpad".to_string());
    }

    Err("Command recognized but no safe shortcut route matched.".to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![process_iris_directive])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
