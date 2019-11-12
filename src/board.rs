use std::fmt;
use std::cmp;

const HEIGHT: usize = 10;
const WIDTH: usize = 10;

pub struct Board {
	pub matrix: Vec<(usize, usize)>
}

impl Board {
	// returns 0 if game is over
	pub fn evolve(&mut self) -> usize {
		let mut new_matrix: Vec<(usize, usize)> = Vec::new();
		let possible_cells = &self.get_possible_cells();
		// println!("Possible cells: {:?}", possible_cells);
		// println!("Length: {}", possible_cells.len());
		for cell in possible_cells {
			// println!("{}, {}", cell.0, cell.1);
			let neighbors = &self.get_neighbors(&cell);
			// println!("{:?}", neighbors);
			// println!("{}", &self);
			let intersection = get_intersection(&self.get_neighbors(&cell), &self.matrix);
			// println!("{:?}", intersection);
			let len = intersection.len();
			// println!("{}", len);
			if (&self.matrix).into_iter().any(|x| x.0 == cell.0 && x.1 == cell.1) {
				// println!("Living!");
				// we have a living cell here
				if len == 2 || len == 3 {
					// live on
					// println!("Live on!");
					new_matrix.push((cell.0, cell.1));
				}
			}
			else { 
				// println!("Dead!");
				// means dead cell
				if len == 3 {
					// birth
					// println!("Birth!");
					new_matrix.push((cell.0, cell.1));
				}
			}
		}
		self.matrix = new_matrix;
		self.matrix.len()

	}

	// returns vector of union of all immediate neighbors of all active cells
	fn get_possible_cells(&self) -> Vec<(usize, usize)> {
		let mut possible_cells: Vec<(usize, usize)> = Vec::new();
		for cell in &self.matrix {
			let neighbors = &self.get_neighbors(&cell).to_owned();
			possible_cells.push((cell.0, cell.1));
			possible_cells = get_union(&possible_cells, neighbors);
			// println!("{:?}", possible_cells);
			// need to create list of possible cells which includes all neighbors and all living cells
			// iterate through that list and check for overlap of those neighbors with original board
			// that will result in correct shit

		}
		// return created vector
		possible_cells
	}
	
	// Does NOT include self
	fn get_neighbors(&self, cell: &(usize, usize)) -> Vec<(usize, usize)> {
		let mut neighbors: Vec<(usize, usize)> = Vec::new();
			// println!("{:?}", cell);
			for x in (cmp::max(0, (cell.0 as isize) - 1)as usize)..cmp::min(WIDTH, cell.0 + 2) { 
				for y in (cmp::max(0, (cell.1 as isize) - 1) as usize)..cmp::min(HEIGHT, cell.1 + 2) {
					if cell.0 != x || cell.1 != y {
						neighbors.push((x, y));
					}
				}
			}
		neighbors
	}
	
}

// helper

fn get_union(vec1: &Vec<(usize, usize)>, vec2: &Vec<(usize, usize)>) -> Vec<(usize, usize)> { // this should work?
		let mut union: Vec<(usize, usize)> = Vec::new();
		// TODO: check which vec is smaller
		// Also make this not suck
		// println!("Vec1: {:?}\nVec2: {:?}", vec1, vec2);
		if vec2.len() > vec1.len() {
			return get_union(vec2, vec1);
		}
		else {
			for cell in vec1 {
				if union.contains(cell) == false { // TODO: contains() is by reference not value
					union.push((cell.0, cell.1));
					// println!("Pushing cell: {}, {}", cell.0, cell.1);
				}
			}
			for cell in vec2 {
				if union.contains(cell) == false {
					// println!("Pushing cell: {}, {}", cell.0, cell.1);c
					union.push((cell.0, cell.1));
				}
			}
		}
		

		union
	}

fn get_intersection(vec1: &Vec<(usize, usize)>, vec2: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
	let mut intersection: Vec<(usize, usize)> = Vec::new();

	if vec2.len() > vec1.len() {
		return get_intersection(vec2, vec1);
	}
	else {
		for cell in vec1 {
			if vec2.into_iter().any(|x| x.0 == cell.0 && x.1 == cell.1) && (intersection.contains(cell) == false) {
				intersection.push((cell.0, cell.1));
			}
		}
		for cell in vec2 {
			if vec1.into_iter().any(|x| x.0 == cell.0 && x.1 == cell.1) && (intersection.contains(cell) == false) {
				intersection.push((cell.0, cell.1));
			}
		}

	}
	intersection
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











