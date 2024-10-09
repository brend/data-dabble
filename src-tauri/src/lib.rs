use tauri::ipc::Response;

mod dbprovider;
mod oracle;

use crate::dbprovider::{DatabaseProvider, Connection, Column};
use crate::oracle::OracleProvider;

fn open_connection() -> Result<Box<dyn Connection>, String> {
    let provider = OracleProvider {
        username: "GPT_PROD".to_string(),
        password: std::env::var("DB_PASSWORD").unwrap(),
        connect_string: "GPT_PROD".to_string(),
    };
    provider.connect()
}

#[tauri::command]
fn query_columns(table_name: String) -> Response {
    let conn = open_connection().unwrap();
    let sql = format!(
        "SELECT column_name AS field, data_type
        FROM all_tab_columns
        WHERE table_name = '{}'
        AND owner = '{}'
        ORDER BY column_id",
        table_name,
        "GPT_PROD"
    );
    let columns = conn
        .query(&sql)
        .unwrap()
        .map(|row| {
            Column {
                field: row.get(0).unwrap(),
                data_type: row.get(1).unwrap(),
            }
        })
        .collect::<Vec<_>>();
    
    Response::new(serde_json::to_string(&columns).unwrap())
}

#[tauri::command]
fn query_rows(table_name: String) -> Response {
    let result_set = open_connection()
        .unwrap()
        .query(&format!("SELECT * FROM {} WHERE ROWNUM <= 100", table_name))
        .unwrap();
    Response::new(serde_json::to_string(&result_set).unwrap())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![query_columns, query_rows])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
