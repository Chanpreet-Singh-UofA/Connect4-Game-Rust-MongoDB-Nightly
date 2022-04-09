use crate::connect4::connect4::Connect4;

pub const NUM_COLS: usize = 7;
pub const NUM_ROWS: usize = 6; // also max height

pub fn printBoard(board: Connect4) {
    for i in 0..6 {
        for j in 0..7 {
            print!("{} ", board.board[i][j]);
        }
        print!("\n");
    }
    print!("\n");
}

pub fn minimax(board: Connect4, depth: usize, player: u8, col: usize) -> (Option<usize>, i32) {
    //https://medium.com/analytics-vidhya/artificial-intelligence-at-play-connect-four-minimax-algorithm-explained-3b5fc32e4a4f
    // check for terminating game state
    if board.clone().check_win_draw(col, 2) == 1 { // cpu wins
        return (None, 1024)
    } else if board.clone().check_win_draw(col, 1) == 1 { // player wins
        return (None, -1024)
    } else if board.clone().check_win_draw(col, 2) == -1 { // draw
        return (None, 0)
    } else if depth == 0 {
        return (None, score_position(board, 2))
    }

    if player == 2 { // maximizing player
        let mut value = -65536;
        let mut column = 0;
        for i in 0..NUM_COLS {
            let mut board_copy = board.clone();
            if board_copy.insert(i, player) {
                let new_score = minimax(board_copy, depth-1, 1, i).1;
                // if depth == 6 { // print values of cpu options
                //     println!("{}", new_score);
                // }
                if new_score > value {
                    value = new_score;
                    column = i;
                }
            }
        }
        return (Some(column), value)

    } else { // minimizing player
        let mut value = 65536;
        let mut column = 0;
        for i in 0..NUM_COLS {
            let mut board_copy = board.clone();
            if board_copy.insert(i, player) {
                let new_score = minimax(board_copy, depth-1, 2, i).1;
                if new_score < value {
                    value = new_score;
                    column = i;
                }
            }
        }
        return (Some(column), value)
    }
}

// evaluates a window for a given player
// note that for the minimax algorithm, the player in question will always be the ai
fn evaluate_window(window: &Vec<u8>, player: u8) -> i32 {
    //https://medium.com/analytics-vidhya/artificial-intelligence-at-play-connect-four-minimax-algorithm-explained-3b5fc32e4a4f
    let mut score = 0;
    let opp: u8;
    if player == 1 {
        opp = 2;
    } else {
        opp = 1;
    }
    let mut count_p = 0;
    let mut count_o = 0;
    let mut count_e = 0;
    for i in window {
        if *i == player {
            count_p += 1;
        } else if *i == opp {
            count_o += 1;
        } else {
            count_e += 1;
        }
    }
    if count_p == 4 {
        score += 100;
    } else if count_p == 3 && count_e == 1 {
        score += 5;
    } else if count_p == 2 && count_e == 2 {
        score += 2;
    } else if count_o == 3 && count_e == 1 {
        score -= 4;
    }

    return score
}

fn score_position(board: Connect4, player: u8) -> i32 {
    //https://medium.com/analytics-vidhya/artificial-intelligence-at-play-connect-four-minimax-algorithm-explained-3b5fc32e4a4f
    const WINDOW_LEN: usize = 4;

    let mut score = 0;
    // score the center column
    {
        let col = 3;
        let mut center_count = 0;
        for row in 0..NUM_ROWS {
            if board.board[row][col] == player {
                center_count += 1;
            }
        }
        score += center_count * 3;
    }
    // score horizontal
    for r in 0..NUM_ROWS {
        let mut window: Vec<u8>;
        for c in 0..NUM_COLS-3 {
            let slice = &board.board[r][c..c+WINDOW_LEN];
            window = slice.to_vec();
            score += evaluate_window(&window, player);
        }
    }
    // score vertical
    for c in 0..NUM_COLS {
        for r in 0..NUM_ROWS-3 {
            let mut window: Vec<u8> = vec![];
            for i in r..r+WINDOW_LEN {
                window.push(board.board[i][c]);
            }
            score += evaluate_window(&window, player);
        }
    }
    // score positive diagonals
    for r in 0..NUM_ROWS-3 {
        for c in 0..NUM_COLS-3 {
            let mut window: Vec<u8> = vec![];
            for i in 0..WINDOW_LEN {
                window.push(board.board[r+i][c+i]);
            }
            score += evaluate_window(&window, player);
        }
    }
    // score negative diagonals
    for r in 0..NUM_ROWS-3 {
        for c in 0..NUM_COLS-3 {
            let mut window: Vec<u8> = vec![];
            for i in 0..WINDOW_LEN {
                window.push(board.board[r+3-i][c+i]);
            }
            score += evaluate_window(&window, player);
        }
    }

    return score
}