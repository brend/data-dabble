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
pub struct Node {
    pub key: String,
    pub label: String,
    #[serde(rename = "type")]
    pub node_type: NodeType,
    pub leaf: bool,
}

impl Node {
    pub fn new(key: &str, label: &str, node_type: NodeType, leaf: bool) -> Self {
        Self {
            key: key.to_string(),
            label: label.to_string(),
            node_type,
            leaf,
        }
    }
}

pub fn get_data_source_nodes(parent_node_key: Option<String>) -> Vec<Node> {
    if let Some(parent_node_key) = parent_node_key {
        match parent_node_key.as_str() {
            "1" => vec![
                Node::new("1.1", "Gigantos - Sales", NodeType::DataSource, false),
                Node::new("1.2", "Gigantos - HR", NodeType::DataSource, false),
            ],
            _ => vec![],
        }
    } else {
        vec![
            Node::new("1", "Gigantos (MS SQL)", NodeType::DataSource, false),
            Node::new("2", "Persephone (Oracle DB)", NodeType::DataSource, false),
        ]
    }
}
