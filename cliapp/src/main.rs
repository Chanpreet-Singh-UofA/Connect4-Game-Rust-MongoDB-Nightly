mod connect4;
use connect4::connect4::Connect4;
use connect4::connect4_cpu::*;
mod toototto;
use toototto::toototto::TootOtto;
use toototto::toototto_cpu::*;

use std::io;

fn printBoard(board: Connect4) {
    for i in 0..6 {
        for j in 0..7 {
            match board.board[i][j] {
                0 => print!("| "),
                1 => print!("|X"),
                2 => print!("|O"),
                3_u8..=u8::MAX => {}
            }
        }
        print!("|\n");
    }
    print!("\n");
}

fn printBoardToot(board: TootOtto) {
    for i in 0..6 {
        for j in 0..7 {
            match board.board[i][j] {
                0 => print!("| "),
                1 => print!("|T"),
                2 => print!("|O"),
                3_u8..=u8::MAX => {}
            }
        }
        print!("|\n");
    }
    print!("\n");
}

fn connect4_loop() {
    let mut board = Connect4::new();
    let depth = 6; // difficulties: easy: 2, medium: 4, hard: 6
    let mut turn = 1;
    let mut play = true;
    let mut playermove: usize = 0;
    printBoard(board);
    while play {
        while turn == 1 { // player turn
            println!("Enter Positon X (0-6):");
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
            let cpu = connect4::connect4_cpu::minimax(board, depth, 2, playermove).0;
            match cpu {
                None => {
                    // game over?
                },
                Some(col) => {
                    println!("{}", col);
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

fn toot_loop_pvp() {
    let mut board = TootOtto::new();
    let mut turn = 1;
    let mut play = true;
    let mut playermove: usize;
    printBoardToot(board);
    while play {
        while turn == 1 { // player 1 turn
            println!("Player 1 spell TOOT, Enter Positon: (pos token), pos:0-6, token:1-2");
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
            let token = userInputVec[1] as u8;
            // insert piece into board
            let res = board.insert(playermove, token);
            if res == true {
                printBoardToot(board);
                let win = board.check_win_draw(playermove);
                if win == -1 {
                    println!("Draw!");
                    play = false;
                    turn = 0;
                } else if win == 1 {
                    println!("Player 1 wins!");
                    play = false;
                    turn = 0;
                } else if win == 2 {
                    println!("Player 2 wins!");
                    play = false;
                    turn = 0;
                } else {
                    turn = 2;
                }
            } else {
                println!("Column is full!");
            }
        }
        while turn == 2 { // player 2 turn
            println!("Player 2 spell OTTO, Enter Positon: (pos token), pos:0-6, token:1-2");
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
            let token = userInputVec[1] as u8;
            // insert piece into board
            let res = board.insert(playermove, token);
            if res == true {
                printBoardToot(board);
                let win = board.check_win_draw(playermove);
                if win == -1 {
                    println!("Draw!");
                    play = false;
                    turn = 0;
                } else if win == 1 {
                    println!("Player 1 wins!");
                    play = false;
                    turn = 0;
                } else if win == 2 {
                    println!("Player 2 wins!");
                    play = false;
                    turn = 0;
                } else {
                    turn = 1;
                }
            } else {
                println!("Column is full!");
            }
        }
    }
}

fn toot_loop() {
    let mut board = TootOtto::new();
    let depth = 4; // difficulties: easy: 2, hard: 4
    let mut turn = 1;
    let mut play = true;
    let mut playermove: usize = 0;
    printBoardToot(board);
    while play {
        while turn == 1 { // player turn
            println!("Player 1 spell TOOT, Enter Positon: (pos token), pos:0-6, token:1-2");
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
            let token = userInputVec[1] as u8;
            // insert piece into board
            let res = board.insert(playermove, token);
            if res == true {
                printBoardToot(board);
                let win = board.check_win_draw(playermove);
                if win == -1 {
                    println!("Draw!");
                    play = false;
                    turn = 0;
                } else if win == 1 {
                    println!("Player 1 wins!");
                    play = false;
                    turn = 0;
                } else if win == 2 {
                    println!("CPU wins!");
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
            let cpu = toototto::toototto_cpu::minimax(board, depth, 2, playermove).0;
            match cpu {
                None => {
                    println!("cpu did nothing -- error");
                },
                Some((col, token)) => {
                    println!("{} {}", col, token);
                    board.insert(col, token); // always will return true
                    printBoardToot(board);
                    let win = board.check_win_draw(col);
                    if win == -1 {
                        println!("Draw!");
                        play = false;
                        turn = 0;
                    } else if win == 1 {
                        println!("Player 1 wins!");
                        play = false;
                        turn = 0;
                    } else if win == 2 {
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

fn main() {
    loop {
        println!("Choose game:");
        println!("Connect4 vs cpu: 1");
        println!("TOOT-OTTO pvp: 2");
        println!("TOOT-OTTO vs cpu: 3");
        let mut userInput = String::new();
        io::stdin().read_line(&mut userInput).expect("failed to readline");
        if let Some('\n')=userInput.chars().next_back() {
            userInput.pop();
        }
        if let Some('\r')=userInput.chars().next_back() {
            userInput.pop();
        }
        let userInputVec: Vec<u32> = userInput.split(" ").map(|x| x.parse::<u32>().unwrap()).collect();

        if userInputVec[0] == 1 {
            println!("Playing Connect4 vs cpu.");
            connect4_loop();
        } else if userInputVec[0] == 2 {
            println!("Playing TOOT-OTTO pvp.");
            toot_loop_pvp();

        } else if userInputVec[0] == 3 {
            println!("Playing TOOT-OTTO vs cpu.");
            toot_loop();
        }
    }
}
