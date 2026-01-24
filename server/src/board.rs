use rand::Rng;
use std::{fmt, i8, io};

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
        let count = rng.random_range(150..=250);
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
        let neighbors: Vec<(isize, isize)> = self.get_neighboring_cells(x, y);

        // apply rules on conway's game of life
        let alive = self.matrix.contains(&(x, y));
        return if alive {
            neighbors.len() == 2 || neighbors.len() == 3
        } else {
            neighbors.len() == 3
        };
    }

    // returns a vector of coordinate tuples of living cells in the board that neighbor the given cell coordinates
    pub fn get_neighboring_cells(&self, x: isize, y: isize) -> Vec<(isize, isize)> {
        self.matrix
            .iter()
            .filter(|&cell| {
                ((cell.0 - x).abs() <= 1)
                    && ((cell.1 - y).abs() <= 1)
                    && !(cell.0 == x && cell.1 == y)
            })
            .cloned()
            .collect()
    }

    pub fn start(&mut self) {
        let mut is_game_running: bool = self.evolve();
        while is_game_running {
            is_game_running = self.evolve();
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
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

#[cfg(test)]

mod tests {
    use super::*;

    // sample board for testing
    //	0 0 0 0 0
    //  0 1 1 1 0
    //  0 0 1 0 0
    //	0 1 0 1 0
    //	0 0 0 0 0
    // (1, 1), (1, 2), (1, 3), (2, 2), (3, 1), (3, 3)

    fn generate_test_board() -> Board {
        let mut board = Board::new(5, 5);
        board.matrix = vec![(1, 1), (1, 2), (1, 3), (2, 2), (3, 1), (3, 3)];
        return board;
    }

    // neighboring cells of (2, 2) are
    // (1, 1), (1, 2), (1, 3), (2, 1), (2, 3), (3, 1), (3, 2), (3, 3)
    #[test]
    fn test_get_neighboring_cells() {
        let board = generate_test_board();
        assert_eq!(
            board.get_neighboring_cells(2, 2),
            vec![(1, 1), (1, 2), (1, 3), (3, 1), (3, 3)]
        );
        assert_eq!(
            board.get_neighboring_cells(0, 2),
            vec![(1, 1), (1, 2), (1, 3)]
        );
    }

    // newly alive cells: (0,2), (3,2)
    // surviving cells: (1,1), (1,2), (1,3)
    // dying cells: (2, 2), (3,1)
    #[test]
    fn test_evolve_cell() {
        let board = generate_test_board();
        // newly born cells
        assert_eq!(board.evolve_cell(0, 2), true);
        assert_eq!(board.evolve_cell(3, 2), true);
        // surviving cells
        assert_eq!(board.evolve_cell(1, 1), true);
        assert_eq!(board.evolve_cell(1, 2), true);
        assert_eq!(board.evolve_cell(1, 3), true);
        // dying cells
        assert_eq!(board.evolve_cell(2, 2), false);
        assert_eq!(board.evolve_cell(3, 1), false);
        // edge cases
        assert_eq!(board.evolve_cell(0, 0), false);
        assert_eq!(board.evolve_cell(4, 4), false);
        assert_eq!(board.evolve_cell(4, 0), false);
        assert_eq!(board.evolve_cell(0, 4), false);
    }
}
