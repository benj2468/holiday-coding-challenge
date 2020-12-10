use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut lines_vector = Vec::new();
        lines_vector.push(0);
        for line in lines {
            if let Ok(ip) = line {
                lines_vector.push(ip.parse::<i64>().unwrap())
            }
        }
        lines_vector.sort();
        let mut i = 0;
        let mut ones = 0;
        let mut threes = 1;
        while i < lines_vector.len() - 1 {
            if let Some(next) = lines_vector.get(i+1) {
                if let Some(cur) = lines_vector.get(i) {
                    let diff = next - cur;
                    if diff == 1 {ones = ones + 1}
                    if diff == 3 {threes = threes + 1}
                }
            }
            i = i + 1
        }
        println!("1: Answer {}", ones * threes);

        let mut memory =  HashMap::new();
        let arr = get_arrangements(&lines_vector[..], &mut memory).0;
        println!("2: Solution {}", arr)
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_arrangements<'a>(list: &[i64], memory: &'a mut HashMap<i64, i64>) -> (i64, &'a mut HashMap<i64, i64>)  {
    let mut arr = 0;
    if list.len() == 1 { return (1, memory) }
    let mut i = 1;
    while i < list.len() && can_execute(list.get(i), list.get(0)) {
        if let Some(key) = list.get(i) {
            if !memory.contains_key(key) {
                let (val, memory) = get_arrangements(&list[i..], memory);
                memory.insert(*key, val);
            }
            if let Some(val) = memory.get(key) {
                arr = arr + val;
            }
        }
        i = i + 1
    }
    (arr, memory)
}

fn can_execute(a: Option<&i64>, b: Option<&i64>) -> bool {
    if let Some(a) = a {
        if let Some(b) = b {
            return a - b <= 3
        }
    }
    return false
}