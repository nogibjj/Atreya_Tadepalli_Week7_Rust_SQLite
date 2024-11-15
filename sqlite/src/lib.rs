use csv::ReaderBuilder; //for loading from csv
use rusqlite::{Connection, Result};
use rusqlite::params;
use std::error::Error;
use std::fs::File; //for loading csv //for capturing errors from loading
                   // Here we will have a function for each of the commands

pub fn create_table(table_name: &str) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open("baseball.db")?;
    let create_query = format!(
        "CREATE TABLE IF NOT EXISTS {} (
            team TEXT,
            wins INTEGER,
            losses INTEGER,
            year INTEGER,
            League TEXT
        )",
        table_name
    );
    conn.execute(&create_query, [])?;
    println!("Table '{}' created successfully.", table_name);
    Ok(()) //returns nothing except an error if it occurs
}

pub fn load_csv(table_name: &str, file_path: &str) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open("baseball.db")?;
    let file = File::open(file_path).expect("failed to open the file path");
    let mut rdr = ReaderBuilder::new().from_reader(file);
    let insert_query = format!("INSERT INTO {} (team, wins, losses, year, League) 
    VALUES (?, ?, ?, ?, ?)", table_name);
    
    for result in rdr.records() {
        let record = result?;
        let team: String = record[0].trim().to_string();
        let wins: i32 = record[1].trim().parse()?;
        let losses: i32 = record[2].trim().parse()?;
        let year: i32 = record[5].trim().parse()?;
        let league: String = record[6].trim().to_string();

        println!("team: {}, wins: {}, losses: {}, year: {}, League: {}",
         team, wins, losses, year, league);

        conn.execute(
            &insert_query,
            params![team, wins, losses, year, league],
        )?;
    }
    
    println!("Data loaded successfully from '{}' into table '{}'.", file_path, table_name);
    Ok(())
}

pub fn read_query(table_name: &str) -> Result<&'static str> {
    // Connect to the SQLite database
    let conn = Connection::open("baseball.db")?;

    // Prepare and execute the SQL query to select the first 20 rows
    let query = format!("SELECT * FROM {} LIMIT 20", table_name);
    let mut stmt = conn.prepare(&query)?;
    
    // Execute the query and fetch the rows
    let rows = stmt.query_map(params![], |row| {
        let column1: String = row.get(0)?;
        let column2: i32 = row.get(1)?;
        let column3: i32 = row.get(2)?;
        let column4: i32 = row.get(3)?;
        let column5: String = row.get(4)?;
        
        // Return a tuple or a struct; here we use a tuple for simplicity
        Ok((column1, column2, column3, column4, column5))
    })?;

    // Print the top 20 rows
    println!("Top 20 rows of the Baseball table:");
    for row in rows {
        match row {
            Ok(data) => println!("{:?}", data),
            Err(e) => eprintln!("Error fetching row: {}", e),
        }
    }

    // Connection is closed automatically when `conn` goes out of scope
    Ok("Success")
}

pub fn update_table(table_name: &str, set_clause: &str, condition: &str) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open("baseball.db")?;
    let update_query = format!("UPDATE {} SET {} WHERE {}", table_name, set_clause, condition);
    let affected_rows = conn.execute(&update_query, [])?;
    println!("Successfully updated {} row(s) in the table '{}'.", affected_rows, table_name);
    Ok(())
}

pub fn drop_table(table_name: &str) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open("baseball.db")?;
    let drop_query = format!("DROP TABLE IF EXISTS {}", table_name);
    conn.execute(&drop_query, [])?;
    println!("Table '{}' dropped successfully.", table_name);
    // / Hi
    Ok(())
}

// TEST FUNCTIONS

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::{Connection, Result};
    // use std::io::Write;

    #[test]
    fn test_create_table() -> Result<(),Box<dyn Error>> {
        let conn = Connection::open_in_memory()?;
        create_table("test_table")?;
        let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='test_table'")?;
        let mut rows = stmt.query([])?;
        assert!(rows.next().expect("Failed to retrieve row").is_some());
        Ok(())
    }

    #[test]
    fn test_load_csv() -> Result<(), Box<dyn Error>> {
        let conn = Connection::open_in_memory()?;
        create_table("test_table")?;
        load_csv("test_table", "data/mlb-test.csv")?;

        // Check that the table loaded some rows
        let mut stmt = conn.prepare("SELECT COUNT(*) FROM test_table")?;
        let count: i32 = stmt.query_row([], |row| row.get(0))?;
        assert!(count > 0, "No rows were entered into the test table");

        Ok(())
    }

    // Test function to verify the `drop_table` behavior
    #[test]
    fn test_drop_table() -> Result<(), Box<dyn std::error::Error>> {
        let conn = Connection::open_in_memory()?;
        create_table("test_table")?;
        drop_table("test_table")?;
        let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='test_table'")?;
        let mut rows = stmt.query([])?;
        assert!(rows.next().expect("Failed to retrieve row").is_none());
        Ok(())
    }

}