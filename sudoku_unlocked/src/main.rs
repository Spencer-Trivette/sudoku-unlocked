use rand::seq::SliceRandom;
use std::io;
use std::io::Write;

const SIZE: usize = 9;

fn generate_sudoku_solution() -> [[u8; SIZE]; SIZE] {
    let mut board = [[0; SIZE]; SIZE];
    solve_sudoku(&mut board);
    board
}

fn solve_sudoku(board: &mut [[u8; SIZE]; SIZE]) -> bool {
    if let Some((row, col)) = find_empty_location(board) {
        for num in 1..=SIZE as u8 {
            if is_safe(board, row, col, num) {
                board[row][col] = num;
                if solve_sudoku(board) {
                    return true;
                }
                board[row][col] = 0;
            }
        }
        return false;
    }
    true
}

fn find_empty_location(board: &[[u8; SIZE]; SIZE]) -> Option<(usize, usize)> {
    for i in 0..SIZE {
        for j in 0..SIZE {
            if board[i][j] == 0 {
                return Some((i, j));
            }
        }
    }
    None
}

fn is_safe(board: &[[u8; SIZE]; SIZE], row: usize, col: usize, num: u8) -> bool {
    !used_in_row(board, row, num)
        && !used_in_col(board, col, num)
        && !used_in_box(board, row - row % 3, col - col % 3, num)
}

fn used_in_row(board: &[[u8; SIZE]; SIZE], row: usize, num: u8) -> bool {
    for j in 0..SIZE {
        if board[row][j] == num {
            return true;
        }
    }
    false
}

fn used_in_col(board: &[[u8; SIZE]; SIZE], col: usize, num: u8) -> bool {
    for i in 0..SIZE {
        if board[i][col] == num {
            return true;
        }
    }
    false
}

fn used_in_box(board: &[[u8; SIZE]; SIZE], start_row: usize, start_col: usize, num: u8) -> bool {
    for i in 0..3 {
        for j in 0..3 {
            if board[i + start_row][j + start_col] == num {
                return true;
            }
        }
    }
    false
}

fn remove_cells(board: &mut [[u8; SIZE]; SIZE], num_to_remove: usize) {
    let mut rng = rand::thread_rng();
    for _ in 0..num_to_remove {
        let (mut row, mut col);
        loop {
            row = rng.gen_range(0, SIZE);
            col = rng.gen_range(0, SIZE);
            if board[row][col] != 0 {
                break;
            }
        }
        board[row][col] = 0;
    }
}

fn main() {
    let mut puzzle = generate_sudoku_solution();
    remove_cells(&mut puzzle, 50); // Adjust the number of cells to remove as needed
    print_sudoku(&puzzle);
}

fn print_sudoku(board: &[[u8; SIZE]; SIZE]) {
    for row in board.iter() {
        println!("{:?}", row);
    }
}


// Print to console
fn print_sudoku(board: &[[u8; SIZE]; SIZE]) {
    for row in board.iter() {
        println!("{:?}", row);
    }
}

// Check if the input is correct
fn is_solution_correct(board: &[[u8; SIZE]; SIZE], solution: &[[u8; SIZE]; SIZE]) -> bool {
    for i in 0..SIZE {
        for j in 0..SIZE {
            if board[i][j] != solution[i][j] {
                return false;
            }
        }
    }
    true
}

fn main() {
    let puzzle = generate_sudoku();
    println!("Here's your Sudoku puzzle:");
    print_sudoku(&puzzle);

    println!("Enter your solution row by row (e.g., '123456789' for the first row):");

    let mut solution = [[0; SIZE]; SIZE];
    for i in 0..SIZE {
        print!("Row {}: ", i + 1);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        for (j, c) in input.trim().chars().enumerate() {
            solution[i][j] = c.to_digit(10).unwrap() as u8;
        }
    }

    if is_solution_correct(&puzzle, &solution) {
        println!("Congratulations! Your solution is correct.");
    } else {
        println!("Sorry, your solution is incorrect.");
    }
}

