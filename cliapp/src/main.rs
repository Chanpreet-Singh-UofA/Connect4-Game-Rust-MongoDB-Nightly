mod connect4;
use connect4::connect4::Connect4;
use connect4::connect4_cpu::minimax;

use std::io;

pub fn printBoard(board: Connect4) {
    for i in 0..6 {
        for j in 0..7 {
            print!("{} ", board.board[i][j]);
        }
        print!("\n");
    }
    print!("\n");
}

fn main() {
    let mut board = Connect4::new();
    let depth = 6;
    let mut turn = 1;
    let mut play = true;
    let mut playermove: usize = 0;
    printBoard(board);
    while play {
        while turn == 1 { // player turn
            println!("Enter Positon:");
            // get input
            let mut userInput = String::new();
            io::stdin().read_line(&mut userInput).expect("failed to readline");
            if let Some('\n')=userInput.chars().next_back() {
                userInput.pop();
            }
            if let Some('\r')=userInput.chars().next_back() {
                userInput.pop();
            }
            let userInputVec: Vec<u32> = userInput.split(" ").map(|x| x.parse::<u32>().unwrap()).collect();
            playermove = userInputVec[0] as usize;
            // insert piece into board
            let res = board.insert(playermove, 1);
            if res == true {
                printBoard(board);
                let win = board.check_win_draw(playermove, turn);
                if win == -1 {
                    println!("Draw!");
                    play = false;
                    turn = 0;
                } else if win == 1 {
                    println!("Player 1 wins!");
                    play = false;
                    turn = 0;
                } else {
                    turn = 2;
                }
            } else {
                println!("Column is full!");
            }
        }
        if turn == 2 { // cpu turn
            println!("CPU turn:");
            let mut alpha = -65536;
            let mut beta = 65536;
            let cpu = minimax(board, depth, 2, playermove).0;
            match cpu {
                None => {
                    // game over?
                },
                Some(col) => {
                    let res = board.insert(col, 2); // always will return true
                    printBoard(board);
                    let win = board.check_win_draw(col, 2);
                    if win == -1 {
                        println!("Draw!");
                        play = false;
                        turn = 0;
                    } else if win == 1 {
                        println!("CPU wins!");
                        play = false;
                        turn = 0;
                    } else {
                        turn = 1;
                    }
                }
            }
        }
    }
    
}
