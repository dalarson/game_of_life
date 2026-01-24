mod board;

fn main() {
    let mut board = board::Board::new(30, 30);
    board.start();
}
