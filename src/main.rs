mod board;


fn main() {
    
    let game_board = board::init_board();

    game_board.print_board();

}