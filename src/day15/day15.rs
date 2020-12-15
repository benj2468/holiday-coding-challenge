use std::fs;
use std::error::Error;
use std::env;
use std::iter::Iterator;
use std::collections::HashMap;


struct Game {
    last_spoken: usize,
    current_turn: usize,
    acc_spoken: HashMap<usize, Vec<usize>>
}

impl Game {
    fn new(starting_numbers: impl Iterator<Item = usize>) -> Self {
        let mut acc_spoken = HashMap::new();
        let mut last_spoken = None;
        let mut current_turn = 1;
        for t in starting_numbers {
            let cases = acc_spoken.entry(t).or_insert(Vec::new());
            cases.push(current_turn);

            last_spoken = Some(t);
            current_turn += 1;
        }
        if let Some(last_spoken) = last_spoken {
            return Game {
                last_spoken,
                current_turn,
                acc_spoken,
            }
        }
        panic!("No Starting Numbers at all!")
    }

    fn get_next_number(&self) -> usize {
        let current_last_cases = self.acc_spoken.get(&self.last_spoken).unwrap();
        let len = current_last_cases.len();
        if len == 1 {
            return 0
        } else {
            return current_last_cases[len - 1] - current_last_cases[len - 2]
        }
    }

    fn turn(&mut self) {
        let next_number = self.get_next_number();

        self.last_spoken = next_number;

        let e = self.acc_spoken.entry(next_number).or_insert(Vec::new());
        e.push(self.current_turn);

        self.current_turn += 1;
    }

    fn execute_until(&mut self, until: usize) {
        while self.current_turn <= until {
            self.turn();
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {

    let args: Vec<String> = env::args().collect();

    let file = read_file_lines(&args[1]).unwrap();

    let starting_numbers = file
        .split(',')
        .map(|v| v.parse::<usize>())
        .filter_map(|v| v.ok());

    let mut game = Game::new(starting_numbers);

    game.execute_until(args[2].parse::<usize>().expect("Error parsing"));

    println!("The last number spoken is {}", game.last_spoken);

    Ok(())
}

fn read_file_lines(file_name: &str) -> Result<String, Box<dyn Error>> {
    let file = fs::read_to_string(file_name)?;

    Ok(file)
}