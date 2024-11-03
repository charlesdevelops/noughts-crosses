use std::io::{self};
use std::{process};

// 9 represents unused point, 1 and 2 will represent p1 and p2
fn init_board() -> [[i32; 3]; 3]{
    [[9,9,9],[9,9,9],[9,9,9]]
}

fn get_move() -> [i32; 2] {
    println!("Enter your move as XY: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Attempt to parse the input as a pair of ints
    // Returns an array if parsing is successful

    let current_nums: [i32; 2] = match input.trim().parse::<i32>() {
        Ok(num) => [(num % 10) - 1, (num / 10) - 1],
        Err(_) => {
            println!("Please enter a valid integer without commas.");
            get_move() // Retry if parsing fails
        }
    };

    // checking >2 here as at this point the 33 input has already been decremented so about the range is 0, 1, 2
    if (current_nums[0] > 2) || (current_nums[1] > 2) || (current_nums[0] < 0) || (current_nums[1] < 0) {
        println!("Move is out of bounds. Enter a valid move (XY as 11 - 33 inclusive).");
        get_move() // retry if out of bounds
    } else {
        current_nums
    }
}

fn handle_move(board: &mut [[i32; 3]; 3], coords: [i32; 2], player_ones_move: &mut bool) -> bool {
    match board[coords[0] as usize][coords[1] as usize] {
        9 => {
            board[coords[0] as usize][coords[1] as usize] = if *player_ones_move { 1 } else { 2 }; // Modify the board
            true // Return true if the modification succeeds
        },
        _ => false // Return false if the cell isn't 9
    }
}

fn print_board(board: &mut [[i32; 3]; 3]){

    println!("\t1\t2\t3");
    println!(" {}", "-".repeat(25));

    for i in 0..3 {
        print!("{}|\t", i + 1);
        for j in board[i]{
            match j{
                1=>print!("X\t"),
                2=>print!("O\t"),
                _=>print!("-\t")
            }
        }
        println!();
    }
}

fn is_winner(board: &mut [[i32; 3]; 3]) -> bool {
    for i in 0..3 {
        if board[i][0] != 9 && board[i][0] == board[i][1] && board[i][1] == board[i][2] {
            return true;
        } else if board[0][i] != 9 && board[0][i] == board[1][i] && board[1][i] == board[2][i] {
            return true;
        }
    }
    if board[1][1] != 9 {
        if board[0][0] == board[1][1] && board[1][1] == board[2][2]{
            return true;
        } else if board[2][0] == board[1][1] && board[1][1] == board[0][2] {
            return true;
        }
    }

    return false;
}

fn is_deadlock(board: &mut [[i32; 3]; 3]) -> bool {
    for i in 0..3 {
        for j in 0..3 {
            if board[i][j] == 9 {
                return false;
            }
        }
    }
    return true;
}

fn game_loop(board: &mut [[i32; 3]; 3], player_ones_move: &mut bool){

    print_board(board);
    println!(
        "Player {}'s turn!",
        if *player_ones_move { 1 } else { 2 }
    );

    let coords: [i32; 2] = get_move();

    if handle_move(board, coords, player_ones_move) {
        if is_winner(board) {
            println!("\nCONGRATULATIONS!! Player {} wins!\n", if *player_ones_move { 1 } else { 2 });
            println!("Here is the final board:");
            print_board(board);
            process::exit(0);
        } else if is_deadlock(board) {
            println!("Draw - nobody wins :(\n");
            println!("Here is the final board:");
            print_board(board);
            process::exit(0);
        }

        // If no winner and no deadlock, then continue the game loop
        if *player_ones_move {
            *player_ones_move = false;
        } else {
            *player_ones_move = true;
        }
        game_loop(board, player_ones_move);
    } else {
        println!("Move has already been taken, try a different square.");
        game_loop(board, player_ones_move);
    }
}

fn main() {
    println!("Noughts and crosses, rust edition!\n");

    // init game state variables
    let mut player_ones_move = true;
    let mut board: [[i32; 3]; 3] = init_board();

    game_loop(&mut board, &mut player_ones_move);
}
