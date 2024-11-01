use provider::DataProvider;
use serde::{Deserialize, Serialize};
use tauri::{ipc::Response, Manager, State};

mod explorer;
mod oracle_provider;
mod provider;

#[derive(Clone, Serialize, Deserialize, Debug)]
struct DataSourceDefinition {
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
    let user_preferences = app
        .path()
        .app_data_dir()?
        .join("user_preferences.json");

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
fn get_nodes(parent_node_key: Option<String>, preferences: State<Preferences>) -> Response {
    let nodes = explorer::get_nodes(parent_node_key, &preferences);
    Response::new(serde_json::to_string(&nodes).unwrap())
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
            }
            _ => {
                println!("Unknown provider: {}", data_source.provider);
            }
        }
    }
    providers
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            match read_user_preferences(&app.handle()) {
                Ok(preferences) => {
                    println!("Preferences: {:?}", preferences);
                    app.manage(preferences);
                }
                Err(e) => {
                    println!("Error reading user preferences: {:?}", e);
                }
            }
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_nodes,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
