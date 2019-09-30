const HEIGHT: usize = 100;
const WIDTH: usize = 100;


fn main() {
    
    let game_board = init_board();

    print_board(game_board);

}

fn init_board() -> Vec<(usize, usize)> {
    let vec = Vec::new();
    
    return vec;
}   

fn print_board(board: Vec<(usize, usize)>) {
    let mut blank_board: [[u8; HEIGHT]; WIDTH] = [[0; HEIGHT]; WIDTH];
    for i in board {
        blank_board[i.0][i.1] = 1;
    }
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            print!("{}", if blank_board[i][j] == 0 {" "} else {"X"});
        }
        print!("\n");
    }

}