mod board;

fn main() {
    let mut board = board::Board::new(20, 20);
    board.start();
}
