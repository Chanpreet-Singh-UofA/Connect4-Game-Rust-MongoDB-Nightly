use yew::{classes, html, Component, Context, Html, NodeRef};
use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};

use yew::prelude::*;
use yew::virtual_dom::VNode;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

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
    gotGameData(Vec<Game>),
    getGameDataFailed(String),
}

pub struct game_history {
    gameData: Option<Vec<Game>>,
}

impl game_history {
    fn view_data(&self) -> Html {

        if let Some(ref games) = self.gameData {
            return html!{
                { games.iter().enumerate().map(|(i, game)| {

                        html! {
                            <tr>
                            <td>{i + 1}</td>
                            <td>{game.gameType.as_str()}</td>
                            <td>{game.player1.as_str()}</td>
                            <td>{game.player2.as_str()}</td>
                            <td>{game.winner.as_str()}</td>
                            <td>{game.playedTime.clone()}</td>
                            </tr>
                        }
                    }).collect::<Html>() }
            };
        }
        html!{}
    }
}

    pub async fn send_post_request(results:Game) -> Result<(), FetchError> {
        let mut options = RequestInit::new();
        options.method("POST");
        options.mode(RequestMode::Cors);
        let results_json = serde_json::to_string(&results).unwrap();
        options.body(Some(&JsValue::from_serde(&results_json).unwrap()));
        let request = Request::new_with_str_and_init("http://localhost:8000/addGame", &options)?;
        request
            .headers()
            .set("Content-Type", "text/plain")?;
        let window = web_sys::window().unwrap();
        let response_value = JsFuture::from(window.fetch_with_request(&request)).await?;
        Ok(())
    }


    pub async fn get_games() -> Result<Vec<Game>, FetchError> {
        let mut options = RequestInit::new();
        options.method("GET");
        options.mode(RequestMode::Cors);
        let request = Request::new_with_str_and_init("http://localhost:8000/getAllGame", &options)?;
        let window = web_sys::window().unwrap();
        let response_value = JsFuture::from(window.fetch_with_request(&request)).await?;
        assert!(response_value.is_instance_of::<Response>());
        let response: Response = response_value.dyn_into().unwrap();
        let json = JsFuture::from(response.json()?).await?;
        let games: Vec<Game> = json.into_serde().unwrap();
        log::info!("game data got");
        Ok(games)
    }

impl Component for game_history {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {

        ctx.link().send_future(async {
            match get_games().await {
                Ok(games) => {
                    Msg::gotGameData(games)
                },
                Err(err) => {
                    Msg::getGameDataFailed(err.to_string())
                }
            }
        });
        game_history{
            gameData: None,
        }

    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::gotGameData(games) => {

                self.gameData = Some(games);
                true
            },
            Msg::getGameDataFailed(err) => {
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> VNode {
        let link = ctx.link();
        html! {
            <div class="w3-container" id="services" style="margin-top:75px;margin-bottom:75px">
            <h5 class="w3-xxxlarge w3-text-blue-gray"><b>{"Game History"}</b></h5>
            <hr style="height:2px;border-width:0;opacity:0"/>
            <hr style="height:2px;border-width:0;color:gray;background-color:gray"/>
            <div id="game-stream">
            <table class="table-center" border=1>
                <tr>
                    <th style="color:white">{"Game-ID"}</th>
                    <th style="color:white">{"Game Type"}</th>
                    <th style="color:white">{"Player1"}</th>
                    <th style="color:white">{"Player2"}</th>
                    <th style="color:white">{"Winner"}</th>
                    <th style="color:white">{"When Played"}</th>
                </tr>
                { self.view_data() }
            </table>
            </div>
            </div>
        }
    }
}        