use serde::Serialize;
use crate::{get_data_providers, Preferences};

const NODE_TYPE_DATASOURCE: &str = "datasource";

#[derive(Serialize)]
pub struct Node {
    pub key: String,
    pub label: String,
    #[serde(rename = "type")]
    pub node_type: String,
    pub leaf: bool,
}

impl Node {
    pub fn new(key: &str, label: &str, node_type: &str, leaf: bool) -> Self {
        Self {
            key: key.to_string(),
            label: label.to_string(),
            node_type: node_type.to_string(),
            leaf,
        }
    }
}

pub fn get_nodes(parent_node_key: Option<String>, preferences: &Preferences) -> Vec<Node> {
    let providers =  get_data_providers(preferences);

    if let Some(parent_node_key) = parent_node_key {    
        for provider in providers {
            if provider.owns_node(&parent_node_key) {
                return provider.get_nodes(parent_node_key);
            }
        }

        vec![]
    } else {
        let mut nodes = vec![];
        for provider in providers {
            nodes.push(Node::new(
                &provider.id(),
                &provider.id(),
                NODE_TYPE_DATASOURCE,
                false,
            ));
        }
        nodes
    }
}
