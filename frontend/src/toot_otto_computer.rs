#[path = "../src/toototto/mod.rs"]
mod toototto;
use toototto::toototto::TootOtto;
use toototto::toototto_cpu::*;

use crate::cell_toot::Cellule;
use rand::Rng;
use yew::html::Scope;
use web_sys::HtmlInputElement;
use yew::{classes, html, Component, Context, Html, NodeRef};
use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};
use serde::{Deserialize, Serialize};
use web_sys::{Request, RequestInit, RequestMode, Response};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use chrono;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct Game {
    gameID: String,
    gameType: String,
    player1: String,
    player2: String,
    winner: String,
    playedTime: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FetchError {
    err: JsValue,
}

impl Display for FetchError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.err, f)
    }
}
impl Error for FetchError {}

impl From<JsValue> for FetchError {
    fn from(value: JsValue) -> Self {
        Self { err: value }
    }
}


pub enum Msg {
    Reset,
    ToggleCellule(usize),
    updatePlayer1(String),
    selectT(),
    selectO(),
    setDifficultyEasy(),
    setDifficultyMedium(),
    GetOK(),
    GetFailed(String),
}

pub struct toot_otto_computer {
    cellules: Vec<Cellule>,
    cellules_width: usize,
    cellules_height: usize,
    player1: String,
    input: NodeRef,
    letter: String,
    difficulty:String,
    depth: usize,
    current_player: u8,
    board: TootOtto,
    winnerString: String,
    is_game_over: bool,
}

pub async fn send_post_request(game_result:Game) -> Result<(), FetchError> {
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);

    let game_result_json = serde_json::to_string(&game_result).unwrap();

    opts.body(Some(&JsValue::from_serde(&game_result_json).unwrap()));


    let request = Request::new_with_str_and_init("http://localhost:8000/addGame", &opts)?;

    request
        .headers()
        .set("Content-Type", "text/plain")?;
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    

    Ok(())
}

impl toot_otto_computer {

    fn reset(&mut self) {
        for cellule in self.cellules.iter_mut() {
            cellule.set_dead();
        }
    }

    fn row_col_as_idx(&self, row: isize, col: isize) -> usize {
        let row = wrap(row, self.cellules_height as isize);
        let col = wrap(col, self.cellules_width as isize);

        row * self.cellules_width + col
    }
    fn idx_to_row_col(&self, idx: usize) -> (isize, isize) {
        let row = idx / self.cellules_width;
        let col = idx % self.cellules_width;
        (row as isize, col as isize)
    }

    fn view_cellule(&self, idx: usize, cellule: &Cellule, link: &Scope<Self>) -> Html {
        let cellule_status = {
            if cellule.is_alive() {
                "cellule-live"
            } else if (cellule.is_dead()) {
                "cellule-dead"
            } else {
                "cellule-green"
            }
        };
        html! {
            <div key={idx} class={classes!("game-cellule", cellule_status)}
                onclick={link.callback(move |_| Msg::ToggleCellule(idx))}>{
                if cellule.is_alive() {
                    "T"
                } else if (cellule.is_dead()) {
                    ""
                } else {
                    "O"
                }
            }
            </div>
        }
    }
}
impl Component for toot_otto_computer {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {

        let (cellules_width, cellules_height) = (7, 6);

