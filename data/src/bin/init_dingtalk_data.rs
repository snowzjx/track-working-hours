extern crate data;

use self::data::*;

fn main() {

    let assign = create_assign("231432234224074112", 1);
    println!("Created {:?}", assign);

    let assign = create_assign("231432234224074112", 2);
    println!("Created {:?}", assign);

    let assign = create_assign("231432234224074112", 3);
    println!("Created {:?}", assign);
}