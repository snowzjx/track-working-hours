extern crate export_csv;

use self::export_csv::*;

fn main() {
    let results = export_csv();
    println!("{:?}", results);
}