use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{self, BufRead, Write};
use std::time::Instant;
use rand::Rng;
use rayon::prelude::*;
use std::env;


#[derive(Debug)]
pub enum Implementation {
    Naive,
    HashSet,
    Parallel,
}

pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<bool>,
    cells_hashset: HashSet<(u32, u32)>,
    implementation: Implementation,
}

impl Universe {
    pub fn new(width: u32, height: u32, implementation: Implementation, seed_path: Option<&String>) -> Universe {
        let (cells, cells_hashset) = match seed_path {
            Some(path) => Universe::initialize_from_file(width, height, path),
            None => Universe::initialize_randomly(width, height),
        };

        Universe {
            width,
            height,
            cells,
            cells_hashset,
            implementation,
        }
    }

    fn initialize_randomly(width: u32, height: u32) -> (Vec<bool>, HashSet<(u32, u32)>) {
        let cells: Vec<bool> = {
            let mut rng = rand::thread_rng();
            (0..width * height)
                .map(|_| rng.gen::<bool>())
                .collect()
        };
        let cells_hashset = Universe::create_hashset(&cells, width, height);

        (cells, cells_hashset)
    }

    fn initialize_from_file(width: u32, height: u32, path: &str) -> (Vec<bool>, HashSet<(u32, u32)>) {
        let file = fs::File::open(path).expect("Failed to open seed file");
        let mut lines = io::BufReader::new(file).lines();

        // First line contains width, height, and iterations
        let first_line = lines.next().unwrap().expect("Failed to read first line");
        let dimensions: Vec<u32> = first_line.split_whitespace()
                                                .map(|number| number.parse().expect("Failed to parse dimensions"))
                                                .collect();

        if dimensions.len() != 3 || dimensions[0] != width || dimensions[1] != height {
            panic!("Provided width and height do not match the file's dimensions.");
        }

        let mut cells = vec![false; (width * height) as usize];

        for (y, line) in lines.enumerate() {
            let line = line.expect("Could not read line");
            for (x, char) in line.chars().enumerate() {
                if x < width as usize && y < height as usize {
                    cells[y * width as usize + x] = char == 'O';
                }
            }
        }

        let cells_hashset = Universe::create_hashset(&cells, width, height);

        (cells, cells_hashset)
    }

    fn create_hashset(cells: &Vec<bool>, width: u32, height: u32) -> HashSet<(u32, u32)> {
        let mut hashset = HashSet::new();
        for y in 0..height {
            for x in 0..width {
                if cells[(y * width + x) as usize] {
                    hashset.insert((x, y));
                }
            }
        }
        hashset
    }

    pub fn game_of_life(&mut self, iterations: u32) {
        let start = Instant::now();
        let next_fn: Box<dyn Fn(&mut Universe)> = match self.implementation {
            Implementation::Naive => Box::new(|u: &mut Universe| u.next_naive()),
            Implementation::HashSet => Box::new(|u: &mut Universe| u.next_hashset()),
            Implementation::Parallel => Box::new(|u: &mut Universe| u.next_parallel()),
        };
    
        for _ in 0..iterations {
            next_fn(self);
            
        }
    
        let duration = start.elapsed();
        println!("{} iterations took {:?} ms using the {:?} implementation", iterations, duration.as_millis(), self.implementation);
    }

