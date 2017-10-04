#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate serde_yaml;

#[derive(Serialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Serialize, Debug)]
struct Line {
    p1: Point,
    p2: Point,
}

fn main() {

    let point = Line {
        p1: Point { x: 0, y: 0 },
        p2: Point { x: 10, y: 10 },
    };

    let json = serde_json::to_string(&point).unwrap();
    let yaml = serde_yaml::to_string(&point).unwrap();

    println!("{:?}", point);
    println!("{}", json);
    println!("{}", yaml);

}
