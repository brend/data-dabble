use std::collections::HashMap;

use serde::Serialize;
use tauri::ipc::Response;
use oracle::Connection;

fn open_connection() -> oracle::Result<Connection> {
    let password = std::env::var("DB_PASSWORD").expect("DB_PASSWORD environment variable not set");
    Ok(Connection::connect("GPT_PROD", &password, "GPT_PROD")?)
}

/// This is the struct that represents column information and
/// will be serialized to JSON
#[derive(Serialize)]
struct Column {
    /// Name of the column in the database
    field: String,
    /// Caption of the column to be displayed in the UI
    header: String,
    /// Data type of the column
    data_type: String,
}

#[tauri::command]
fn query_columns(table_name: String) -> Response {
    let conn = open_connection().unwrap();
    let sql = format!(
        "SELECT column_name AS field, column_name AS header, data_type
        FROM all_tab_columns
        WHERE table_name = '{}'
        AND owner = '{}'
        ORDER BY column_id",
        table_name,
        "GPT_PROD"
    );
    let columns = conn
        .query(&sql, &[])
        .unwrap()
        .map(|row| {
            let row = row.unwrap();
            Column {
                field: row.get("field").unwrap(),
                header: row.get("header").unwrap(),
                data_type: row.get("data_type").unwrap(),
            }
        })
        .collect::<Vec<_>>();
    
    Response::new(serde_json::to_string(&columns).unwrap())
}

#[tauri::command]
fn query_rows(table_name: String) -> Response {
    let rows = open_connection()
        .unwrap()
        .query(&format!("SELECT * FROM {} WHERE ROWNUM <= 100", table_name), &[])
        .unwrap()
        .map(|row| {
            let row = row.unwrap();
            let mut map: HashMap<String, String> = HashMap::new();
            for column in row.column_info() {
                map.insert(column.name().to_string(), row.get(column.name()).unwrap_or(String::new()));
            }
            map
        })
        .collect::<Vec<_>>();

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
