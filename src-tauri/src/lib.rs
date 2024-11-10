use explorer::Node;
use provider::DataError;
use provider::DataProvider;
use serde::{Deserialize, Serialize};
use tauri::{Manager, State};

mod explorer;
mod oracle_provider;
mod mock_provider;
mod provider;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DataSourceDefinition {
    pub name: String,
    pub provider: String,
    pub connection_string: String,
    pub user: String,
    pub password: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Preferences {
    pub version: String,
    pub data_sources: Vec<DataSourceDefinition>,
}

impl Default for Preferences {
    fn default() -> Self {
        Preferences {
            version: "0.1".to_string(),
            data_sources: vec![],
        }
    }
}

#[derive(Debug)]
enum PersistencyError {
    TauriError(tauri::Error),
    IoError(std::io::Error),
    SerdeError(serde_json::Error),
}

impl From<tauri::Error> for PersistencyError {
    fn from(error: tauri::Error) -> Self {
        PersistencyError::TauriError(error)
    }
}

fn read_user_preferences(app: &tauri::AppHandle) -> Result<Preferences, PersistencyError> {
    let user_preferences = app.path().app_data_dir()?.join("user_preferences.json");

    print!("Looking for user preferences file: {:?}", user_preferences);

    if user_preferences.exists() {
        let user_preferences_file = std::fs::read_to_string(user_preferences).unwrap();
        let user_preferences = serde_json::from_str::<Preferences>(&user_preferences_file)
            .map_err(|e| PersistencyError::SerdeError(e))?;
        Ok(user_preferences)
    } else {
        Err(PersistencyError::IoError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "User preferences file not found",
        )))
    }
}

#[tauri::command]
fn get_nodes(
    parent_node_key: Option<String>,
    preferences: State<Preferences>,
) -> Result<Vec<Node>, DataError> {
    let nodes = explorer::get_nodes(parent_node_key, &preferences)?;
    Ok(nodes)
}

#[derive(Serialize, Deserialize)]
pub struct Product {
    pub code: String,
    pub name: String,
    pub category: String,
    pub quantity: i64,
}

#[tauri::command]
fn execute_query(
    sql_query: String,
    preferences: State<Preferences>,
) -> Result<Vec<Product>, DataError> {
    Ok(vec![Product {
        code: "P01".to_string(),
        name: "Red Apple".to_string(),
        category: "Fruit".to_string(),
        quantity: 10,
    },
    Product {
        code: "P02".to_string(),
        name: "Green Cheery".to_string(),
        category: "Fruit".to_string(),
        quantity: 20,
    }])
}

pub fn get_data_providers(preferences: &Preferences) -> Vec<Box<dyn DataProvider>> {
    let mut providers: Vec<Box<dyn DataProvider>> = vec![];
    for data_source in &preferences.data_sources {
        match data_source.provider.as_str() {
            "oracle" => {
                providers.push(Box::new(oracle_provider::OracleProvider::new(
                    &data_source.connection_string,
                    &data_source.user,
                    &data_source.password,
                )));
            },
            "mock" => {
                providers.push(Box::new(mock_provider::MockProvider));
            },
            _ => {
                println!("Unknown provider: {}", data_source.provider);
            }
        }
    }
    providers
}

fn log_persistency_error(e: &PersistencyError) {
    match e {
        PersistencyError::TauriError(e) => {
            println!("Tauri error: {:?}", e);
        }
        PersistencyError::IoError(e) => {
            println!("IO error: {:?}", e);
        }
        PersistencyError::SerdeError(e) => {
            println!("Serde error: {:?}", e);
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let user_preferences: Preferences;

            match read_user_preferences(&app.handle()) {
                Ok(preferences) => {
                    println!("Preferences: {:?}", preferences);
                    user_preferences = preferences;
                }
                Err(e) => {
                    log_persistency_error(&e);
                    user_preferences = Preferences::default();
                }
            }

            app.manage(user_preferences);
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_nodes,execute_query])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
