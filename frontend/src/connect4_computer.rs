#[path = "../src/connect4/mod.rs"]
mod connect4;
use connect4::connect4::Connect4;
use connect4::connect4_cpu::*;

use crate::cell::Cellule;
use rand::Rng;
use yew::html::Scope;
use yew::events::Event;
use web_sys::{EventTarget, HtmlInputElement};
use yew::{classes, html, Component, Context, Html, NodeRef};


pub enum Msg {
    Reset,
    ToggleCellule(usize),
    updatePlayer1(String),
    setDifficultyEasy(),
    setDifficultyMedium(),
    setDifficultyHard(),
}

pub struct InputData {
    pub value: String,
}

pub struct connect4_computer {
    cellules: Vec<Cellule>,
    cellules_width: usize,
    cellules_height: usize,
    player1: String,
    input: NodeRef,
    difficulty:String,
    depth: usize,
    current_player: u8,
    board: Connect4,
    winnerString: String,
    is_game_over: bool,
}

impl connect4_computer {

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
                onclick={link.callback(move |_| Msg::ToggleCellule(idx))}>
            </div>
        }
    }
}
impl Component for connect4_computer {
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
            difficulty:String::from("Easy"),
            depth: 2,
            current_player: 1,
            board: Connect4::new(),
            winnerString: String::from(""),
            is_game_over: true,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Reset => {
                self.reset();
                log::info!("Reset");
                self.is_game_over = false;
                self.winnerString = String::from("");
                self.board = Connect4::new();
                self.current_player = 1;
                true
            }
            Msg::ToggleCellule(idx) => {
                if(!self.is_game_over){
                    let (_, col) = self.idx_to_row_col(idx);
                    if(self.board.insert(col as usize,self.current_player)){
                        let row = 6 - self.board.col_height[col as usize];
                        let index = self.row_col_as_idx(row as isize,col);
                        let cellule = self.cellules.get_mut(index).unwrap();
                        cellule.toggle(self.current_player);
                        let gameState = self.board.check_win_draw(col as usize, self.current_player);
                        if( gameState == 1){
                            self.winnerString = format!("{} wins!", self.player1);
                            self.is_game_over = true;
                        }
                        else if( gameState == -1){
                            self.winnerString = String::from("Draw");
                            self.is_game_over = true;
                        }
                        else{
                            self.current_player = 2;
                        }
                        // ai turn
                        let cpu = connect4::connect4_cpu::minimax(self.board, self.depth, 2, col as usize).0;
                        match cpu {
                            None => {
                                // game over?
                            },
                            Some(col) => {
                                let res = self.board.insert(col, 2); // always will return true
                                let row = 6 - self.board.col_height[col as usize];
                                let index = self.row_col_as_idx(row as isize,col as isize);
                                let cellule = self.cellules.get_mut(index).unwrap();
                                cellule.toggle(self.current_player);

                                let win = self.board.check_win_draw(col, 2);
                                if win == -1 {
                                    self.winnerString = String::from("Draw");
                                    self.is_game_over = true;
                                } else if win == 1 {
                                    self.winnerString = String::from("Computer wins!");
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
                self.player1 = player1;
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
            }
            Msg::setDifficultyHard() => {
                if(self.is_game_over){
                    self.difficulty = String::from("Hard");
                    self.depth = 6;
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let update_difficulty_easy = ctx.link().callback(|_| Msg::setDifficultyEasy());
        let update_difficulty_medium = ctx.link().callback(|_| Msg::setDifficultyMedium());
        let update_difficulty_hard = ctx.link().callback(|_| Msg::setDifficultyHard());
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
                        <h1 class="app-title">{ "Connect4 vs Computer" }</h1>
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
                            {"Select Difficulty"}
                            <input type="radio" id="Easy" value="Easy" checked={self.difficulty=="Easy" } oninput = {update_difficulty_easy} />
                            <label for="Easy">{"Easy"}</label>
                            <input type="radio" id="Medium" value="Medium" checked={self.difficulty=="Medium"} oninput = {update_difficulty_medium}/>
                            <label for="Medium">{"Medium"}</label>
                            <input type="radio" id="Hard" value="Hard" checked={self.difficulty=="Hard"} oninput = {update_difficulty_hard}/>
                            <label for="Hard">{"Hard"}</label>
                        </div>
                        <div class="readout">
                            <div>
                                {format!("Player 1: {}", self.player1)}
                            </div>
                            <div>
                                {format!("Current Difficulty:{}", self.difficulty)}
                            </div>
                            <div>
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
