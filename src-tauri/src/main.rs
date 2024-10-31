// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dotenv::dotenv;

use std::fs;
use std::path::PathBuf;
use tauri::path::PathResolver;

// fn get_user_preferences_path() -> PathBuf {
//     let resolver = PathResolver::new();
//     let mut config_path = resolver
//         .config_dir()
//         .expect("Could not determine config directory");
//     config_path.push("Data Dabble");
//     fs::create_dir_all(&config_path).expect("Could not create app config directory");
//     config_path.push("user_preferences.json");
//     config_path
// }

// #[derive(serde::Serialize, serde::Deserialize)]
// struct Prefs {
//     pub name: String,
//     pub age: u8,
// }

fn main() {
    // let user_preferences_path = get_user_preferences_path();
    // println!("User preferences path: {:?}", user_preferences_path);
    // let prefs = Prefs {
    //     name: "Alice".to_string(),
    //     age: 30,
    // };
    // let prefs_json = serde_json::to_string(&prefs).unwrap();
    // fs::write(user_preferences_path, prefs_json).expect("Could not write user preferences");

    dotenv().ok();
    data_dabble_lib::run()
}
