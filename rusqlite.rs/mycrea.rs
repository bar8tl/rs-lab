use rusqlite::{params, Connection, Result};

fn main() -> Result<()> {
  let conn = Connection::open("_myfile.db")?;

  conn.execute(
    "CREATE TABLE person (
      id   INTEGER PRIMARY KEY,
      name TEXT NOT NULL,
      age  INTEGER,
      data TEXT
    )",
    [],
  )?;

  Ok(())
}
