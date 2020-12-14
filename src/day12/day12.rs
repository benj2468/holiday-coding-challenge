use std::fs::File;
use std::error::Error;
use std::io::{BufRead, BufReader, Lines};

enum Command {
    N,
    S,
    E,
    W,
    R,
    L,
    T
}

struct Position {
    x: i32,
    y: i32,
}

fn main() -> Result<(), Box<dyn Error>> {
    // File hosts must exist in current path before this produces output
    
    let lines = read_file_lines("./test.txt").expect("error");

    let ship = Position {
        x: 0,
        y: 0
    };
    let waypoint = Position {
        x: 10,
        y: 1
    };

    for line in lines.filter_map(|result| result.ok()) {
        let (ship, waypoint) = process_line(&ship, &waypoint, &line);
    }

    Ok(())
}

fn read_file_lines(file_name: &str) -> Result<Lines<BufReader<File>>, Box<dyn Error>> {
    let file = File::open(file_name).expect("cannot open file");
    let file = BufReader::new(file);

    Ok(file.lines())
}

fn process_line<'a>(ship: &Position, waypoint: &Position, line: &str) -> (Position, Position) {
    let command = &line[..0];
    let value = &line[1..].parse::<i32>().expect("Could not parse value");
    println!("{}, {}", command, value);

    return ('a::ship, 'a::waypoint)
}