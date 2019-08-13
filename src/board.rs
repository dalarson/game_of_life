const HEIGHT: usize = 20;
const WIDTH: usize = 40;

pub struct Board {
    game_board: [[u8; WIDTH]; HEIGHT]
}

impl Board {

    pub fn print_board(&self){
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if self.game_board[y][x] != 0 { print!("X"); }
                else { print!(" "); }
            }
            println!();
        }
    }

    fn len(&self) -> usize{
        return self.game_board.len();
    }
}

pub fn init_board() -> Board {
    let new_board = Board{
        game_board: [
            [1; WIDTH],
            [0; WIDTH],
            [1; WIDTH],
            [0; WIDTH],
            [1; WIDTH],
            [0; WIDTH],
            [1; WIDTH],
            [0; WIDTH],
            [1; WIDTH],
            [0; WIDTH],
            [1; WIDTH],
            [0; WIDTH],
            [1; WIDTH],
            [0; WIDTH],
            [1; WIDTH],
            [0; WIDTH],
            [1; WIDTH],
            [0; WIDTH],
            [1; WIDTH],
            [0; WIDTH]
        ]};
    return new_board;
}








