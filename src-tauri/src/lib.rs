use tauri::ipc::Response;

mod explorer;

#[tauri::command]
fn get_data_sources(parent_node_key: Option<String>) -> Response {
    let data_sources = explorer::get_data_source_nodes(parent_node_key);
    Response::new(serde_json::to_string(&data_sources).unwrap())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_data_sources,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
