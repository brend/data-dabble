use serde::Serialize;

#[derive(Serialize)]
pub enum NodeType {
    DataSource,
    // Database,
    // Schema,
    // Table,
    // Column,
}

#[derive(Serialize)]
pub struct ExplorerNode {
    pub key: String,
    pub label: String,
    #[serde(rename = "type")]
    pub node_type: NodeType,
    pub leaf: bool,
}

impl ExplorerNode {
    pub fn new(key: &str, label: &str, node_type: NodeType, leaf: bool) -> Self {
        Self {
            key: key.to_string(),
            label: label.to_string(),
            node_type,
            leaf,
        }
    }
}

pub fn get_data_source_nodes(parent_node_key: Option<String>) -> Vec<ExplorerNode> {
    if let Some(parent_node_key) = parent_node_key {
        match parent_node_key.as_str() {
            "1" => vec![
                ExplorerNode.new(
                    "1.1".to_string(),
                    "Gigantos - Sales".to_string(),
                    NodeType::DataSource,
                    false,
                ),
                ExplorerNode.new(
                    "1.2".to_string(),
                    "Gigantos - HR".to_string(),
                    NodeType::DataSource,
                    false,
                ),
            ],
            _ => vec![],
        }
    } else {
        vec![
            ExplorerNode.new(
                "1".to_string(),
                "Gigantos (MS SQL)".to_string(),
                NodeType::DataSource,
                false,
            ),
            ExplorerNode.new(
                "2".to_string(),
                "Persephone (Oracle DB)".to_string(),
                NodeType::DataSource,
                false,
            ),
        ]
    }
}
