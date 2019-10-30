use std::fs;

mod board;

type Point = (usize, usize);

fn main() {

    let filepath = "src/input.txt".to_string();
    let mut game_board = board::Board{matrix: init_board_from_file(filepath)};
    
    println!("{}", game_board);
    game_board.evolve();
    println!("{}", game_board);



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
    vec
}

/*
 * init_board()
 * Initializes a new empty sparse array and returns it
 */
fn init_board() -> Vec<Point> {
    Vec::new()
}   





















