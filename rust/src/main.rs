fn init_board() -> [[i32; 3]; 3]{
    [[9,9,9],[9,9,9],[9,9,9]]
}

fn print_board(board: [[i32; 3]; 3]){

    println!("\t1\t2\t3");
    println!(" {}", "-".repeat(25));

    for i in 0..3 {
        print!("{}|\t", i + 1);
        for j in board[i]{
            print!("{}\t", j);
        }
        println!();
    }
}

fn game_loop(board: [[i32; 3]; 3], player_ones_move: bool){
    print_board(board);
}

fn main() {
    println!("Noughts and crosses, rust edition!\n");

    // init game state variables
    let player_ones_move = true;
    let board: [[i32; 3];3] = init_board();

    game_loop(board, player_ones_move);
}
