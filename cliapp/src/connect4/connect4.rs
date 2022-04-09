#[derive(Copy, Clone)]
pub struct Connect4 {
    pub board: [[u8; NUM_COLS]; NUM_ROWS],
    pub col_height: [usize; NUM_COLS],
}

pub const NUM_COLS: usize = 7;
pub const NUM_ROWS: usize = 6; // also max height

impl Connect4 {
    // initialize empty board
    pub fn new() -> Self {
        Connect4 {
            board: [[0; NUM_COLS]; NUM_ROWS],
            col_height: [0; NUM_COLS],
        }
    }

    // inserts game piece in some column
    pub fn insert(&mut self, col: usize, player: u8) -> bool {
        if NUM_ROWS == self.col_height[col] { // check if col is full
            return false;
        } // col is not full
        let row = NUM_ROWS - self.col_height[col] - 1;
        self.board[row][col] = player; // insert piece
        self.col_height[col] += 1;
        return true;
    }

    // takes in the last placed piece as an input and checks for a win
    // return (0 if game is still playing), (1 if player has won), (-1 if draw)
    pub fn check_win_draw(&mut self, col: usize, player: u8) -> i8 {
        if col >= NUM_COLS { // check valid input
            return 0;
        }
        // check draw
        let mut counter = 0;
        for i in 0..NUM_COLS { // check if all columns are full
            if self.col_height[i] == NUM_ROWS {
                counter+=1;
            }
        }
        if counter == NUM_COLS {
            return -1; // draw
        }

        let row = NUM_ROWS - self.col_height[col];
        let mut counter = 0;
        // check across rows (vertical)
        for i in 0..NUM_ROWS {
            if self.board[i][col] == player {
                counter += 1;
            } else {
                counter = 0;
            }
            if counter == 4 {
                return 1;
            }
        }
        counter = 0;
        // check across cols (horizontal)
        for i in 0..NUM_COLS {
            if self.board[row][i] == player {
                counter += 1;
            } else {
                counter = 0;
            }
            if counter == 4 {
                return 1;
            }
        }
        counter = 0;
        // check diagonals
        // pos slope
        let mut start_row = row;
        let mut start_col = col;
        while start_row < 3 || start_col < 3 {
            start_row += 1;
            start_col += 1;
        }
        start_row -= 3;
        start_col -= 3;
        let mut j = start_row;
        for i in start_col..NUM_COLS {
            if j < NUM_ROWS {
                if self.board[j][i] == player {
                    counter += 1;
                } else {
                    counter = 0;
                }
                if counter == 4 {
                    return 1;
                }
                j+=1;
            }
        }
        counter = 0;
        // neg slope
        start_row = row+3;
        start_col = col;
        while start_row > 5 || start_col < 3 {
            start_row -= 1;
            start_col += 1;
        }
        start_col -= 3;
        j = start_row;
        for i in start_col..NUM_COLS {
            if self.board[j][i] == player {
                counter += 1;
            } else {
                counter = 0;
            }
            if counter == 4 {
                return 1;
            }
            if j > 0 {
                j -= 1;
            } else {
                break;
            }
        }
        return 0;
    }
}