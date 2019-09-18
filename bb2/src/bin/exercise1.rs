#[macro_use]
extern crate serde;

use std::fmt;
use std::error::Error;
use std::path::Path;
use std::process;
use std::io::{self, Read, Write};
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

#[derive(Debug)]
enum CustomError {
    Io(io::Error),
    JsonError(serde_json::Error),
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CustomError::Io(ref err) => err.fmt(f),
            CustomError::JsonError(ref err) => err.fmt(f),
        }
    }
}

impl Error for CustomError {
    fn description(&self) -> &str {
        match *self {
            CustomError::Io(ref err) => err.description(),
            CustomError::JsonError(ref err) => err.description(),
        }
    }
}

impl From<io::Error> for CustomError {
    fn from(err: io::Error) -> CustomError {
        CustomError::Io(err)
    }
}

impl From<serde_json::Error> for CustomError {
    fn from(err: serde_json::Error) -> CustomError {
        CustomError::JsonError(err)
    }
}

fn main() {
    let a = Move { num_squares: 10, direction: Direction::West};
    let j = serde_json::to_string(&a).unwrap();

    match write_to_file("move.ron", j) {
    Err(e) =>  {
        println!("{} {}", e, e.description());
        process::exit(1);
    },
    Ok(_) => ()
    }

    let contents: String = match read_from_file("move.ron") {
        Err(e) => {
            println!("{} {:?}", e, e.description());
            process::exit(1);
        },
        Ok(contents) => contents,
    };

    let b: Move = serde_json::from_str(&contents).unwrap();
    println!("{:?}", a);
    println!("{:?}", b);
}

fn write_to_file<P: AsRef<Path>>(path: P, r: String) -> Result<(), CustomError> {
    let mut writer = File::create(&path)?;
    writer.write_all(r.as_bytes())?;
    Ok(())
}

fn read_from_file<P: AsRef<Path>>(path: P) -> Result<String, CustomError> {
    let mut reader = File::open(&path)?;
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    Ok(contents)
}