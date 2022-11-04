use serde::Deserialize;
use std::fs::File;

#[derive(Debug, Deserialize)]
struct Item {
  name  : String,
  source: String
}

#[derive(Debug, Deserialize)]
struct Project {
  pub name : String,
  #[serde(rename = "Item", default)]
  pub items: Vec<Item>,
}

fn main() {
  let s = "_project.xml";
  let f = File::open(s).expect(&format!("Cannot open file {}", s));
  let prj: Project = serde_xml_rs::de::from_reader(f).unwrap();
  println!("{:?} {:?}", prj.name, prj.items[1].source);
}
