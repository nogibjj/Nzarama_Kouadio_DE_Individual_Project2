// src/database.rs
use rusqlite::{Connection, Result};

pub struct Database {
    conn: Connection,
}

impl Database {
    // Initialize a new Database with an SQLite file
    pub fn new(db_file: &str) -> Result<Self> {
        let conn = Connection::open(db_file)?;
        Ok(Database { conn })
    }

    // Create table with a given name and columns
    pub fn create_table(&self, table_name: &str, columns: Vec<&str>) -> Result<()> {
        let columns_sql = columns
            .into_iter()
            .map(|col| format!("{} TEXT", col))
            .collect::<Vec<String>>()
            .join(", ");
        let sql = format!(
            "CREATE TABLE IF NOT EXISTS {} (id INTEGER PRIMARY KEY, {})",
            table_name, columns_sql
        );
        self.conn.execute(&sql, [])?;
        println!("Table '{}' created!", table_name);
        Ok(())
    }

    // Insert a row into the specified table
    pub fn insert_row(&self, table_name: &str, values: Vec<&str>) -> Result<()> {
        let placeholders = vec!["?"; values.len()].join(", ");
        let sql = format!("INSERT INTO {} VALUES (NULL, {})", table_name, placeholders);
        let params: Vec<&dyn rusqlite::ToSql> =
            values.iter().map(|v| v as &dyn rusqlite::ToSql).collect();
        self.conn.execute(&sql, params.as_slice())?;
        println!("Data inserted into '{}'", table_name);
        Ok(())
    }

    // Query all rows from a table
    pub fn query_table(&self, table_name: &str) -> Result<()> {
        let sql = format!("SELECT * FROM {}", table_name);
        let mut stmt = self.conn.prepare(&sql)?;

        let column_count = stmt.column_count(); // Get the number of columns
        let rows = stmt.query_map([], |row| {
            let mut result = Vec::new();
            for i in 0..column_count {
                let value: String = row.get(i).unwrap_or_default();
                result.push(value);
            }
            Ok(result)
        })?;

        println!("Table '{}' contents:", table_name);
        for row in rows {
            println!("{:?}", row?);
        }

        Ok(())
    }

    // Delete a table
    pub fn delete_table(&self, table_name: &str) -> Result<()> {
        let sql = format!("DROP TABLE IF EXISTS {}", table_name);
        self.conn.execute(&sql, [])?;
        println!("Table '{}' deleted!", table_name);
        Ok(())
    }
}
