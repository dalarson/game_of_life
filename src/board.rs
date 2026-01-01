use rand::Rng;
use std::{fmt, i8};

pub struct Board {
    width: isize,
    height: isize,
    // this is a vector of tuples representing coordinates on the board
    matrix: Vec<(isize, isize)>,
    age: i128,
}

impl Board {
    pub fn new(width: isize, height: isize) -> Self {
        // initialize a random vector of initial points
        let mut rng = rand::rng();
        let count = rng.random_range(5..=30);
        let initial_matrix = (0..count)
            .map(|_| {
                let x = rng.random_range(0..width as i8) as isize;
                let y = rng.random_range(0..height as i8) as isize;
                (x, y)
            })
            .collect();

        // create board object
        Self {
            width,
            height,
            matrix: initial_matrix,
            age: 0,
        }
    }

    // run a single generation of the board and replace the state of the game board.
    // returns false if the board is empty after evolution, true otherwise
    pub fn evolve(&mut self) -> bool {
        // print game board
        println!("Generation {}:\n{}", self.age, self);
        // create new board
        let mut new_matrix = Vec::new();
        for x in 0..self.width {
            for y in 0..self.height {
                if self.evolve_cell(x, y) {
                    new_matrix.push((x, y));
                }
            }
        }
        self.matrix = new_matrix;
        self.age += 1;
        if self.matrix.is_empty() {
            return false;
        } else {
            return true;
        }
    }

    // return whether the cell at (x, y) will be alive or dead in the next generation
    pub fn evolve_cell(&self, x: isize, y: isize) -> bool {
        // get array of neighboring alive cells
        let neighbors: Vec<(isize, isize)> = self
            .matrix
            .iter()
            .filter(|&cell| ((cell.0 - x).abs() <= 1) || (cell.1 - y).abs() <= 1)
            .cloned()
            .collect();

        // apply rules on conway's game of life
        let alive = self.matrix.contains(&(x, y));
        return if alive {
            neighbors.len() == 2 || neighbors.len() == 3
        } else {
            neighbors.len() == 3
        };
    }

    pub fn start(&mut self) {
        let mut is_game_running: bool = self.evolve();
        while is_game_running {
            is_game_running = self.evolve();
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // populate active cells
        let mut board_matrix = vec![vec![0; self.width as usize]; self.height as usize];
        for point in self.matrix.iter() {
            board_matrix[point.0 as usize][point.1 as usize] = 1;
        }

        // print the board
        for row in board_matrix.iter() {
            for cell in row.iter() {
                let symbol = if *cell == 1 { '█' } else { ' ' };
                write!(f, "{}", symbol)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
