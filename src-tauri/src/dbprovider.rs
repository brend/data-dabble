use serde::Serialize;

#[derive(Serialize)]
pub struct Column {
    pub field: String,
    pub data_type: String,
}

#[derive(Serialize)]
pub struct Row {
    pub values: Vec<String>,
}

impl Row {
    pub fn get(&self, index: usize) -> Result<String, String> {
        self.values.get(index).map(|s| s.clone()).ok_or("Index out of bounds".to_string())
    }
}

#[derive(Serialize)]
pub struct ResultSet {
    pub columns: Vec<Column>,
    pub rows: Vec<Row>,
}

impl Iterator for ResultSet {
    type Item = Row;

    fn next(&mut self) -> Option<Self::Item> {
        self.rows.pop()
    }
}

pub trait Connection {
    fn query(&self, sql: &str) -> Result<ResultSet, String>;
}

pub trait DatabaseProvider {
    fn connect(&self) -> Result<Box<dyn Connection>, String>;
}
