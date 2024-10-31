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

pub fn get_data_source_nodes(parent_node_key: Option<String>) -> Vec<ExplorerNode> {
    if let Some(parent_node_key) = parent_node_key {
        match parent_node_key.as_str() {
            "1" => vec![
                ExplorerNode {
                    key: "1.1".to_string(),
                    label: "Gigantos - Sales".to_string(),
                    node_type: NodeType::DataSource,
                    leaf: false,
                },
                ExplorerNode {
                    key: "1.2".to_string(),
                    label: "Gigantos - HR".to_string(),
                    node_type: NodeType::DataSource,
                    leaf: false,
                },
            ],
            _ => vec![],
        }
    } else {
        vec![
            ExplorerNode {
                key: "1".to_string(),
                label: "Gigantos (MS SQL)".to_string(),
                node_type: NodeType::DataSource,
                leaf: false,
            },
            ExplorerNode {
                key: "2".to_string(),
                label: "Persephone (Oracle DB)".to_string(),
                node_type: NodeType::DataSource,
                leaf: false,
            },
        ]
    }
}
