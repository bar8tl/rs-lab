#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use serde::Deserialize;
use std::fs::File;

#[derive(Debug, Deserialize)]
pub struct rutaTp {
  pub remitente   : String,
  pub destinatario: String
}

#[derive(Debug, Deserialize)]
pub struct documentoTp {
  pub referenciaProveedor: String,
  pub serie              : String,
  pub folioFiscal        : String,
  pub UUID               : String
}

#[derive(Debug, Deserialize)]
pub struct recepcionTp {
  pub fechahora: String,
  pub estatus  : String
}

#[derive(Debug, Deserialize)]
pub struct acuseReciboTp {
  pub ruta     : rutaTp,
  pub documento: documentoTp,
  pub recepcion: recepcionTp,
  pub error    : Vec<String>
}

fn main() {
  let s = "_fcaack.xml";
  let f = File::open(s).expect(&format!("Cannot open file {}", s));
  let prj: acuseReciboTp = serde_xml_rs::de::from_reader(f).unwrap();
  println!("{:?}", prj);
  println!("{:?} {:?}", prj.ruta.remitente, prj.error[0]);
}
