
use std::fs::File;
use std::error::Error;
use std::io::{BufRead, BufReader, Lines};
use std::env;
use std::collections::HashMap;

struct Memory {
    mask: HashMap<u32, usize>,
    ram: HashMap<usize, usize>,
}

enum BinaryChange {
    Add(u32),
    Subtract(u32),
    None,
}

fn check_k_bit(value: usize, k: u32) -> bool {
    let mut helper = value;
    for _i in 0..k {
        helper = helper >> 1;
    }
    helper % 2 == 1
}

fn get_positions_helper(idx: u32, wildCards: &Vec<&u32>, min_location: usize) -> Vec<usize> {
    let base: usize = 2;
    if wildCards.len() == idx as usize {
        return vec![min_location]
    }
    let mut locations = vec![];
    let my_pos = wildCards[idx as usize];
    for loc in get_positions_helper(idx + 1, &wildCards, min_location) {
        locations.push(loc);
    }
    for loc in get_positions_helper(idx + 1, &wildCards, min_location + (base.pow(*my_pos) as usize)) {
        locations.push(loc);
    }
    locations
}

impl Memory {

    fn get_true_locations(&self, location: usize) -> Vec<usize> {
        let base: usize = 2;
        let mut true_location = location;
        let mut wildCards: Vec<&u32> = vec![];
        for (b_digit, mask_val) in self.mask.iter() {
            if *mask_val == 1  && !check_k_bit(location, *b_digit){
                true_location += base.pow(*b_digit);
            }
            if *mask_val == 2 {
                wildCards.push(b_digit);
                if check_k_bit(location, *b_digit) { true_location -= base.pow(*b_digit); }
            }

        }

        get_positions_helper(0, &wildCards, true_location)
    }
    fn put(mut self, location: usize, value: usize) -> Self {
        let base: usize = 2;

        for loc in self.get_true_locations(location) {
            self.ram.insert(loc, value);
        }
        
        self
    }

    fn update_mask(mut self, mask: HashMap<u32, usize>) -> Self {
        self.mask = mask;
        return self
    }

    fn new() -> Self {
        return Memory {
            mask: HashMap::new(),
            ram: HashMap::new()
        }
    }

    fn handle_line(self, line: &str) -> Self {
        match Instruction::parse(line) {
            Instruction::Entry {location, value} => return self.put(location, value),
            Instruction::Mask(mask) => return self.update_mask(mask)
        }
    }
}

enum Instruction {
    Mask(HashMap<u32, usize>),
    Entry { location: usize, value: usize }
}

impl Instruction {
    fn parse_mask(mask_str: &str) -> HashMap<u32, usize> {
        let mut mask: HashMap<u32, usize> = HashMap::new();
        for (i, c) in mask_str.chars().rev().enumerate() {
            if c == 'X' { mask.insert(i as u32, 2); }
            else { mask.insert( i as u32, c.to_digit(2).unwrap() as usize); }
        }
        return mask
    }
    fn parse(line: &str) -> Self {
        let split1 = line.split(" = ").collect::<Vec<&str>>();
        if split1[0] == "mask" {
            return Instruction::Mask(Instruction::parse_mask(split1[1]))
        } else {
            let location = &split1[0][4..split1[0].len()-1].parse::<usize>().expect("Error parsing mem location");
            let value = split1[1].parse::<usize>().expect("Error parsing value");
            return Instruction::Entry { location: *location, value }
        }

    }
}
fn main() -> Result<(), Box<dyn Error>> {

    let args: Vec<String> = env::args().collect();

    let lines = read_file_lines(&args[1]).expect("error reading file").filter_map(|f| f.ok()).collect::<Vec<_>>();

    let mut memory = Memory::new();

    for line in lines.iter() {
        memory = memory.handle_line(line)
    }

    let sum: usize = memory.ram.values().sum();

    println!("Solution: {}", sum);

    Ok(())
}

fn read_file_lines(file_name: &str) -> Result<Lines<BufReader<File>>, Box<dyn Error>> {
    let file = File::open(file_name).expect("cannot open file");
    let file = BufReader::new(file);

    Ok(file.lines())
}