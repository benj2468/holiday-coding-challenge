
use std::fs;
use std::error::Error;
use std::io::{BufReader};
use std::env;
use std::str::{FromStr, Lines};
use std::iter::Iterator;
use std::num::ParseIntError;
use std::collections::HashMap;

type Mask = Vec<char>;

const BASE: usize = 2;

struct Memory {
    mask: Mask,
    ram: HashMap<usize, usize>,
}

trait Mem {
    fn new() -> Self;
    fn put(&mut self, location: usize, value: usize);
    fn update_mask(&mut self, mask: Mask);
    fn apply(&mut self, instr: Instruction);
}

impl Mem for Memory {
    fn new() -> Self {
        return Memory {
            mask: vec![],
            ram: HashMap::new()
        }
    }

    fn put(&mut self, location: usize, value: usize) {
        self.ram.insert(location, value);
    }

    fn update_mask(&mut self, mask: Mask) {
        self.mask = mask;
    }

    fn apply(&mut self, instr: Instruction) {
        match instr {
            Instruction::Entry {location, value} => {
                for loc in part_two_get_locations(&self, location) {
                    self.put(loc, value);
                }
            } 
            Instruction::Mask(mask) => { self.update_mask(mask); }
        }

        ()
    }
}

enum Instruction {
    Mask(Mask),
    Entry { location: usize, value: usize }
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split(" = ").collect::<Vec<&str>>();
        if split[0] == "mask" {
            let mask = split[1].chars().rev().collect::<Vec<char>>();
            return Ok(Instruction::Mask(mask))
        } else {
            let location = &split[0][4..split[0].len()-1].parse::<usize>().expect("Error parsing mem location");
            let value = split[1].parse::<usize>().expect("Error parsing value");
            return Ok(Instruction::Entry { location: *location, value })
        }
    }
}

trait ToMem : Iterator {
    fn to_mem<'a, T: Mem>(&'a mut self) -> T 
        where Self: Sized + Iterator<Item = Instruction>
    {
        let mut mem = T::new();
        self.for_each(|inst| mem.apply(inst));
        mem
    }
}

impl<T: ?Sized> ToMem for T where T: Iterator { }

fn main() -> Result<(), Box<dyn Error>> {

    let args: Vec<String> = env::args().collect();

    // let mut memory = Memory::new();

    let memory = read_file_lines(&args[1])
        .expect("error reading file")
        .lines()
        .into_iter()
        .map(|line| line.parse::<Instruction>())
        .filter_map(|f| f.ok())
        .to_mem::<Memory>();

    let sum: usize = memory.ram.values().sum();

    println!("Solution: {}", sum);

    Ok(())
}

fn read_file_lines(file_name: &str) -> Result<String, Box<dyn Error>> {
    let file = fs::read_to_string(file_name)?;

    Ok(file)
}

fn check_k_bit(value: usize, k: u32) -> bool {
    let mut helper = value;
    for _i in 0..k {
        helper = helper >> 1;
    }
    helper % 2 == 1
}

fn get_positions_recursive_helper(idx: u32, wild_cards: &Vec<u32>, min_location: usize) -> Vec<usize> {
    
    if wild_cards.len() == idx as usize {
        return vec![min_location]
    }
    
    let my_pos = wild_cards[idx as usize];

    let mut locations = vec![];
    get_positions_recursive_helper(idx + 1, wild_cards, min_location).iter()
        .for_each(|loc| locations.push(*loc));
    get_positions_recursive_helper(idx + 1, wild_cards, min_location + (BASE.pow(my_pos) as usize)).iter()
        .for_each(|loc| locations.push(*loc));

    locations
}

fn part_two_get_locations(memory: &Memory, location: usize) -> Vec<usize> {

    let mut fixed_location = location;
    let mut wild_cards = vec![];

    for (b_digit, mask_val) in memory.mask.iter().enumerate().map(|(i,val)| (i as u32, val)) {
        if *mask_val == '1'  && !check_k_bit(location, b_digit){
            fixed_location += BASE.pow(b_digit);
        }
        if *mask_val == 'X' {
            wild_cards.push(b_digit);
            if check_k_bit(location, b_digit) { 
                fixed_location -= BASE.pow(b_digit); 
            }
        }

    }

    get_positions_recursive_helper(0, &wild_cards, fixed_location)
}