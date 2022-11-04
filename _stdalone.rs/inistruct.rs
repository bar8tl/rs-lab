struct ColTp {
  fld: String,
  wdt: f64,
  dsc: String
}

fn main() {
  let data1 = vec![
    ColTp{fld:"ackno".to_string(), wdt:10.29, dsc:"ack#".to_string()        },
    ColTp{fld:"issue".to_string(), wdt:11.30, dsc:"remitente".to_string()   },
    ColTp{fld:"rceiv".to_string(), wdt:12.00, dsc:"destinatario".to_string()},
    ColTp{fld:"invoi".to_string(), wdt:13.14, dsc:"refProveedor".to_string()},
    ColTp{fld:"serie".to_string(), wdt: 5.30, dsc:"serie".to_string()       },
    ColTp{fld:"folio".to_string(), wdt: 7.00, dsc:"folio".to_string()       },
    ColTp{fld:"uuidn".to_string(), wdt:41.00, dsc:"uuid".to_string()        },
    ColTp{fld:"dtime".to_string(), wdt:18.90, dsc:"fechahora".to_string()   },
    ColTp{fld:"stats".to_string(), wdt: 7.60, dsc:"estatus".to_string()     },
    ColTp{fld:"errn1".to_string(), wdt:33.00, dsc:"error".to_string()       },
    ColTp{fld:"errn2".to_string(), wdt:33.00, dsc:"error".to_string()       },
    ColTp{fld:"notas".to_string(), wdt:10.00, dsc:"notas".to_string()       }
  ];
  for dt in data1 {
    println!("{} {} {}", dt.fld, dt.wdt, dt.dsc);
  }
}