#include <stdio.h>
#include <stdbool.h>

// init player turn variable
bool playerOnesTurn = true;

// init board
// its 3 x 3 ints, we will use 9 as an empty int (nothing placed here yet)
// otherwise 1 and 2 for players 1 (X) and 2 (O / uppercase o)
int board[3][3];
void init_board(int board[3][3])
{
    for (int i = 0; i < 3; i++)
    {
        for (int j = 0; j < 3; j++)
        {
            board[i][j] = 9;
        }
    }
}

void print_board(int board[3][3])
{
    puts("\t1\t2\t3");
    puts("  ------------------------");
    for (int i = 0; i <= 2; i++)
    {
        printf("%d|", i + 1);
        for (int j = 0; j <= 2; j++)
        {
            if (board[i][j] == 9)
            {
                printf("\t.");
            }
            else if (board[i][j] == 1)
            {
                printf("\tX");
            }
            else
            {
                printf("\tO");
            }
        }
        printf("\n");
    }
}

int handle_move(int board[3][3], int moveCoords, bool playerOnesTurn)
{
    // Split the number into two digits
    int y = moveCoords / 10; // Get the tens place
    int x = moveCoords % 10; // Get the ones place

    if (x > 3 | y > 3)
    {
        return -1; // invalid move (out of board grid)
    }
    else
    {
        x--; // -1 from each move so that its 0, 1, or 2 instead of 1, 2, or 3
        y--; // this way it will work properly with the board indexes
    }

    if (board[x][y] != 9)
    {
        return -1; // invalid move (square has already been taken)
    }
    else
    {
        board[x][y] = playerOnesTurn ? 1 : 2;
        return 0;
    }
}

// returns the winning player, -1 if there is a draw, 9 if there is no outcome yet
int isWinner(int board[3][3])
{

    for (int i = 0; i < 3; i++)
    {
        // check if there are any rows that are the same
        if (board[i][0] != 9 & board[i][0] == board[i][1] & board[i][1] == board[i][2])
        {
            return board[i][0];
        }
    }
    for (int i = 0; i < 3; i++)
    {
        // check if there any columns that are the same
        if (board[0][i] != 9 & board[0][i] == board[1][i] & board[1][i] == board[2][i])
        {
            return board[0][i];
        }
    }
    // finally, check diagonals manually
    if (board[1][1] != 9)
    {
        if (board[0][0] == board[1][1] & board[1][1] == board[2][2])
        {
            return board[0][0];
        }
        else if (board[2][0] == board[1][1] & board[1][1] == board[0][2])
        {
            return board[0][0];
        }
    }

    // edge case: all positions are taken and there is no winner, aka draw
    // if there is ever a nine present then there is a viable move, otherwise
    // we MUST return a result, and since earlier returns returned a winner,
    // we can safely return -1 here (draw outcome)
    bool ninePresent = false;
    for (int i = 0; i < 3; i++)
    {
        for (int j = 0; j < 3; j++)
        {
            if (board[i][j] == 9)
            {
                ninePresent = true;
            }
        }
    }

    return ninePresent ? 9 : -1;
}

// returns 1 if player 1 wins, 2 if player 2 wins
int game_loop(int board[3][3], bool playerOnesTurn)
{
    int gameWon = isWinner(board);
    if (gameWon != 9)
    {
        return gameWon;
    }

    printf("Player %d, make your move. Here is the board: \n", (playerOnesTurn) ? 1 : 2);
    print_board(board);
    puts("Enter your move coordinates as XY (where the top left is 11): ");
    int userMove;
    int holdScanfResult = scanf("%d", &userMove);
    if (handle_move(board, userMove, playerOnesTurn) == -1)
    {
        puts("Invalid move, please re-enter a different move.\n");
        return game_loop(board, playerOnesTurn); // recursion, this is where the loop is :)
    }
    else
    {
        printf("your move was %d\n", userMove);
        print_board(board);
        playerOnesTurn = !playerOnesTurn;        // flip the next players move
        return game_loop(board, playerOnesTurn); // recursion, this is where the loop is also :)
    }
}

int main()
{

    printf("noughts and crosses\n\n");
    puts("setting up the board");
    printf("sending board at %p pointer to the board initialiser\n", board);

    init_board(board);

    int winner = game_loop(board, playerOnesTurn);

    if (winner == 1)
    {
        puts("player 1 wins!");
    }
    else if (winner == 2)
    {
        puts("player 2 wins!");
    }
    else
    {
        puts("draw - no winner :(");
    }

    return 0;
}