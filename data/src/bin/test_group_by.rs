extern crate data;

use self::data::*;

fn main() {
    let results = select_grouped_trackings();
    println!("{:?}", results);
}