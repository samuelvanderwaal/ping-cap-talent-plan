#[macro_use]
extern crate serde;

// use std::io::{Read, Write};
use std::fs::File;
use bson::{decode_document, encode_document};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Debug, Deserialize, Serialize)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0, 4) {
            0 => Direction::North,
            1 => Direction::South,
            2 => Direction::East,
            _ => Direction::West,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Move {
    num_squares: i32,
    direction: Direction,
}

fn main() {
    let mut writer = File::create("move.bson").unwrap();

    for n in 0..1000 {
        let direction: Direction = rand::random();
        let m = Move { num_squares: n, direction: direction};
        let ser_bson = bson::to_bson(&m).unwrap();
        if let bson::Bson::Document(document) = ser_bson {
            encode_document(&mut writer, &document).unwrap();
        } else {
            println!("Error converting {} document!", n);
        }
    }

    let mut reader = File::open("move.bson").unwrap();

    while let Ok(decoded) = decode_document(&mut reader) {
        println!("{:?}", decoded);
    }

}