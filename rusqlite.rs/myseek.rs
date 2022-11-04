use rusqlite::{Connection, Result};

fn main() -> Result<()> {
  let conn = Connection::open("_myfile.db")?;

  let age: u8 = 2;
  let mut found: bool = false;

  let rsult = conn.query_row("SELECT name FROM person WHERE age=?1",
    [age], |row| { Ok(()) });
  match rsult{
    Ok(dummy)  => { found = true;  }
    Err(error) => { found = false; }
  }
  println!("Record found: {}", found);
  Ok(())
}

/* version 1. combining match and query statements
use rusqlite::{Connection, Result};

fn main() -> Result<()> {
  let conn = Connection::open("_myfile.db")?;

  let age: u8 = 2;
  let mut fnd: bool = false;
  
  match conn.query_row(
    "SELECT name FROM person WHERE age=?1", [age], |row| { Ok(()) }) {
      Ok(dummy)  => { fnd = true;  }
      Err(error) => { println!("{:?}", error); fnd = false; }
  }
  println!("Record found: {}", fnd);
  Ok(())
}
*/
