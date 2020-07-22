extern crate data;

use self::data::*;

fn main() {

    let project1 = create_project("Cloud Kitchen", "POC", "Sample URL1", 5).unwrap();
    println!("Created {:?}", project1);

    let project2 = create_project("废钢质量检判断", "正式", "Sample URL2", 5).unwrap();
    println!("Created {:?}", project2);

    let project3 = create_project("烧桔颗粒度检测", "POC", "Sample URL3", 4).unwrap();
    println!("Created {:?}", project3);

    let project4 = create_project("其他项目", "调研", "Other URLs" , 1).unwrap();
    println!("Created {:?}", project4);

    let user_snow = create_user("snow", "123", "Junxue ZHANG", true).unwrap();
    println!("Created {:?}", user_snow);

    let user_junhuan = create_user("junhuan", "123", "Junhuna Sun", false).unwrap();
    println!("Created {:?}", user_snow);

    let assign = create_assign(&user_snow.username, project1.id);
    println!("Created {:?}", assign);

    let assign = create_assign(&user_snow.username, project2.id);
    println!("Created {:?}", assign);

    let assign = create_assign(&user_snow.username, project3.id);
    println!("Created {:?}", assign);

    let assign = create_assign(&user_snow.username, project4.id);
    println!("Created {:?}", assign);

    let assign = create_assign(&user_junhuan.username, project1.id);
    println!("Created {:?}", assign);

    let assign = create_assign(&user_junhuan.username, project4.id);
    println!("Created {:?}", assign);
}