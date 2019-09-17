#[macro_use]
extern crate serde;

use std::io::{Read, Write};
use serde_json;
use std::fs::File;

#[derive(Debug, Deserialize, Serialize)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Deserialize, Serialize)]
struct Move {
    num_squares: u32,
    direction: Direction,
}

fn main() {
    let a = Move { num_squares: 10, direction: Direction::West};
    let j = serde_json::to_string(&a).unwrap();
    let mut writer = File::create("move.json").unwrap();
    writer.write_all(j.as_bytes()).unwrap();

    let mut reader = File::open("move.json").unwrap();
    let mut contents = String::new();
    reader.read_to_string(&mut contents).unwrap();
    let b: Move = serde_json::from_str(&contents).unwrap();
    println!("{:?}", a);
    println!("{:?}", b);
}

