#![cfg_attr(
    all(not(debug_assertions), target_os = "linux"),
    windows_subsystem = "gtk"
)]

fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
