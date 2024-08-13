package main
import (
	"fmt"
	"strconv"
)

func printBoard(board [3][3]int){
	for rowId := 0; rowId < len(board); rowId++ {
		rowtxt := ""
		for colId := 0; colId < len(board[rowId]); colId++ {
			if board[rowId][colId] == 0 {
				rowtxt += "- "
			} else if board[rowId][colId] == 1 {
				rowtxt += "X "
			} else {
				rowtxt += "O "
			}
		}
		fmt.Println(rowtxt)
	}
}

func isWinner(board [3][3]int) bool {
    // checking each row
    for line := 0; line < len(board); line++ {
        if board[line][0] == board[line][1] && board[line][1] == board[line][2] && board[line][0] != 0 {
            return true
        }
    }

    // checking each diagonal (00, 11, 22) and (02, 11, 20)
    if board[0][0] == board[1][1] && board[1][1] == board[2][2] && board[0][0] != 0 {
        return true
    }
    if board[0][2] == board[1][1] && board[1][1] == board[2][0] && board[0][2] != 0 {
        return true
    }

    // checking every vertical line
    for i := 0; i < len(board); i++ {
        if board[0][i] == board[1][i] && board[1][i] == board[2][i] && board[0][i] != 0 {
            return true
        }
    }

    return false
}

func makeMove(board [3][3]int, current_player int) [3][3]int {

	if current_player == 1 {
		fmt.Println("Player X, enter your move coordinates as x,y: ")
	} else {
		fmt.Println("Player O, enter your move coordinates as x,y: ")
	}

	var move string
	fmt.Scan(&move)

	xVal, xErr := strconv.Atoi(string(move[0]))
	yVal, yErr := strconv.Atoi(string(move[2]))

	if xErr != nil || yErr != nil {
		fmt.Println("invalid move")
		return makeMove(board, current_player)
	}

	if board[xVal][yVal] == 0 {
		board[xVal][yVal] = current_player
		return board
	} else {
		fmt.Println("invalid move")
		return makeMove(board, current_player)
	}
}

func boardFull(board [3][3]int) bool {

	isFull := true
	// return false if there is a single non empty coord as board not full
    for row := 0; row < len(board); row++ {
    	for col := 0; col < len(board[row]); col++ {
    		if board[row][col] == 0 {
    			isFull = false
    		}
    	}
    }
    return isFull
}


func main() {

	// init variables
	current_player := 1
	board := [3][3]int{{0,0,0},{0,0,0},{0,0,0}}
	winner := 0

	// game loop
	for winner == 0 {

		// if the board is full exit game loop
		if boardFull(board) {
			break
		}

		printBoard(board)
		board = makeMove(board, current_player)

		if isWinner(board) {
			winner = current_player
		} else {
			if current_player == 1 {
				current_player = 2
			} else {
				current_player = 1
			}
		}

	}

	// display winner and board
	if winner == 1 {
		fmt.Println("X wins!")
	} else if winner == 2 {
		fmt.Println("O wins!")
	} else {
		fmt.Println("Draw - no winners!")
	}

	printBoard(board)
}