# we need to set up some variables to store data about the game first
winner = "None"
current_player = "X"
board = [
    ["-", "-", "-",],
    ["-", "-", "-",],
    ["-", "-", "-",],
    ]

# prints the current board variable out
def print_board():
    for line in board:
        print(line)

# this will return true if there is a winner, and will store the winer in the `winner` variable
def is_winner():
    # checking every horizontal line
    for line in board:
        if line[0] == line[1] == line[2] != "-":
            return True
    # checking each diagonal (00, 11, 22) and (02, 11, 20)
    if board[0][0] == board[1][1] == board[2][2] != "-":
        return True
    if board[0][2] == board[1][1] == board[2][0] != "-":
        return True
    # checking every vertical line
    for i in range(3):
        if board[0][i] == board[1][i] == board[2][i] != "-":
            return True
    return False

while winner == "None":
    # print the board
    print_board()
    # get the user's input
    user_input = input("Enter the coordinates of your move (row, column): ")
    # split the input into a list
    user_input = user_input.split(",")
    # convert the input to integers
    user_input = [int(i) for i in user_input]
    # check if the move is valid
    if board[user_input[0]][user_input[1]] == "-":
        board[user_input[0]][user_input[1]] = current_player
    else:
        print("Invalid move!")
        continue
    # check if there is a winner
    if is_winner():
        winner = current_player
        break
    # switch the player
    if current_player == "X":
        current_player = "O"
    else:
        current_player = "X"
# print the winner
print(winner + " won!")
for line in board:
    print(line)