fn init_board() -> [[i32; 3]; 3]{
    [[9,9,9],[9,9,9],[9,9,9]]
}

fn print_board(board: [[i32; 3]; 3]){
    for i in 0..3 {
        println!("{:?}", board[i]);
    }
}

fn game_loop(board: [[i32; 3]; 3], player_ones_move: bool){
    print_board(board);
}

fn main() {
    println!("Noughts and crosses, rust edition!\n");
    let player_ones_move = true;
    let board: [[i32; 3];3] = init_board();
    game_loop(board, player_ones_move);
}
