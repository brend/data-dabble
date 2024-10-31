use crate::explorer::Node;
use crate::oracle_provider::OracleProvider;

pub trait DataProvider {
    fn id(&self) -> String;
    fn get_nodes(&self, parent_node_key: String) -> Vec<Node>;
    fn owns_node(&self, node_key: &str) -> bool;
}

pub fn get_data_providers() -> Vec<Box<dyn DataProvider>> {
    vec![Box::new(OracleProvider::new(
        "GPT_PROD@Persephone",
        "GPT_PROD",
    ))]
}
