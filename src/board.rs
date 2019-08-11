const HEIGHT: usize = 10;
const WIDTH: usize = 10;

pub struct Board {
    game_board: [[u8; HEIGHT]; WIDTH]
}

impl Board {

    pub fn print_board(&self){
        for (i, row) in self.game_board.iter().enumerate() {
            println!("{:?}", self.game_board[i]);
        }
    }

    fn len(&self) -> usize{
        return self.game_board.len();
    }
}

pub fn init_board() -> Board {
        let new_board = Board{
            game_board: [[0; HEIGHT]; WIDTH]
        };
        return new_board;
    }







