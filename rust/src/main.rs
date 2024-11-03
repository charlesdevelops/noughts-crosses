use std::io::{self, Write};

// 9 represents unused point, 1 and 2 will represent p1 and p2
fn init_board() -> [[i32; 3]; 3]{
    [[9,9,9],[9,9,9],[9,9,9]]
}

fn print_board(board: [[i32; 3]; 3]){

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

fn game_loop(board: [[i32; 3]; 3], player_ones_move: bool){
    print_board(board);

    println!("Enter your move as XY: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Attempt to parse the input as an integer
    let number: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid integer.");
            return;
        }
    };

    println!("{}", number);
}

fn main() {
    println!("Noughts and crosses, rust edition!\n");

    // init game state variables
    let player_ones_move = true;
    let board: [[i32; 3];3] = init_board();

    game_loop(board, player_ones_move);
}
