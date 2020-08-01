extern crate dingtalk;

use self::dingtalk::*;

fn main() {
    let result = get_token();
    println!("{:?}", result);
}