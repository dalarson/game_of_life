
const HEIGHT: usize = 7;
const WIDTH: usize = 10;

pub struct Board {
	pub matrix: Vec<(usize, usize)>
}

impl Board {

	pub fn evolve(&self) {
		for point in &self.matrix {
        	println!("{:?}", point); // need to accumulate neighbors of all points
   		}
	}

	pub fn print(&self) {
		let mut blank_board: [[u8; HEIGHT]; WIDTH] = [[0; HEIGHT]; WIDTH];
	    for i in &self.matrix {
	        blank_board[i.0][i.1] = 1;
	    }
	    for i in 0..HEIGHT {
	        for j in 0..WIDTH {
	            print!("{}", if blank_board[j][i] == 0 {"O"} else {"X"});
	        }
	        print!("\n");
	    }
	}


}