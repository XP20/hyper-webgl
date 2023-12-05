use std::fs;

mod obj_parse;
use obj_parse::obj::parse;

fn main() {
    let str = fs::read_to_string("../web/public/cube.obj").unwrap();
    let (res1, res2) = parse(&str);
    println!("{:?}", res1);
    println!("{:?}", res2);
}
