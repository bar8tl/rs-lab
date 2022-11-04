use rusqlite::{params, Connection, Result};

fn main() -> Result<()> {
  let conn = Connection::open("_myfile.db")?;

  let age  = 8;
  let name = "john";

  match conn.execute("UPDATE person SET age = ?1 WHERE name = ?2", (age, name)) {
    Ok(updated) => println!("{} rows were updated", updated),
    Err(err)    => println!("update failed: {}", err),
  }
  Ok(())
}


