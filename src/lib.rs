use std::collections::HashSet;
use wasm_timer::Instant;
use wasm_bindgen::prelude::*;
use rand::Rng;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(Debug)]
pub enum Implementation {
    Naive,
    HashSet,
    Parallel,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<bool>,
    cells_hashset: HashSet<(u32, u32)>,
    implementation: Implementation,
}


#[wasm_bindgen]
impl Universe {
    pub fn new(width: u32, height: u32, implementation: Implementation) -> Universe {
        wasm_logger::init(wasm_logger::Config::default());
        log::info!("Using {:?} implementation", implementation);
        let cells: Vec<bool> = {
            let mut rng = rand::thread_rng();
            (0..width * height)
                .map(|_| rng.gen::<bool>())
                .collect()
        };
        let cells_hashset = {
            let mut hashset = HashSet::new();
            for y in 0..height {
                for x in 0..width {
                    if cells[(y * width + x) as usize] {
                        hashset.insert((x, y));
                    }
                }
            }
            hashset
        };

        Universe {
            width,
            height,
            cells,
            cells_hashset,
            implementation,
        }
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn get_cell(&self, row: u32, col: u32) -> bool {
        match self.implementation {
            Implementation::Naive => self.get_cell_naive(row, col),
            Implementation::HashSet => self.get_cell_hashset(row, col),
            Implementation::Parallel => self.get_cell_parallel(row, col),
        }
    }

    pub fn get_cell_naive(&self, row: u32, col: u32) -> bool {
        self.cells[(row * self.width + col) as usize]
    }

    pub fn get_cell_hashset(&self, row: u32, col: u32) -> bool {
        self.cells_hashset.contains(&(row, col))
    }

    pub fn get_cell_parallel(&self, _row: u32, _col: u32) -> bool {
        // ... parallelized computation using rayon ...
        false
    }
    
    pub fn tick(&mut self) {
        let start = Instant::now();
        match self.implementation {
            Implementation::Naive => self.tick_naive(),
            Implementation::HashSet => self.tick_hashset(),
            Implementation::Parallel => self.tick_parallel(),
        }
        let duration = start.elapsed();
        log::info!("Tick took {} milliseconds", duration.as_millis());
    }

    pub fn tick_naive(&mut self) {
        let mut next = self.cells.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = (y * self.width + x) as usize;
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(x, y);

                next[idx] = match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (true, x) if x < 2 => false,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (true, 2) | (true, 3) => true,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (true, x) if x > 3 => false,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (false, 3) => true,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };
            }
        }

        self.cells = next;
    }

    fn tick_hashset(&mut self) {
        let mut next = HashSet::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let live_neighbors = self.live_neighbor_count(x, y);

                let cell = self.cells_hashset.contains(&(x, y));
                if (cell && (live_neighbors == 2 || live_neighbors == 3))
                    || (!cell && live_neighbors == 3)
                {
                    next.insert((x, y));
                }
            }
        }

        self.cells_hashset = next;
    }

    fn tick_parallel(&mut self) {
        // ... parallelized computation using rayon ...
        // TODO: implement
    }

    fn live_neighbor_count(&self, x: u32, y: u32) -> u8 {
        let mut count = 0;
        for delta_y in [self.height - 1, 0, 1].iter().cloned() {
            for delta_x in [self.width - 1, 0, 1].iter().cloned() {
                if delta_y == 0 && delta_x == 0 {
                    continue;
                }
    
                let neighbor_y = (y + delta_y) % self.height;
                let neighbor_x = (x + delta_x) % self.width;
    
                match self.implementation {
                    Implementation::Naive | Implementation::Parallel => {
                        count += self.cells[(neighbor_y * self.width + neighbor_x) as usize] as u8;
                    },
                    Implementation::HashSet => {
                        if self.cells_hashset.contains(&(neighbor_x, neighbor_y)) {
                            count += 1;
                        }
                    },
                }
            }
        }
        count
    }
}
