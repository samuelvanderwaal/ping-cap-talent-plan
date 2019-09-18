#[macro_use]
extern crate serde;

use std::fmt;
use std::error::Error;
use std::path::Path;
use std::process;
use std::io::{self, Read, Write};
use std::fs::File;
use ron;
use serde_json;

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
    let a = Move { num_squares: 7, direction: Direction::South};
    let j = serde_json::to_string(&a).unwrap();
    let mut buffer1: Vec<u8> = Vec::new();
    let mut buffer2: Vec<u8> = Vec::new();
    buffer1.write_all(&j.as_bytes()).unwrap();
    println!("Json as u8 bytes: {:?}", buffer1);
    println!("Json as a string: {:?}", std::str::from_utf8(&buffer1).unwrap());

    let r = ron::ser::to_string(&a).unwrap();
    buffer2.write_all(&r.as_bytes()).unwrap();
    println!("Ron as u8 bytes: {:?}", buffer2);
    println!("Ron as a String: {:?}", std::str::from_utf8(&buffer2).unwrap());
}