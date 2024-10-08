use std::collections::HashMap;

use serde::Serialize;
use tauri::ipc::Response;

#[derive(Serialize)]
struct Column {
    field: String,
    header: String,
    data_type: String,
}

#[tauri::command]
fn query_columns(table_name: String) -> Response {
    let columns = vec![
        Column {
            field: "code".to_string(),
            header: "Code".to_string(),
            data_type: "string".to_string(),
        },
        Column {
            field: "name".to_string(),
            header: "Name".to_string(),
            data_type: "string".to_string(),
        },
        Column {
            field: "category".to_string(),
            header: "Category".to_string(),
            data_type: "string".to_string(),
        },
        Column {
            field: "quantity".to_string(),
            header: "Quantity".to_string(),
            data_type: "number".to_string(),
        },
    ];

    Response::new(serde_json::to_string(&columns).unwrap())
}

#[tauri::command]
fn query_rows(table_name: String) -> Response {
    let rows = vec![
        HashMap::from([
            ("code".to_string(), "P001".to_string()),
            ("name".to_string(), "Product 1".to_string()),
            ("category".to_string(), "Category 1".to_string()),
            ("quantity".to_string(), 100.to_string()),
        ]),
    ];

    Response::new(serde_json::to_string(&rows).unwrap())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![query_columns, query_rows])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
