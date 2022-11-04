#[derive(Debug)]
struct Data {
  id: String,
  nm: String,
}

fn main() {
  let mut x: Vec<Data> = Vec::new();
  let mut some:Data = Data{
    id:String::from("bar8tl"),
    nm:String::from("Ricardo Barrera")
  };
  x.push(some);
  some = Data{
    id:String::from("bir8mtp"), 
    nm:String::from("Rafal Bielski")
  };
  x.push(some);
  println!("{:?}", x);

  println!("{} {}", &x[0].id, &x[0].nm);
  println!("{} {}", x[1].id, x[1].nm);

  for i in &x {
    println!("{} {}", i.id, i.nm);
  }
}
