use tauri::ipc::Response;

mod explorer;
mod oracle_provider;
mod provider;

#[tauri::command]
fn get_nodes(parent_node_key: Option<String>) -> Response {
    let nodes = explorer::get_nodes(parent_node_key);
    Response::new(serde_json::to_string(&nodes).unwrap())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_nodes,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
