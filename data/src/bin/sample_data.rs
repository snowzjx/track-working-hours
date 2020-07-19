extern crate data;

use self::data::*;

fn main() {
    let projects = show_project();
    println!("{:?}", projects);

    let user = select_user("snow", "123").unwrap().unwrap(); // Result -> Option -> User
    println!("{:?}", user);

    let trackings = select_trackings_by_user(&user);
    println!("{:?}", trackings);

    let project = &projects.unwrap()[0];
    let trackings = select_trackings_by_project(&project);
    println!("{:?}", trackings);
}