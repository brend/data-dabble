use crate::explorer::Node;
use crate::provider::{DataError, DataProvider};
use oracle::Connection;

const PATH_SEPARATOR: &str = ".";
const NODE_TYPE_TABLE: &str = "table";
const NODE_TYPE_COLUMN: &str = "column";

pub struct OracleProvider {
    id: String,
    tns_name: String,
    password: String,
}

impl OracleProvider {
    pub fn new(id: &str, connection_string: &str, _user: &str, password: &str) -> Self {
        let tns_name = connection_string.to_string();
        let password = password.to_string();
        OracleProvider {
            id: id.to_string(),
            tns_name,
            password,
        }
    }

    fn open_connection(&self) -> Result<Connection, DataError> {
        match Connection::connect(&self.tns_name, &self.password, &self.tns_name) {
            Ok(connection) => Ok(connection),
            Err(error) => Err(DataError::DataProviderError(error.to_string())),
        }
    }

    fn get_tables(&self) -> Result<Vec<Node>, DataError> {
        println!("get_tables");
        let mut nodes = vec![];
        let connection = self.open_connection()?;
        let sql = "SELECT table_name FROM user_tables ORDER BY table_name";
        let rows = connection.query(sql, &[])?;
        for row in rows {
            let table_name: String = row?.get(0)?;
            let node = Node::new(
                format!("{}.{}", self.id(), table_name).as_str(),
                &table_name,
                NODE_TYPE_TABLE,
                false,
            );
            nodes.push(node);
        }
        Ok(nodes)
    }

    fn get_columns(&self, table_name: &str) -> Result<Vec<Node>, DataError> {
        println!("get_columns: {}", table_name);
        let mut nodes = vec![];
        let connection = self.open_connection()?;
        let sql = format!(
            "SELECT column_name FROM user_tab_columns WHERE table_name = '{}' ORDER BY column_name",
            table_name
        );
        let rows = connection.query(&sql, &[])?;
        for row in rows {
            let column_name: String = row?.get(0)?;
            let node = Node::new(&column_name, &column_name, NODE_TYPE_COLUMN, true);
            nodes.push(node);
        }
        Ok(nodes)
    }
}

impl DataProvider for OracleProvider {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn name(&self) -> String {
        self.tns_name.to_string()
    }

    fn owns_node(&self, node_key: &str) -> bool {
        node_key.starts_with(&self.id)
    }

    fn get_nodes(&self, parent_node_key: String) -> Result<Vec<Node>, DataError> {
        println!("get_nodes: {}", parent_node_key);
        let path = parent_node_key.split(PATH_SEPARATOR).collect::<Vec<&str>>();

        if path.len() == 1 {
            self.get_tables()
        } else if path.len() == 2 {
            self.get_columns(path[1])
        } else {
            Ok(vec![])
        }
    }

    fn execute_query(&self, sql_query: &str) -> Result<Vec<Vec<String>>, DataError> {
        let connection = self.open_connection()?;
        let rows = connection.query(sql_query, &[])?;
        let mut results = vec![];
    
        for row_result in rows {
            let row = row_result?;
            let mut result = vec![];
    
            // Iterate through each column in the row
            for i in 0..row.column_info().len() {
                // Retrieve the value as a string
                let value: String = match row.get(i) {
                    Ok(Some(val)) => val,
                    Ok(None) => "NULL".to_string(),
                    Err(_) => "ERROR".to_string(),
                };
                result.push(value);
            }
            results.push(result);
        }
        Ok(results)
    }
}

impl From<oracle::Error> for DataError {
    fn from(error: oracle::Error) -> Self {
        DataError::DataProviderError(error.to_string())
    }
}
