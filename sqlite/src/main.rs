// src/main.rs
// src/main.rs
mod database;
use clap::{Parser, Subcommand};
use database::Database;

#[derive(Parser)]
#[command(name = "sqlite_cli")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Create { table_name: String, columns: Vec<String> },
    Insert { table_name: String, values: Vec<String> },
    Query { table_name: String },
    Delete { table_name: String },
}

fn main() -> rusqlite::Result<()> {  // Ensure main returns rusqlite::Result
    let db = Database::new("my_database.db")?;  // Open or create SQLite file
    let cli = Cli::parse();

    match cli.command {
        Commands::Create { table_name, columns } => {
            db.create_table(&table_name, columns.iter().map(AsRef::as_ref).collect())?;
        }
        Commands::Insert { table_name, values } => {
            db.insert_row(&table_name, values.iter().map(AsRef::as_ref).collect())?;
        }
        Commands::Query { table_name } => {
            db.query_table(&table_name)?;
        }
        Commands::Delete { table_name } => {
            db.delete_table(&table_name)?;
        }
    }

    Ok(())
}
