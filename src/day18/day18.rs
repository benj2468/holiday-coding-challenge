use std::fs::File;
use std::error::Error;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

fn evaluate(str: &[char]) -> (usize, usize) {
    
    let mut op = '+';
    let mut eval = 0;
    let mut idx = 0;
    while idx < str.len() {
        let char = str[idx];
        if char == '(' {
            let (rhs, diff) = evaluate(&str[idx+1..]);
            if op == '+' { eval += rhs; }
            if op == '*' { eval *= rhs; }
            idx += diff + 1;
        }
        else if char == ')' {
            return (eval, idx)
        }
        else if idx % 2 == 1 {
            op = char;
        }
        else if let Some(rhs) = char.to_digit(10) {
            if op == '+' { 
                eval += rhs as usize;
            }
            else if op == '*' { eval *= rhs as usize; }
        }
        idx += 1
    }

    return (eval, idx)
}

fn main() -> Result<(), Box<dyn Error>> {

    let args: Vec<String> = env::args().collect();

    let lines = read_lines(&args[1]).unwrap().filter_map(|f| f.ok());
    let lines = lines.map(|f| f.chars().filter(|f| *f != ' ').collect::<Vec<char>>());

    let mut tot = 0;
    for line in lines {
        tot += evaluate(&line[..]).0;
    }

    println!("{}", tot);

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}