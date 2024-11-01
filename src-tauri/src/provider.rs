use crate::explorer::Node;

pub trait DataProvider {
    fn id(&self) -> String;
    fn get_nodes(&self, parent_node_key: String) -> Vec<Node>;
    fn owns_node(&self, node_key: &str) -> bool;
}