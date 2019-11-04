use std::fmt;
use std::cmp;

const HEIGHT: usize = 10;
const WIDTH: usize = 10;

pub struct Board {
	pub matrix: Vec<(usize, usize)>
}

impl Board {

	pub fn evolve(&mut self) {
		let possible_cells = &self.get_possible_cells();
	}

	// returns vector of union of all immediate neighbors of all active cells
	fn get_possible_cells(&self) -> Vec<(u8, u8)> {
		let mut possible_cells = Vec::new();
		for cell in &self.matrix {
			let mut neighbors = Vec::new();
			println!("{:?}", cell);
			for x in (cmp::max(0, (cell.0 as isize) - 1)as usize)..cmp::min(WIDTH, cell.0 + 2) { // can't underflow a usize
				for y in (cmp::max(0, (cell.1 as isize) - 1) as usize)..cmp::min(HEIGHT, cell.1 + 2) {
					if x != cell.0 || y != cell.1 { 
						neighbors.push((x, y));
						println!("{}, {}", x, y)
					}
				}
			}
			let living_neighbors: u8 = self.get_intersection_cardinality(&neighbors);
			// need to create list of possible cells which includes all neighbors and all living cells
			// iterate through that list and check for overlap of those neighbors with original board
			// that will result in correct shit
			// yunk

		}
		// return created vector
		possible_cells
	}

	fn get_intersection_cardinality(&self, neighbors: &Vec<(usize, usize)>) -> u8 {
		let mut count: u8 = 0;
		// TODO: check which vec is smaller
		for cell in &self.matrix {
			if neighbors.contains(cell) {
				count += 1;
			}
		}
		count
	}
	
}

impl fmt::Display for Board {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {	
		let mut blank_board: [[u8; HEIGHT]; WIDTH] = [[0; HEIGHT]; WIDTH];
	    for i in &self.matrix {
	        blank_board[i.0][i.1] = 1;
	    }
	    for i in 0..HEIGHT {
	        for j in 0..WIDTH {
	            write!(f, "{}", if blank_board[j][i] == 0 {" "} else {"X"})?;
	        }
	        write!(f, "\n")?;
	    }
	    Ok(())
	}
}











