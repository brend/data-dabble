use tauri::{ipc::Response, Manager};

mod explorer;
mod oracle_provider;
mod provider;

#[tauri::command]
fn get_nodes(app: tauri::AppHandle, parent_node_key: Option<String>) -> Response {
    let dir = app.app_handle().path().app_data_dir().unwrap();
    println!("dir: {:?}", dir);

    let nodes = explorer::get_nodes(parent_node_key);
    Response::new(serde_json::to_string(&nodes).unwrap())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let dir = app.path().app_data_dir().unwrap();
            println!("dir: {:?}", dir);
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_nodes,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
