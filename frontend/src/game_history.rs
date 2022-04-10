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
use chrono::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH, Duration};

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
    GetOK(Vec<Game>),
    GetFailed(String),
}

pub struct game_history {
    data: Option<Vec<Game>>,
}

impl game_history {
    fn view_data(&self) -> Html {

        if let Some(ref games) = self.data {
            html!{
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
            }
        }
        else {
            html! {
                <tr><td colspan="6">{"Loading..."}</td></tr>
            }
        }
    }
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


pub async fn get_game_data() -> Result<Vec<Game>, FetchError> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);
    let request = Request::new_with_str_and_init("http://localhost:8000/getAllGame", &opts)?;
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();
    let json = JsFuture::from(resp.json()?).await?;
    let game_data: Vec<Game> = json.into_serde().unwrap();
    log::info!("game data got");
    Ok(game_data)
}

impl Component for game_history {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {

        ctx.link().send_future(async {
            match get_game_data().await {
                Ok(game_data) => {
                    Msg::GetOK(game_data)
                },
                Err(err) => {
                    Msg::GetFailed(err.to_string())
                }
            }
        });
        game_history{
            data: None,
        }

    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GetOK(game_data) => {

                for game in &game_data {
                }

                self.data = Some(game_data);
                true
            },
            Msg::GetFailed(err) => {
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> VNode {
        let link = ctx.link();
        html! {
            <div class="w3-container" id="services" style="margin-top:75px">
            <h5 class="w3-xxxlarge w3-text-red"><b>{"Game History"}</b></h5>
            <hr style="width:50px;border:5px solid red" class="w3-round"/>
            <div id="game-stream">
            <table border=1>
                <tr>
                    <th>{"Game-ID"}</th>
                    <th>{"Game Type"}</th>
                    <th>{"Player1"}</th>
                    <th>{"Player2"}</th>
                    <th>{"Winner"}</th>
                    <th>{"When Played"}</th>
                </tr>
                { self.view_data() }
            </table>
            </div>
            </div>
        }
    }
}        