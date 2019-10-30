
use std::fmt;

const HEIGHT: usize = 7;
const WIDTH: usize = 10;

pub struct Board {
	pub matrix: Vec<(usize, usize)>
}

impl Board {

	pub fn evolve(&mut self) {
		for point in &self.matrix {

   		}
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











