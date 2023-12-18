use wasm_bindgen::prelude::*;
use rand::Rng;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<bool>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new(width: u32, height: u32) -> Universe {
        let cells = {
            let mut rng = rand::thread_rng();
            (0..width * height)
                .map(|_| rng.gen::<bool>())
                .collect()
        };

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn get_cell(&self, row: u32, col: u32) -> bool {
        let index = (row * self.width + col) as usize;
        self.cells[index]
    }

    pub fn tick(&mut self) {
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

    fn live_neighbor_count(&self, x: u32, y: u32) -> u8 {
        let mut count = 0;
        for delta_y in [self.height - 1, 0, 1].iter().cloned() {
            for delta_x in [self.width - 1, 0, 1].iter().cloned() {
                if delta_y == 0 && delta_x == 0 {
                    continue;
                }

                let neighbor_y = (y + delta_y) % self.height;
                let neighbor_x = (x + delta_x) % self.width;
                count += self.cells[(neighbor_y * self.width + neighbor_x) as usize] as u8;
            }
        }
        count
    }
}
