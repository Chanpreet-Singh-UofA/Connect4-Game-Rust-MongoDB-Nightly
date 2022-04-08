use cell::Cellule;
use rand::Rng;
use yew::html::Scope;
use yew::{classes, html, Component, Context, Html};

mod cell;

pub enum Msg {
    Reset,
    ToggleCellule(usize),
}

pub struct App {
    cellules: Vec<Cellule>,
    cellules_width: usize,
    cellules_height: usize,
}

impl App {

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
impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {

        let (cellules_width, cellules_height) = (7, 6);

        Self {
            cellules: vec![Cellule::new_dead(); cellules_width * cellules_height],
            cellules_width,
            cellules_height,
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
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
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
                            <button class="game-button" onclick={ctx.link().callback(|_| Msg::Reset)}>{ "Reset" }</button>
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

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::trace!("Initializing yew...");
    yew::start_app::<App>();
}