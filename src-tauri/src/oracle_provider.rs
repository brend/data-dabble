use std::env;

use crate::explorer::Node;
use crate::provider::DataProvider;
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

    pub fn new(connection_string: &str, user: &str, password: &str) -> Self {
        let tns_name = connection_string.to_string();
        let id = format!("oracle-{}", tns_name).to_string();
        let password = password.to_string();
        OracleProvider { id, tns_name, password }
    }

    fn open_connection(&self) -> Connection {
        Connection::connect(
            &self.tns_name,
            env::var("DB_PASSWORD").unwrap(),
            &self.tns_name,
        )
        .unwrap()
    }

    fn get_tables(&self) -> Vec<Node> {
        println!("get_tables");
        let mut nodes = vec![];
        let connection = self.open_connection();
        let sql = "SELECT table_name FROM user_tables ORDER BY table_name";
        let rows = connection.query(sql, &[]).unwrap();
        for row in rows {
            let table_name: String = row.unwrap().get(0).unwrap();
            let node = Node::new(
                format!("{}.{}", self.id(), table_name).as_str(),
                &table_name,
                NODE_TYPE_TABLE,
                false,
            );
            nodes.push(node);
        }
        nodes
    }

    fn get_columns(&self, table_name: &str) -> Vec<Node> {
        println!("get_columns: {}", table_name);
        let mut nodes = vec![];
        let connection = self.open_connection();
        let sql = format!(
            "SELECT column_name FROM user_tab_columns WHERE table_name = '{}' ORDER BY column_name",
            table_name
        );
        let rows = connection.query(&sql, &[]).unwrap();
        for row in rows {
            let column_name: String = row.unwrap().get(0).unwrap();
            let node = Node::new(&column_name, &column_name, NODE_TYPE_COLUMN, true);
            nodes.push(node);
        }
        nodes
    }
}

impl DataProvider for OracleProvider {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn owns_node(&self, node_key: &str) -> bool {
        node_key.starts_with(&self.id)
    }

    fn get_nodes(&self, parent_node_key: String) -> Vec<Node> {
        println!("get_nodes: {}", parent_node_key);
        let path = parent_node_key.split(PATH_SEPARATOR).collect::<Vec<&str>>();

        if path.len() == 1 {
            self.get_tables()
        } else if path.len() == 2 {
            self.get_columns(path[1])
        } else {
            vec![]
        }
    }
}
