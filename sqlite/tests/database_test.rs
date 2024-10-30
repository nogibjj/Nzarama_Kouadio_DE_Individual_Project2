// tests/database_test.rs

// Import the rusqlite Result type to handle results and errors
use rusqlite::Result;
// Import the Database struct from the main project file to test its functions
use sqlite::database::Database; // Use crate name `sqlite`

// Function to set up an in-memory database for testing
fn setup_test_db() -> Result<Database> {
    Database::new(":memory:") // Use ":memory:" for a temporary, non-persistent database
}

// Test to verify creating a table in the database
#[test]
fn test_create_table() -> Result<()> {
    let db = setup_test_db()?; // Set up a test database
    db.create_table("books", vec!["title", "author"])?; // Create a "books" table
    Ok(()) // Test passes if successful
}

// Test to verify inserting a row into the "books" table
#[test]
fn test_insert_row() -> Result<()> {
    let db = setup_test_db()?;
    db.create_table("books", vec!["title", "author"])?;
    db.insert_row("books", vec!["The Catcher in the Rye", "J.D. Salinger"])?;
    Ok(())
}

// Test to verify querying data from the "books" table
#[test]
fn test_query_table() -> Result<()> {
    let db = setup_test_db()?;
    db.create_table("books", vec!["title", "author"])?;
    db.insert_row("books", vec!["The Catcher in the Rye", "J.D. Salinger"])?;
    db.query_table("books")?;
    Ok(())
}

// Test to verify deleting the "books" table from the database
#[test]
fn test_delete_table() -> Result<()> {
    let db = setup_test_db()?;
    db.create_table("books", vec!["title", "author"])?;
    db.delete_table("books")?;
    Ok(())
}
