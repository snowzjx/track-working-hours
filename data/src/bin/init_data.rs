extern crate data;

use self::data::*;

fn main() {
    let project = create_project("其他项目", "Other URLs" , 1);
    println!("Created {:?}", project);
    
    let project = create_project("Cloud Kitchen", "Sample URL1", 5);
    println!("Created {:?}", project);

    let project = create_project("废钢质量检判断", "Sample URL2", 5);
    println!("Created {:?}", project);

    let project = create_project("烧桔颗粒度检测", "Sample URL3", 4);
    println!("Created {:?}", project);

    let user = create_user("snow", "123", "Junxue ZHANG");
    println!("Created {:?}", user);

    let tracking = create_tracking("snow", 1, 0.5);
    println!("{:?}", tracking);

    let tracking = create_tracking("snow", 2, 0.5);
    println!("{:?}", tracking);

    let tracking = create_tracking("snow", 10, 0.5);
    println!("{:?}", tracking);

    let tracking = create_tracking("test", 1, 0.5);
    println!("{:?}", tracking);
}