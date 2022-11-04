use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct Person {
  id  : i32,
  name: String,
  age : u8,
  data: String,
}

fn main() -> Result<()> {
  let conn = Connection::open_in_memory()?;

  conn.execute(
    "CREATE TABLE person (
      id   INTEGER PRIMARY KEY,
      name TEXT NOT NULL,
      age  INTEGER,
      data TEXT
    )",
    [],
  )?;
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
  
  let age: u8 = 1;
  
  let mut stmt = conn.prepare(
    "SELECT id, name, age, data FROM person WHERE age=:age;")?;

  let person_iter = stmt.query_map(&[(":age", age.to_string().as_str())], |row| {
    Ok(Person {
      id  : row.get(0)?,
      name: row.get(1)?,
      age : row.get(2)?,
      data: row.get(3)?,
    })
  })?;

  for person in person_iter {
    println!("Found person {:?}", person);
  }

  Ok(())
}