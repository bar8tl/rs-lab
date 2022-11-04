use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct Person {
  id  : i32,
  name: String,
  age : u8,
  data: String,
}

fn main() -> Result<()> {
  let conn = Connection::open("_myfile.db")?;

  let me = Person {
    id  : 0,
    name: "Steven".to_string(),
    age : 1,
    data: "data1".to_string(),
  };
  conn.execute(
    "INSERT INTO person (name, age, data) VALUES (?1, ?2, ?3)",
      params![me.name, me.age, me.data],
  )?;

  conn.execute(
    "INSERT INTO person (name, age, data) VALUES (?1, ?2, ?3)",
      params!["john".to_string(), 2, "data2".to_string()],
  )?;
  
  Ok(())
}