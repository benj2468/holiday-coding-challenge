use std::fs::File;
use std::error::Error;
use std::io::{BufRead, BufReader, Lines};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error>> {
    // File hosts must exist in current path before this produces output
    
    let lines = read_file_lines("./test.txt").expect("error");

    let mut lines_vector = Vec::new();
    lines_vector.push(0);
    for line in lines.filter_map(|result| result.ok()) {
        lines_vector.push(line.parse::<i64>().expect("cannot convert to integer"))
    }
    // Consumes the iterator, returns an (Optional) String

    lines_vector.sort();
    let mut ones = 0;
    let mut threes = 1;
    for (cur, next) in lines_vector.windows(2).map(|pair| (pair[0], pair[1])) {
        let diff = next - cur;
        if diff == 1 {ones = ones + 1}
        if diff == 3 {threes = threes + 1}
    }
    println!("1: Answer {}", ones * threes);

    let mut memory =  HashMap::new();
    let arr = get_arrangements(&lines_vector[..], &mut memory).expect("").0;
    println!("2: Solution {}", arr);

    Ok(())
}

fn read_file_lines(file_name: &str) -> Result<Lines<BufReader<File>>, Box<dyn Error>> {
    let file = File::open(file_name).expect("cannot open file");
    let file = BufReader::new(file);

    Ok(file.lines())
}

fn get_arrangements<'a>(list: &[i64], memory: &'a mut HashMap<i64, i64>) -> Result<(i64, &'a mut HashMap<i64, i64>), Box<dyn Error>>   {
    let mut arr = 0;
    if list.len() == 1 { return Ok((1, memory)) }
    let zeroth = list.get(0).expect("");

    for (i, key) in list.iter().enumerate() {
        if i == 0 { continue }
        if key - zeroth > 3 { break }

        if !memory.contains_key(key) {
            let (val, memory) = get_arrangements(&list[i..], memory).expect("Could not get arrangements");
            memory.insert(*key, val);
        }
        let val = memory.get(key).expect("");
        arr = arr + val;
    }
    Ok((arr, memory))
}