    fn next_naive(&mut self) {
        let mut next = vec![false; (self.width * self.height) as usize];
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = (y * self.width + x) as usize;
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count_array(x, y);

                next[idx] = matches!(
                    (cell, live_neighbors),
                    (true, 2) | (true, 3) | (false, 3)
                );
            }
        }
        self.cells = next;
        println!();
    }

    fn next_hashset(&mut self) {
        let mut next = HashSet::new();
        let mut to_check: HashSet<(u32, u32)> = HashSet::new();

        // Populate to_check with all cells that are alive and their neighbors
        for &(x, y) in self.cells_hashset.iter() {
            for delta_y in [self.height - 1, 0, 1].iter().cloned() {
                for delta_x in [self.width - 1, 0, 1].iter().cloned() {
                    if delta_y == 0 && delta_x == 0 {
                        continue;
                    }

                    let neighbor_x = (x + delta_x) % self.width;
                    let neighbor_y = (y + delta_y) % self.height;
                    to_check.insert((neighbor_x, neighbor_y));
                }
            }
            to_check.insert((x, y)); // Include the cell itself to be checked
        }

        // Check each cell in to_check to determine if it should be alive in the next state
        for (x, y) in to_check {
            let live_neighbors = self.live_neighbor_count_hashset(x, y);
            let cell_alive = self.cells_hashset.contains(&(x, y));
            if (cell_alive && (live_neighbors == 2 || live_neighbors == 3)) || (!cell_alive && live_neighbors == 3) {
                next.insert((x, y));
            }
        }

        self.cells_hashset = next;
    }

    fn next_parallel(&mut self) {
        let cells = &self.cells;
        let width = self.width;
        let height = self.height;

        self.cells = (0..height * width)
            .into_par_iter()
            .map(|i| {
                let x = (i % width) as u32;
                let y = (i / width) as u32;
                let cell = cells[i as usize];
                let live_neighbors = self.live_neighbor_count_array(x, y);

                match (cell, live_neighbors) {
                    (true, x) if x < 2 => false,
                    (true, 2) | (true, 3) => true,
                    (true, x) if x > 3 => false,
                    (false, 3) => true,
                    (otherwise, _) => otherwise,
                }
            })
            .collect();
    }

    fn live_neighbor_count_array(&self, x: u32, y: u32) -> u8 {
        let mut count = 0;
        for delta_y in [self.height.wrapping_sub(1), 0, 1] {
            for delta_x in [self.width.wrapping_sub(1), 0, 1] {
                if delta_y == 0 && delta_x == 0 {
                    continue;
                }
    
                let neighbor_x = (x + delta_x) % self.width;
                let neighbor_y = (y + delta_y) % self.height;
                let idx = (neighbor_y * self.width + neighbor_x) as usize;
                if self.cells[idx] {
                    count += 1;
                }
            }
        }
        count
    }

    fn live_neighbor_count_hashset(&self, x: u32, y: u32) -> u8 {
        let mut count = 0;
        for delta_y in [self.height.wrapping_sub(1), 0, 1].iter().cloned() {
            for delta_x in [self.width.wrapping_sub(1), 0, 1].iter().cloned() {
                if delta_y == 0 && delta_x == 0 {
                    continue;
                }
    
                let neighbor_x = (x + delta_x) % self.width;
                let neighbor_y = (y + delta_y) % self.height;
                if self.cells_hashset.contains(&(neighbor_x, neighbor_y)) {
                    count += 1;
                }
            }
        }
        count
    }
}


fn write_state_to_file(universe: &Universe, file_path: &str, iterations: u32) -> io::Result<()> {
    let mut file = File::create(file_path)?;

    // Write width, height, and number of iterations as the first line
    writeln!(file, "{} {} {}", universe.width, universe.height, iterations)?;

    match universe.implementation {
        Implementation::HashSet => {
            for y in 0..universe.height {
                for x in 0..universe.width {
                    let symbol = if universe.cells_hashset.contains(&(x, y)) {
                        'O'
                    } else {
                        '.'
                    };
                    write!(file, "{}", symbol)?;
                }
                writeln!(file)?;
            }
        }
        _ => {
            for y in 0..universe.height {
                for x in 0..universe.width {
                    let idx = (y * universe.width + x) as usize;
                    let symbol = if universe.cells[idx] {
                        'O'
                    } else {
                        '.'
                    };
                    write!(file, "{}", symbol)?;
                }
                writeln!(file)?;
            }
        }
    }

    Ok(())
}



fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 5 {
        println!("Usage: game_of_life <width> <height> <iterations> <implementation> [seed_file (random if empty)]");
        std::process::exit(1);
    }

    let width = args[1].parse::<u32>().unwrap();
    let height = args[2].parse::<u32>().unwrap();
    let iterations = args[3].parse::<u32>().unwrap();
    let implementation = match args[4].as_str() {
        "naive" => Implementation::Naive,
        "hash" => Implementation::HashSet,
        "parallel" => Implementation::Parallel,
        _ => {
            println!("Invalid implementation. Choose from 'naive', 'hash', or 'parallel'.");
            std::process::exit(1);
        }
    };

    let mut total_iterations = iterations;
    if let Some(seed_path) = args.get(5) {
        let seed_file_content = fs::read_to_string(seed_path).unwrap();
        let first_line = seed_file_content.lines().next().unwrap();
        let previous_iterations: u32 = first_line.split_whitespace().nth(2).unwrap().parse().unwrap();
        total_iterations += previous_iterations;
    }

    let mut universe = Universe::new(width, height, implementation, args.get(5));
    universe.game_of_life(iterations);
    let path = format!("game_of_life_{}_{}_{}.txt", width, height, total_iterations);
    write_state_to_file(&universe, &path, total_iterations).unwrap();
}
