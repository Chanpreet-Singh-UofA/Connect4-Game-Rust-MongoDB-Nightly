#[derive(Copy, Clone)]
pub struct TootOtto {
    pub board: [[u8; NUM_COLS]; NUM_ROWS],
    pub col_height: [usize; NUM_COLS],
}

pub const NUM_COLS: usize = 7;
pub const NUM_ROWS: usize = 6; // also max height

impl TootOtto {
    // initialize empty board
    pub fn new() -> Self {
        TootOtto {
            board: [[0; NUM_COLS]; NUM_ROWS],
            col_height: [0; NUM_COLS],
        }
    }

    // inserts game piece in some column
    pub fn insert(&mut self, col: usize, piece: u8) -> bool {
        if NUM_ROWS == self.col_height[col] { // check if col is full
            return false;
        } // col is not full
        let row = NUM_ROWS - self.col_height[col] - 1;
        self.board[row][col] = piece; // insert piece
        self.col_height[col] += 1;
        return true;
    }

    // takes in the last placed piece as an input and checks for a win
    // return (0 if game is still playing), (1 if player has won), (-1 if draw) (2 for player 2 win) 
    pub fn check_win_draw(&mut self, col: usize) -> i8 {
        if col >= NUM_COLS { // check valid input
            return 0;
        }
        // check draw
        {
            let mut counter = 0;
            for i in 0..NUM_COLS { // check if all columns are full
                if self.col_height[i] == NUM_ROWS {
                    counter+=1;
                }
            }
            if counter == NUM_COLS {
                return -1; // draw
            }
        }
        let mut p1w = false;
        let mut p2w = false;
        let cond1: [u8;4] = [1,2,2,1]; // TOOT
        let cond2: [u8;4] = [2,1,1,2]; // OTTO

        let row = NUM_ROWS - self.col_height[col];
        let mut counter1 = 0;
        let mut counter2 = 0;
        // check across rows (vertical)
        for i in 0..NUM_ROWS {
            if self.board[i][col] == cond1[counter1] {
                counter1 += 1;
            } else if self.board[i][col] == cond1[0] {
                counter1 = 1;
            } else {
                counter1 = 0;
            }
            if self.board[i][col] == cond2[counter2] {
                counter2 += 1;
            } else if self.board[i][col] == cond2[0] {
                counter2 = 1;
            } else {
                counter2 = 0;
            }
            if counter1 == 4 {
                counter1 = 0;
                p1w = true;
            }
            if counter2 == 4 {
                counter2 = 0;
                p2w = true;
            }
        }
        counter1 = 0;
        counter2 = 0;
        // check across cols (horizontal)
        for i in 0..NUM_COLS {
            if self.board[row][i] == cond1[counter1] {
                counter1 += 1;
            } else if self.board[row][i] == cond1[0] {
                counter1 = 1;
            } else {
                counter1 = 0;
            }
            if self.board[row][i] == cond2[counter2] {
                counter2 += 1;
            } else if self.board[row][i] == cond2[0] {
                counter2 = 1;
            } else {
                counter2 = 0;
            }
            if counter1 == 4 {
                counter1 = 0;
                p1w = true;
            }
            if counter2 == 4 {
                counter2 = 0;
                p2w = true;
            }
        }
        counter1 = 0;
        counter2 = 0;
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
                if self.board[j][i] == cond1[counter1] {
                    counter1 += 1;
                } else if self.board[j][i] == cond1[0] {
                    counter1 = 1;
                } else {
                    counter1 = 0;
                }
                if counter1 == 4 {
                    counter1 = 0;
                    p1w = true;
                }
                if self.board[j][i] == cond2[counter2] {
                    counter2 += 1;
                } else if self.board[j][i] == cond2[0] {
                    counter2 = 1;
                } else {
                    counter2 = 0;
                }
                if counter2 == 4 {
                    counter2 = 0;
                    p2w = true;
                }
                j+=1;
            }
        }
        counter1 = 0;
        counter2 = 0;
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
            if self.board[j][i] == cond1[counter1] {
                counter1 += 1;
            } else if self.board[j][i] == cond1[0] {
                counter1 = 1;
            } else {
                counter1 = 0;
            }
            if counter1 == 4 {
                counter1 = 0;
                p1w = true;
            }
            if self.board[j][i] == cond2[counter2] {
                counter2 += 1;
            } else if self.board[j][i] == cond2[0] {
                counter2 = 1;
            } else {
                counter2 = 0;
            }
            if counter2 == 4 {
                counter2 = 0;
                p2w = true;
            }
            if j > 0 {
                j -= 1;
            } else {
                break;
            }
        }
        if p1w && p2w { // draw
            return -1
        } else if p1w {
            return 1
        } else if p2w {
            return 2
        } else {
            return 0;
        }
    }
}