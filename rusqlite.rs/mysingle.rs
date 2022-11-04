use rusqlite::{Connection, Result, named_params};

#[derive(Debug, Clone)]
struct Person {
  name: String
}

fn main() -> Result<()> {
  let conn = Connection::open("_myfile.db")?;

  let age: u8 = 1;
  let mut stmt = conn.prepare("SELECT name FROM person WHERE age=?1")?;
  let person_iter = stmt.query_map([age.to_string().as_str()], |row| {
    Ok(Person { name: row.get(0)? })
  })?;
  for person in person_iter {
    let persn = person.unwrap().clone();
    println!("1) Found person {}", persn.name);
  }

  let mut stmt1 = conn.prepare("SELECT name FROM person WHERE age=:age")
    .expect("select failed");
  let mut rows = stmt1.query_named(named_params!{ ":age": age.to_string().as_str() })
    .expect("rows failed");
  while let Some(row) = rows.next().expect("while row failed") {
    let name1: String = row.get(0).expect("get row failed");
    println!("2) {}", name1);
  }

  let mut stmt2 = conn.prepare("SELECT name FROM person WHERE age=?1")?;
  let mut rows = stmt2.query([age.to_string().as_str()])?;
  while let Some(row) = rows.next().expect("while row failed") {
    let name2: String = row.get(0)?;
    println!("3) {}", name2);
  }

  let mut name3: String = "".to_string();
  let mut stmt3 = conn.prepare("SELECT name FROM person WHERE age=?1")?;
  stmt3.query_row([age.to_string().as_str()], |row| {
    Ok(name3 = row.get(0).unwrap())
  })?;
  println!("4) {}", name3);

  let mut name4: String = "".to_string();
  conn.query_row("SELECT name FROM person WHERE age=?1", [age], |row| {
    Ok(name4 = row.get(0).unwrap())
  })?;
  println!("5) {}", name4);

/*
  let mut name5: String = "".to_string();
  match conn.execute("SELECT name FROM person WHERE age=?1",
    [age.to_string().as_str()]) {
    Ok(col) => { println!("{:?}", col) }
    Err(err) => { println!("error {}", err) }
  }
  println!("6) Found person {}", name5);
*/

  Ok(())
}
