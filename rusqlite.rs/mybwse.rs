use rusqlite::{params, Connection, Result};

#[derive(Debug, Clone)]
struct Person {
  id  : i32,
  name: String,
  age : u8,
  data: String,
}

fn main() -> Result<()> {
  let conn = Connection::open("_myfile.db")?;

  let age: u8 = 1;
  
  let mut stmt = conn.prepare(
    "SELECT id, name, age, data FROM person WHERE age=:age;")?;

  let person_iter = stmt.query_map(&[(":age", age.to_string().as_str())],
    |row| {
      Ok(Person {
        id  : row.get(0)?,
        name: row.get(1)?,
        age : row.get(2)?,
        data: row.get(3)?,
      }
    )
  })?;

  for person in person_iter {
    let persn = person.unwrap().clone();
    println!("Found person {} {}", persn.name, persn.age);
  }
  Ok(())
}


/* Para determinar el tipo de una variable
    print_type_of(&person);
fn print_type_of<T>(_: &T) {
  println!("{}", std::any::type_name::<T>());
}
*/
