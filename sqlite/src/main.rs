// src/main.rs

// Import the database module and external dependencies
mod database;
use clap::{Parser, Subcommand}; // clap for command-line argument parsing
use database::Database; // Use Database struct from database module

// Define the main CLI structure
#[derive(Parser)]
#[command(name = "sqlite_cli")]
struct Cli {
    #[command(subcommand)]
    command: Commands, // Parse the command as one of the defined subcommands
}

// Define CLI commands for CRUD operations
#[derive(Subcommand)]
enum Commands {
    // Command to create a new table with specified columns
    Create {
        table_name: String,
        columns: Vec<String>,
    },
    // Command to insert values into a specified table
    Insert {
        table_name: String,
        values: Vec<String>,
    },
    // Command to query and display contents of a specified table
    Query {
        table_name: String,
    },
    // Command to delete a specified table
    Delete {
        table_name: String,
    },
}

// Main function to handle CLI and execute database operations
fn main() -> rusqlite::Result<()> {
    let db = Database::new("my_database.db")?; // Initialize or create SQLite database file
    let cli = Cli::parse(); // Parse CLI arguments

    // Match and execute the specified command
    match cli.command {
        Commands::Create {
            table_name,
            columns,
        } => {
            // Create a new table with the provided name and columns
            db.create_table(&table_name, columns.iter().map(AsRef::as_ref).collect())?;
        }
        Commands::Insert { table_name, values } => {
            // Insert a new row with values into the specified table
            db.insert_row(&table_name, values.iter().map(AsRef::as_ref).collect())?;
        }
        Commands::Query { table_name } => {
            // Query and display all rows from the specified table
            db.query_table(&table_name)?;
        }
        Commands::Delete { table_name } => {
            // Delete the specified table from the database
            db.delete_table(&table_name)?;
        }
    }

    Ok(()) // Return success if all operations completed without error
}
