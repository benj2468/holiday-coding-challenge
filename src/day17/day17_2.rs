use std::fs;
use std::error::Error;
use std::env;
use std::cmp;
use std::iter::Iterator;
use std::collections::HashSet;

struct Conway {
    active_cubes: Points
}

type Point = (isize, isize, isize, isize);
type Points = HashSet<Point>;

impl Conway {
    fn new(init_state: String) -> Self {
        let mut active_cubes = HashSet::new();
        let lines = init_state.split('\n');
        for (i, line) in lines.enumerate() { // Column
            for (j, char) in line.chars().enumerate() {
                if char == '#' { active_cubes.insert((i as isize,j as isize,0 as isize, 0 as isize)); }
            }
        }

        Conway {
            active_cubes
        }
    }

    fn get_all_neighbors(&self) -> Points {
        let mut points = HashSet::new();
        for point in self.active_cubes.iter() {
            let neighbors = Conway::get_neighbors(point);
            points.extend(neighbors)
        }
        points
    }

    fn get_neighbors(point: &Point) -> Points {
        let mut points = HashSet::new();
        for i in -1..2 {
            for j in -1..2 {
                for k in -1..2 {
                    for h in -1..2 {
                        if (i,j,k,h) == (0,0,0,0) { continue }
                        points.insert(( point.0 + i, point.1 + j, point.2 + k, point.3 + h ));
                    }
                }
            }
        }
        points
    }

    fn cycle(&mut self) {
        let mut next_set = HashSet::new();
        for neighbor in self.get_all_neighbors().into_iter() {
            let mut active_neighbors = 0;
            for n in Conway::get_neighbors(&neighbor) {
                if self.active_cubes.contains(&n) { active_neighbors += 1; }
            }
            if self.active_cubes.contains(&neighbor) && [2,3].contains(&active_neighbors) {
                next_set.insert(neighbor);
            }
            else if !self.active_cubes.contains(&neighbor) && active_neighbors == 3 {
                next_set.insert(neighbor);
            }
        }
        println!("{}", next_set.len());
        self.active_cubes = next_set;        
    }
}



fn main() -> Result<(), Box<dyn Error>> {

    let args: Vec<String> = env::args().collect();

    let file = read_file(&args[1]).unwrap();

    let mut conway = Conway::new(file);

    for _ in 0..6 {
        conway.cycle();
    }

    Ok(())
}

fn read_file(file_name: &str) -> Result<String, Box<dyn Error>> {
    let file = fs::read_to_string(file_name)?;

    Ok(file)
}