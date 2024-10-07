use serde::Serialize;
use tauri::ipc::Response;

struct Column {
    name: String,
    data_type: String,
}

#[tauri::command]
fn query_columns() -> Response {
    let columns = vec![
        Column {
            name: "code".to_string(),
            data_type: "string".to_string(),
        },
        Column {
            name: "name".to_string(),
            data_type: "string".to_string(),
        },
        Column {
            name: "category".to_string(),
            data_type: "string".to_string(),
        },
        Column {
            name: "quantity".to_string(),
            data_type: "number".to_string(),
        },
    ];

    Response::new(serde_json::to_string(&columns).unwrap())
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", if name.len() > 0 { name } else { "stranger" })
}

#[derive(Serialize)]
struct Product {
    code: String,
    name: String,
    category: String,
    quantity: i32,
}

impl Product {
    fn new(code: String, name: String, category: String, quantity: i32) -> Self {
        Self {
            code,
            name,
            category,
            quantity,
        }
    }
}

#[tauri::command]
fn query_products() -> Response {
    let products = vec![
        Product::new("P001".to_string(), "Product 1".to_string(), "Category 1".to_string(), 10),
        Product::new("P002".to_string(), "Product 2".to_string(), "Category 1".to_string(), 20),
        Product::new("P003".to_string(), "Product 3".to_string(), "Category 2".to_string(), 30),
        Product::new("P004".to_string(), "Product 4".to_string(), "Category 2".to_string(), 40),
    ];

    Response::new(serde_json::to_string(&products).unwrap())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![query_products])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
