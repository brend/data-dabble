use crate::provider::{DataError, DataProvider};

pub struct MockProvider;

impl DataProvider for MockProvider {
    fn id(&self) -> String {
        "mock".to_string()
    }

    fn name(&self) -> String {
        "Mock DB".to_string()
    }

    fn get_nodes(&self, _parent_node_key: String) -> Result<Vec<crate::explorer::Node>, DataError> {
        Ok(vec![
            crate::explorer::Node::new("mock.1", "Mock Node 1", "table", false),
            crate::explorer::Node::new("mock.2", "Mock Node 2 has a rather long name don't you think", "table", false),
            crate::explorer::Node::new("mock.3", "Mock Node 3", "table", false),
        ])
    }

    fn owns_node(&self, node_key: &str) -> bool {
        node_key.starts_with("mock")
    }

    fn execute_query(&self, _sql_query: &str) -> Result<Vec<Vec<String>>, DataError> {
        Ok(vec![
            vec!["P01".to_string(), "Red Apple".to_string(), "Fruit".to_string(), "10".to_string()],
            vec!["P02".to_string(), "Green Cheery".to_string(), "Fruit".to_string(), "20".to_string()],
        ])
    }
}