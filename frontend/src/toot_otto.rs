#[path = "../src/toototto/mod.rs"]
mod toototto;
use toototto::toototto::TootOtto;

use crate::cell_toot::Cellule;
use rand::Rng;
use yew::html::Scope;
use web_sys::HtmlInputElement;
use yew::{classes, html, Component, Context, Html, NodeRef};


pub enum Msg {
    Reset,
    ToggleCellule(usize),
    updatePlayer1(String),
    updatePlayer2(String),
    selectT(),
    selectO(),
}

pub struct toot_otto {
    cellules: Vec<Cellule>,
    cellules_width: usize,
    cellules_height: usize,
    player1: String,
    player2: String,
    input: NodeRef,
    input2: NodeRef,
    letter: String,
    current_player: u8,
    board: TootOtto,
    winnerString: String,
    is_game_over: bool,
}

impl toot_otto {

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
impl Component for toot_otto {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {

        let (cellules_width, cellules_height) = (7, 6);

        Self {
            cellules: vec![Cellule::new_dead(); cellules_width * cellules_height],
            cellules_width,
            cellules_height,
            player1: String::from(""),
            player2: String::from(""),
            input: NodeRef::default(),
            input2: NodeRef::default(),
            letter: "T".to_string(),
            current_player: 1,
            board: TootOtto::new(),
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
                self.board = TootOtto::new();
                self.current_player = 1;
                true
            }
            Msg::ToggleCellule(idx) => {
                //cellule.toggle();
                
                true
            }
            Msg::updatePlayer1(player1) => {
                if(self.is_game_over){
                    self.player1 = player1;
                    return true;
                }
                false
            }
            Msg::updatePlayer2(player2) => {
                if(self.is_game_over){
                    self.player2 = player2;
                    return true;
                }
                false    
            }
            Msg::selectT() => {
                if(self.is_game_over){
                    self.letter = "T".to_string();
                    return true;
                }
                false
            }
            Msg::selectO() => {
                if(self.is_game_over){
                    self.letter = "O".to_string();
                    return true;
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let update_letter = ctx.link().callback(|_| Msg::selectT());
        let update_letter2 = ctx.link().callback(|_| Msg::selectO());
        let my_input_ref = self.input.clone();
        let my_input_ref2 = self.input2.clone();
        let onchange = ctx.link().batch_callback(move |_| {
            let input = my_input_ref.cast::<HtmlInputElement>();

            input.map(|input| Msg::updatePlayer1(input.value()))
        });

        let onchange2 = ctx.link().batch_callback(move |_| {
            let input = my_input_ref2.cast::<HtmlInputElement>();

            input.map(|input| Msg::updatePlayer2(input.value()))
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
                        <h1 class="app-title">{ "toot_otto" }</h1>
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
                    <input
                        ref={self.input2.clone()}
                        id="textbox2"
                        type="text"
                        placeholder="Player 2's Name"
                        //read text from textbox
                        onchange = {onchange2}
                    />
                    <button class="game-button" onclick={ctx.link().callback(|_| Msg::Reset)}>{ "Start" }</button>
                </div>
                <div>
                    {"Select a Disc Type:  "}
                    <input type="radio" id="T" value="T" checked={self.letter=="T" } oninput = {update_letter} />
                    <label for="T">{"T"}</label>
                    <input type="radio" id="O" value="O" checked={self.letter=="O"} oninput = {update_letter2}/>
                    <label for="O">{"O"}</label>
                </div>
                <div class="readout">
                    <div>
                        {format!("player1:{}\tplayer2:{}\tletter:{}", self.player1,self.player2,self.letter)}
                    </div>
                </div>
                    </section>
                </section>
                <footer class="app-footer">
                    <strong class="footer-text">
                      { "connect 4 game vs AI " }
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
