use std::fs;

const HEIGHT: usize = 7;
const WIDTH: usize = 10;
type Point = (usize, usize);


fn main() {
    let filepath = "src/input.txt".to_string();
    let game_board = init_board_from_file(filepath);
    print!("{:?}", game_board);
    print_board(game_board);


}

/*
 * init_board_from_file
 */

fn init_board_from_file(filepath: String) -> Vec<Point> {
    let mut vec = init_board(); // get new sparse matrix instance
    let contents = fs::read_to_string(filepath).expect("File not found!");

    // maintain coordinates to populate sparse matrix
    let mut x: usize = 0;
    let mut y: usize = 0;

    for c in contents.chars() {
        if c == '\n' { // new row
            y += 1;
            x = 0;
        }
        else if c == '1' {vec.push((x, y));} // add point to sparse matrix
        x += 1;
    }

    // return newly created sparce matrix
    return vec;
}

/*
 * init_board()
 * Initializes a new empty sparse array and returns it
 */
fn init_board() -> Vec<Point> {
    let vec = Vec::new();
    return vec;
}   

fn print_board(board: Vec<Point>) {
    let mut blank_board: [[u8; HEIGHT]; WIDTH] = [[0; HEIGHT]; WIDTH];
    for i in board {
        blank_board[i.0][i.1] = 1;
    }
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            print!("{}", if blank_board[j][i] == 0 {" "} else {"X"});
        }
        print!("\n");
    }

}