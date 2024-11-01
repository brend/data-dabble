use serde::{Deserialize, Serialize};

use crate::explorer::Node;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataError {
    DataProviderError(String),
    DataProviderNotFound(String),
}

pub trait DataProvider {
    fn id(&self) -> String;
    fn get_nodes(&self, parent_node_key: String) -> Result<Vec<Node>, DataError>;
    fn owns_node(&self, node_key: &str) -> bool;
}