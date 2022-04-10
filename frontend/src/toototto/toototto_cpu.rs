use super::toototto::TootOtto;

//use rayon::prelude::*;

pub const NUM_COLS: usize = 7;
pub const NUM_ROWS: usize = 6; // also max height

// fn run_next(board:TootOtto, depth: usize, i: usize, token: u8) -> (Option<(usize, u8)>, i32) {
//     // inspiration taken from:
//     //https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=5f08e8089e6053687d148dd1ebce1007
//     if i < NUM_COLS-1 {
//         //println!("creating thread: {}", i);
//         let mut board_copy = board.clone();
//         if board_copy.insert(i, token) {
//             let (r1, r2) = rayon::join(|| minimax(board_copy, depth-1, 1, i), || run_next(board, depth, i+1, token));
//             // println!("{},{}: {}", i, token, r1.1);
//             if r1.1 >= r2.1 {
//                 return (Some((i,token)), r1.1)
//             } else {
//                 return r2;
//             }
//         } else {
//             return run_next(board, depth, i+1, token);
//         }
//     } else {
//         let mut board_copy = board.clone();
//         if board_copy.insert(i, token) {
//             let r1 = minimax(board_copy, depth-1, 1, i);
//             // println!("{},{}: {}", i, token, r1.1);
//             return (Some((i,token)), r1.1);
//         } else {
//             return (None, -1030)
//         }
//     }
// }

pub fn minimax(board: TootOtto, depth: usize, player: u8, col: usize) -> (Option<(usize, u8)>, i32) {
    // if depth == 4 { // if depth is 6, then the first 7 branches will be ran in parallel, increasing speed by up to 7 times
    //     let mut board_copy = board.clone();
    //     let i = 0;
    //     let (r1,r2) = rayon::join(||run_next(board_copy, depth, i, 1), || run_next(board_copy, depth, i, 2));
    //     // println!("{:?}", r1);
    //     // println!("{:?}", r2);
    //     if r1.1 == r2.1 {
    //         if r1.0.unwrap().0 < r2.0.unwrap().0 {
    //             return r1;
    //         } else {
    //             return r2;
    //         }
    //     } else if r1.1 > r2.1 {
    //         return r1;
    //     } else {
    //         return r2;
    //     }
    // }
    // psuedo code gotten from:
    //https://medium.com/analytics-vidhya/artificial-intelligence-at-play-connect-four-minimax-algorithm-explained-3b5fc32e4a4f
    // check for terminating game state
    let terminal = board.clone().check_win_draw(col);
    if terminal == 2 { // cpu wins
        return (None, 1024-6+depth as i32)
    } else if terminal == 1 { // player wins
        return (None, -1024+6-depth as i32)
    } else if terminal == -1 { // draw
        return (None, 0)
    } else if depth == 0 { // only happens on maximizing players turn
        return (None, score_position(board))
    }

    if player == 2 { // maximizing player
        let mut value = -65536;
        let mut column = 0;
        let mut token = 1;
        for i in 0..NUM_COLS {
            for t in 1..3 {
                let mut board_copy = board.clone();
                if board_copy.insert(i, t) {
                    let new_score = minimax(board_copy, depth-1, 1, i).1;
                    if new_score > value {
                        value = new_score;
                        column = i;
                        token = t;
                    }
                    // if depth == 4 {
                    //     println!("{} {} {}", i, t, new_score);
                    // }
                }
            }
        }
        return (Some((column, token)), value)

    } else { // minimizing player
        let mut value = 65536;
        let mut column = 0;
        let mut token = 1;
        for i in 0..NUM_COLS {
            for t in 0..3 {
                let mut board_copy = board.clone();
                if board_copy.insert(i, t) {
                    let new_score = minimax(board_copy, depth-1, 2, i).1;
                    if new_score < value {
                        value = new_score;
                        column = i;
                        token = t;
                    }
                }
            }
        }
        return (Some((column, token)), value)
    }
}

// evaluates a window for a given player
// note that for the minimax algorithm, the player in question will always be the ai
fn evaluate_window(window: &Vec<u8>) -> i32 {
    // psuedo code gotten from:
    //https://medium.com/analytics-vidhya/artificial-intelligence-at-play-connect-four-minimax-algorithm-explained-3b5fc32e4a4f
    let mut score = 0;
    let condo = [1,2,2,1]; // TOOT -- player wants
    let condp = [2,1,1,2]; // OTTO -- cpu wants
    let mut count_p = 0;
    let mut count_o = 0;
    let mut count_e = 0;
    let mut count = 0;
    for i in window {
        if *i == condp[count] {
            count_p += 1;
        } else if *i == condo[count] {
            count_o += 1;
        } else {
            count_e += 1;
        }
        count += 1;
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

fn score_position(board: TootOtto) -> i32 {
    // psuedo code gotten from:
    //https://medium.com/analytics-vidhya/artificial-intelligence-at-play-connect-four-minimax-algorithm-explained-3b5fc32e4a4f
    const WINDOW_LEN: usize = 4;

    let mut score = 0;
    // score horizontal
    for r in 0..NUM_ROWS {
        let mut window: Vec<u8>;
        for c in 0..NUM_COLS-3 {
            let slice = &board.board[r][c..c+WINDOW_LEN];
            window = slice.to_vec();
            score += evaluate_window(&window);
        }
    }
    // score vertical
    for c in 0..NUM_COLS {
        for r in 0..NUM_ROWS-3 {
            let mut window: Vec<u8> = vec![];
            for i in r..r+WINDOW_LEN {
                window.push(board.board[i][c]);
            }
            score += evaluate_window(&window);
        }
    }
    // score positive diagonals
    for r in 0..NUM_ROWS-3 {
        for c in 0..NUM_COLS-3 {
            let mut window: Vec<u8> = vec![];
            for i in 0..WINDOW_LEN {
                window.push(board.board[r+i][c+i]);
            }
            score += evaluate_window(&window);
        }
    }
    // score negative diagonals
    for r in 0..NUM_ROWS-3 {
        for c in 0..NUM_COLS-3 {
            let mut window: Vec<u8> = vec![];
            for i in 0..WINDOW_LEN {
                window.push(board.board[r+3-i][c+i]);
            }
            score += evaluate_window(&window);
        }
    }

    return score
}