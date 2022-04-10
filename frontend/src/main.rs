use yew::prelude::*;
use yew_router::prelude::*;
mod cell;
mod connect_4;
mod cell_toot;
mod toot_otto;
mod connect4_computer;
mod toot_otto_computer;
mod scoreboard;
mod game_history;
use yew::html::Scope;
//implement the yew router
#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
    #[at("/conntect4")]
    connect4,
    #[at("/toot_otto")]
    toot_otto,
    #[at("/connect4_computer")]
    connect4_computer,
    #[at("/toot_otto_computer")]
    toot_otto_computer,
    #[at("/ganme_history")]
    game_history,
    #[at("/scoreboard")]
    scoreboard,
}
pub enum Msg {
    ToggleNavbar,
}
pub struct App {
    navbar_active: bool,
}
impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            navbar_active: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleNavbar => {
                self.navbar_active = !self.navbar_active;
                true
            }
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                { self.view_nav(ctx.link()) }

                <main>
                    <Switch<Route> render={Switch::render(switch)} />
                </main>
                <footer class="footer">
                    <div class="content has-text-centered">
                        { "made by group 7" }
                    </div>
                </footer>
            </BrowserRouter>
        }
    }

}
impl App {
    fn view_nav(&self, link: &Scope<Self>) -> Html {
        let Self { navbar_active, .. } = *self;

        let active_class = if !navbar_active { "is-active" } else { "" };

        html! {
            <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
                <div class="navbar-brand">
                    <h1 class="navbar-item is-size-3">{ "Connect4 with TOOT-OTTO" }</h1>

                    <button class={classes!("navbar-burger", "burger", active_class)}
                        aria-label="menu" aria-expanded="false"
                        onclick={link.callback(|_| Msg::ToggleNavbar)}
                    >
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                    </button>
                </div>
                <div class={classes!("navbar-menu", active_class)}>
                    <div class="navbar-start">
                        <div class="navbar-item has-dropdown is-hoverable">
                            <div class="navbar-link">
                                { "Connect4" }
                            </div>
                            <div class="navbar-dropdown">
                                <Link<Route> classes={classes!("navbar-item")} to={Route::connect4}>
                                    { "play connect4 vs human" }
                                </Link<Route>>
                                <Link<Route> classes={classes!("navbar-item")} to={Route::connect4_computer}>
                                    { "play connect4 vs computer" }
                                </Link<Route>>
                            </div>
                        </div>
                        <div class="navbar-item has-dropdown is-hoverable">
                            <div class="navbar-link">
                                { "TOOT-OTTO" }
                            </div>
                            <div class="navbar-dropdown">
                                <Link<Route> classes={classes!("navbar-item")} to={Route::toot_otto}>
                                    { "play TOOT-OTTO vs human" }
                                </Link<Route>>
                                <Link<Route> classes={classes!("navbar-item")} to={Route::toot_otto_computer}>
                                    { "play TOOT-OTTO vs computer" }
                                </Link<Route>>
                            </div>
                        </div>
                        <div class="navbar-item has-dropdown is-hoverable">
                            <div class="navbar-link">
                                { "Stats" }
                            </div>
                            <div class="navbar-dropdown">
                                <Link<Route> classes={classes!("navbar-item")} to={Route::scoreboard}>
                                    { "Scoreboard" }
                                </Link<Route>>
                                <Link<Route> classes={classes!("navbar-item")} to={Route::game_history}>
                                    { "Game History" }
                                </Link<Route>>
                            </div>
                        </div>
                    </div>
                </div>
            </nav>
        }
    }
}

fn switch(routes: &Route) -> Html {
    match routes.clone() {
        Route::connect4 => html! {
            <connect_4::connect_4 />
        },
        Route::toot_otto => html! {
            <toot_otto::toot_otto />
        },
        Route::connect4_computer => html! {
            <connect4_computer::connect4_computer />
        },
        Route::toot_otto_computer => html! {
            <toot_otto_computer::toot_otto_computer />
        },
        Route::game_history => html! {
            <game_history::game_history />
        },
        Route::scoreboard => html! {
            <scoreboard::ScoreBoard />
        },
    }
}


fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::trace!("Initializing yew...");
    yew::start_app::<App>();
}