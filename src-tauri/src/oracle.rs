use crate::dbprovider::{Column, Row, DatabaseProvider, Connection as Conn, ResultSet};
use oracle::Connection;

pub struct OracleProvider {
    pub username: String,
    pub password: String,
    pub connect_string: String,
}

struct OracleConnection {
    conn: Connection,
}

impl DatabaseProvider for OracleProvider {
    fn connect(&self) -> Result<Box<dyn Conn>, String> {
        println!("Connecting to Oracle with username: {}, password: {}, connect_string: {}", self.username, self.password, self.connect_string);
        let connection = Connection::connect(&self.username, &self.password, &self.connect_string)
            .map_err(|e| e.to_string());

        match connection {
            Ok(connection) => Ok(Box::new(OracleConnection { conn: connection })),
            Err(e) => Err(e.to_string())
        }
    }

    fn query_tables(&self) -> Result<Vec<String>, String> {
        let conn = self.connect()?;
        let sql = "SELECT table_name FROM all_tables WHERE owner = 'GPT_PROD' ORDER BY table_name";
        let result_set = conn.query(sql)?;
        let tables = result_set.rows.iter().map(|row| row.values[0].clone()).collect();
        Ok(tables)
    }
}

impl Conn for OracleConnection {
    fn query(&self, query: &str) -> Result<ResultSet, String> {
        let mut statement = self.conn.statement(query).build().map_err(|e| e.to_string())?;
        let res = statement.query(&[]).map_err(|e| e.to_string())?;
        let mut columns = vec![];

        for info in res.column_info() {
            columns.push(Column {
                field: info.name().to_string(),
                data_type: info.oracle_type().to_string(),
            });
        }

        let column_count = columns.len();
        let mut rows = vec![];

        for row in res {
            let row = row.map_err(|e| e.to_string())?;
            let mut values = vec![];
            for i in 0..column_count {
                let value = row.get(i).map_err(|e| e.to_string())?;
                values.push(value);
            }
            rows.push(Row {values });
        }
        Ok(ResultSet { columns, rows })
    }
}