        Self {
            cellules: vec![Cellule::new_dead(); cellules_width * cellules_height],
            cellules_width,
            cellules_height,
            player1: String::from(""),
            input: NodeRef::default(),
            letter: "T".to_string(),
            difficulty:String::from("Easy"),
            depth: 2,
            current_player: 1,
            board:TootOtto::new(),
            winnerString: String::from(""),
            is_game_over: true,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Reset => {
                self.reset();
                log::info!("Reset");
                self.is_game_over = false;
                self.winnerString = String::from("");
                self.board = TootOtto::new();
                self.current_player = 1;
                true
            }
            Msg::ToggleCellule(idx) => {
                if(!self.is_game_over){
                    let (_, col) = self.idx_to_row_col(idx);
                    let mut piece:u8 = 0;
                    if(self.letter == "T") {
                        piece = 1;
                    }
                    else if(self.letter == "O"){
                        piece = 2;
                    }
                    if(self.board.insert(col as usize,piece)){
                        let row = 6 - self.board.col_height[col as usize];
                        let index = self.row_col_as_idx(row as isize,col);
                        let cellule = self.cellules.get_mut(index).unwrap();
                        cellule.toggle(self.letter.clone());
                        let gameState = self.board.check_win_draw(col as usize);
                        if( gameState == 1){
                            self.winnerString = format!("{} wins!", self.player1);
                            let game_result = Game {
                                gameID: "".to_string(),
                                gameType: "TOOT-OTTO".to_string(),
                                player1: self.player1.clone(),
                                player2: "Computer".to_string(),
                                winner: self.player1.clone(),
                                playedTime: "test time".to_string(),
                            };
                            ctx.link().send_future(async {
                                match send_post_request(game_result).await {
                                    Ok(_) => {
                                        Msg::GetOK()
                                    },
                                    Err(err) => {
                                        Msg::GetFailed(err.to_string())
                                    }
                                }
                            });
                            self.is_game_over = true;
                        }
                        else if (gameState == 2){
                            self.winnerString = format!("Computer wins!");
                            let game_result = Game {
                                gameID: "".to_string(),
                                gameType: "TOOT-OTTO".to_string(),
                                player1: self.player1.clone(),
                                player2: "Computer".to_string(),
                                winner: "Computer".to_string(),
                                playedTime: "test time".to_string(),
                            };
                            ctx.link().send_future(async {
                                match send_post_request(game_result).await {
                                    Ok(_) => {
                                        Msg::GetOK()
                                    },
                                    Err(err) => {
                                        Msg::GetFailed(err.to_string())
                                    }
                                }
                            });
                            self.is_game_over = true;
                        }
                        else if( gameState == -1){
                            self.winnerString = String::from("Draw");
                            let game_result = Game {
                                gameID: "".to_string(),
                                gameType: "TOOT-OTTO".to_string(),
                                player1: self.player1.clone(),
                                player2: "Computer".to_string(),
                                winner: "Draw".to_string(),
                                playedTime: "test time".to_string(),
                            };
                            ctx.link().send_future(async {
                                match send_post_request(game_result).await {
                                    Ok(_) => {
                                        Msg::GetOK()
                                    },
                                    Err(err) => {
                                        Msg::GetFailed(err.to_string())
                                    }
                                }
                            });
                            self.is_game_over = true;
                        }
                        else{
                            self.current_player = 2;
                        }
                        // computer turn
                        let cpu = toototto::toototto_cpu::minimax(self.board, self.depth, 2, col as usize).0;
                        match cpu {
                            None => {
                                //println!("cpu did nothing -- error");
                            },
                            Some((ccol, token)) => {
                                self.board.insert(ccol, token); // always will return true
                                let letter = if token == 1 {
                                    "T"
                                } else {
                                    "O"
                                };
                                let row = 6 - self.board.col_height[ccol as usize];
                                let index = self.row_col_as_idx(row as isize,ccol as isize);
                                let cellule = self.cellules.get_mut(index).unwrap();
                                cellule.toggle(letter.to_string());

                                let win = self.board.check_win_draw(ccol);
                                if win == -1 {
                                    self.winnerString = String::from("Draw");
                                    let game_result = Game {
                                        gameID: "".to_string(),
                                        gameType: "TOOT-OTTO".to_string(),
                                        player1: self.player1.clone(),
                                        player2: "Computer".to_string(),
                                        winner: "Draw".to_string(),
                                        playedTime: "test time".to_string(),
                                    };
                                    ctx.link().send_future(async {
                                        match send_post_request(game_result).await {
                                            Ok(_) => {
                                                Msg::GetOK()
                                            },
                                            Err(err) => {
                                                Msg::GetFailed(err.to_string())
                                            }
                                        }
                                    });
                                    self.is_game_over = true;
                                } else if win == 1 {
                                    self.winnerString = format!("{} wins!", self.player1);
                                    let game_result = Game {
                                        gameID: "".to_string(),
                                        gameType: "TOOT-OTTO".to_string(),
                                        player1: self.player1.clone(),
                                        player2: "Computer".to_string(),
                                        winner: self.player1.clone(),
                                        playedTime: "test time".to_string(),
                                    };
                                    ctx.link().send_future(async {
                                        match send_post_request(game_result).await {
                                            Ok(_) => {
                                                Msg::GetOK()
                                            },
                                            Err(err) => {
                                                Msg::GetFailed(err.to_string())
                                            }
                                        }
                                    });
                                    self.is_game_over = true;
                                } else if win == 2 {
                                    self.winnerString = format!("Computer wins!");
                                    let game_result = Game {
                                        gameID: "".to_string(),
                                        gameType: "TOOT-OTTO".to_string(),
                                        player1: self.player1.clone(),
                                        player2: "Computer".to_string(),
                                        winner: "Computer".to_string(),
                                        playedTime: "test time".to_string(),
                                    };
                                    ctx.link().send_future(async {
                                        match send_post_request(game_result).await {
                                            Ok(_) => {
                                                Msg::GetOK()
                                            },
                                            Err(err) => {
                                                Msg::GetFailed(err.to_string())
                                            }
                                        }
                                    });
                                    self.is_game_over = true;
                                } else {
                                    self.current_player = 1;
                                }
                            }
                        }

                        return true;
                    }
                }

                false
            }
            Msg::updatePlayer1(player1) => {
                if(self.is_game_over){
                    self.player1 = player1;
                    return true;
                }
                false
            }
            Msg::selectT() => {
                if(!self.is_game_over){
                    self.letter = "T".to_string();
                }
                true
            }
            Msg::selectO() => {
                if(!self.is_game_over){
                    self.letter = "O".to_string();
                }
                true
            }
            Msg::setDifficultyEasy() => {
                if(self.is_game_over){
                    self.difficulty = String::from("Easy");
                    self.depth = 2;
                }
                true
            }
            Msg::setDifficultyMedium() => {
                if(self.is_game_over){
                    self.difficulty = String::from("Medium");
                    self.depth = 4;
                }
                true
            },
            Msg::GetOK() => {
                false
            },
            Msg::GetFailed(err) => {
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let update_difficulty_easy = ctx.link().callback(|_| Msg::setDifficultyEasy());
        let update_difficulty_medium = ctx.link().callback(|_| Msg::setDifficultyMedium());
        let update_letter = ctx.link().callback(|_| Msg::selectT());
        let update_letter2 = ctx.link().callback(|_| Msg::selectO());
        let my_input_ref = self.input.clone();
        let onchange = ctx.link().batch_callback(move |_| {
            let input = my_input_ref.cast::<HtmlInputElement>();

            input.map(|input| Msg::updatePlayer1(input.value()))
        });
        let cell_rows =
            self.cellules
                .chunks(self.cellules_width)
                .enumerate()
                .map(|(y, cellules)| {
                    let idx_offset = y * self.cellules_width;

                    let cells = cellules
                        .iter()
                        .enumerate()
                        .map(|(x, cell)| self.view_cellule(idx_offset + x, cell, ctx.link()));
                    html! {
                        <div key={y} class="game-row">
                            { for cells }
                        </div>
                    }
                });

        html! {
            <div>
                <section class="game-container">
                    <header class="app-header">
                        <h1 class="app-title">{ "TOOT-OTTO vs Computer" }</h1>
                    </header>
                    <section class="game-area">
                    <div class="game-of-life">
                    { for cell_rows }
                    </div>
                <div class="game-buttons">
                    <input
                        ref={self.input.clone()}
                        id="textbox1"
                        type="text"
                        placeholder="Player 1's Name"
                        onchange = {onchange}
                    />
                    <button class="game-button" onclick={ctx.link().callback(|_| Msg::Reset)}>{ "Start" }</button>
                </div>
                <div class="readout">
                    {"Select a Letter:  "}
                    <input type="radio" id="T" value="T" checked={self.letter=="T" } oninput = {update_letter} />
                    <label for="T">{"T"}</label>
                    <input type="radio" id="O" value="O" checked={self.letter=="O"} oninput = {update_letter2}/>
                    <label for="O">{"O"}</label>
                </div>
                <div class="readout">
                    {"Select Difficulty"}
                    <input type="radio" id="Easy" value="Easy" checked={self.difficulty=="Easy" } oninput = {update_difficulty_easy} />
                    <label for="Easy">{"Easy"}</label>
                    <input type="radio" id="Medium" value="Medium" checked={self.difficulty=="Medium"} oninput = {update_difficulty_medium}/>
                    <label for="Medium">{"Medium"}</label>
                </div>
                <div class="readout">
                    <div class = "text2">
                        {format!("Player 1: {}", self.player1)}
                    </div>
                    <div class = "text2">
                    {format!("Objective: spell TOOT")}
                    </div>
                    <div class = "text2">
                        {format!("Current Difficulty: {}", self.difficulty)}
                    </div>
                    <div class = "text2">
                        {format!("{}", self.winnerString)}
                    </div>
                </div>
                    </section>
                </section>
                <footer class="app-footer">
                    <strong class="footer-text">
                      { "" }
                    </strong>
                </footer>
            </div>
        }
    }
}

fn wrap(coord: isize, range: isize) -> usize {
    let result = if coord < 0 {
        coord + range
    } else if coord >= range {
        coord - range
    } else {
        coord
    };
    result as usize
}
