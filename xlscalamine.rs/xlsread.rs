use calamine::{Reader, Xlsx, open_workbook};

fn main() {
  let mut excel: Xlsx<_> = open_workbook("_file.xlsx").unwrap();
  if let Some(Ok(r)) = excel.worksheet_range("Sheet1") {
    for row in r.rows() {
      println!("row={:?}, row[0]={:?}", row, row[0]);
    }
  }
}
