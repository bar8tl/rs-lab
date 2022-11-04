trait Speaks {
  fn speak(&self);
}

trait Animal {
  fn animal_type(&self) -> &str;
  fn noise(&self) -> &str;
}

trait Human {
  fn name(&self) -> &str;
  fn sentence(&self) -> &str;
}

impl<T> Speaks for T where T: Animal {
  fn speak(&self) {
    println!("The {} said {}", self.animal_type(), self.noise());
  }
}

/*
impl<T> Speaks for T where T: Human {
  fn speak(&self) {
    println!("{} said {}", self.name(), self.sentence());
  }
}
*/

struct Dog {}
struct Cat {}
//struct Person {}

impl Animal for Dog {
  fn animal_type(&self) -> &str {
    "dog"
  }

  fn noise(&self) -> &str {
    "woof"
  }
}

impl Animal for Cat {
  fn animal_type(&self) -> &str {
    "cat"
  }

  fn noise(&self) -> &str {
    "meow"
  }
}

fn main() {
  let dog = Dog {};
  let cat = Cat {};
  dog.speak();
  cat.speak();
}
