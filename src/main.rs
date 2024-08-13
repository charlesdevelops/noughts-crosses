fn generate_board(n: usize, m: usize) -> Vec<Vec<String>> {
    let mut board = Vec::new();  // Create an empty vector for the board
    for _ in 0..n {
        let mut row = Vec::new();  // Create an empty vector for each row
        for _ in 0..m {
            row.push("*".to_string());  // Add the desired value to the row
        }
        board.push(row);  // Add the row to the board
    }
    board  // Return the board
}

fn main() {
    let board = generate_board(3, 3);  // Generate a 3x3 board
    for row in &board {
        println!("{:?}", row);  // Print each row of the board
    }
}
