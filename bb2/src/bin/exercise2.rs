#[macro_use]
extern crate serde;

use std::fmt;
use std::error::Error;
use std::path::Path;
use std::process;
use std::io::{self, Read, Write};
use std::fs::File;
use ron::de::from_str;
use ron::ser::to_string;

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
    RonError(ron::ser::Error),
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CustomError::Io(ref err) => err.fmt(f),
            CustomError::RonError(ref err) => err.fmt(f),
        }
    }
}

impl Error for CustomError {
    fn description(&self) -> &str {
        match *self {
            CustomError::Io(ref err) => err.description(),
            CustomError::RonError(ref err) => err.description(),
        }
    }
}

impl From<io::Error> for CustomError {
    fn from(err: io::Error) -> CustomError {
        CustomError::Io(err)
    }
}

impl From<ron::ser::Error> for CustomError {
    fn from(err: ron::ser::Error) -> CustomError {
        CustomError::RonError(err)
    }
}

fn main() {
    let a = Move { num_squares: 7, direction: Direction::South};
    let r = to_string(&a).expect("Serialization failed");
    match write_to_file("move.ron", r) {
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
    let b: Move = from_str(&contents).unwrap();
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