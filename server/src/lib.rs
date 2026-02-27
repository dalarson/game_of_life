use wasm_bindgen::prelude::*;
use rand::random;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    next_cells: Vec<Cell>, // Secondary buffer
}

#[wasm_bindgen]
impl Universe {
    pub fn new(width: u32, height: u32) -> Universe {
        let size = (width * height) as usize;
        let cells = (0..size)
            .map(|_i| {
                if random::<bool>() {
                     Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        let next_cells = vec![Cell::Dead; size];

        Universe { width, height, cells, next_cells }
    }

    pub fn new_with_cells(width: u32, height: u32, initial_cells: &[u8]) -> Universe {
        let size = (width * height) as usize;
        let cells: Vec<Cell> = (0..size)
            .map(|i| {
                if i < initial_cells.len() && initial_cells[i] != 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        let next_cells = vec![Cell::Dead; size];

        Universe { width, height, cells, next_cells }
    }

    pub fn tick(&mut self) {
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let live_neighbors = self.live_neighbor_count(row, col);

                self.next_cells[idx] = match (self.cells[idx], live_neighbors) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };
            }
        }
        // Swap buffers to avoid allocation
        std::mem::swap(&mut self.cells, &mut self.next_cells);
    }

    // Returns a pointer to the start of the cells vector in WASM memory
    pub fn cells_ptr(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    // Get the number of living neighbors for cell at (row, column)
    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }
                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
}