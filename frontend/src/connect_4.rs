#[path = "../src/connect4/mod.rs"]
mod connect4;
use connect4::connect4::Connect4;

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
    updatePlayer2(String),
}

pub struct InputData {
    pub value: String,
}

pub struct connect_4 {
    cellules: Vec<Cellule>,
    cellules_width: usize,
    cellules_height: usize,
    player1: String,
    player2: String,
    input: NodeRef,
    input2: NodeRef,
}

impl connect_4 {

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
impl Component for connect_4 {
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
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Reset => {
                self.reset();
                log::info!("Reset");
                true
            }
            Msg::ToggleCellule(idx) => {
                let cellule = self.cellules.get_mut(idx).unwrap();
                cellule.toggle();
                true
            }
            Msg::updatePlayer1(player1) => {
                self.player1 = player1;
                true
            }
            Msg::updatePlayer2(player2) => {
                self.player2 = player2;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
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
                        <h1 class="app-title">{ "connect4" }</h1>
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
                        <div class="readout">
                            <div>
                                {format!("player1:{}\tplayer2:{}", self.player1,self.player2)}
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
