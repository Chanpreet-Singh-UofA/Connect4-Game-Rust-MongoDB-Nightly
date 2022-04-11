
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
use std::collections::HashMap;

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

pub struct ScoreBoard {
    gameData: Option<Vec<Game>>,
}

impl ScoreBoard {
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
    // returns the requested stats table (1 = Computer stats, 2 = Computer games, 3 = Player wins)
    fn stats_tables(&self, data_type:u8) -> Html
    {
        if data_type == 1{
            if let Some(ref games) = self.gameData {
                return html!{
    
                    <tr>
                    <td>{games.len() }</td>
                    <td>{ games.iter().filter(|game| game.player2 == "Computer").count() }</td>
                    <td>{ games.iter().filter(|game| game.winner == "Computer").count() }</td>
                    </tr>
    
                };
            }
        }
        else if data_type == 2{
            if let Some(ref games) = self.gameData {
                return html! {
                    { games.iter().filter(|game| game.winner == "Computer").enumerate().map(|(i, game)| {
                            return html! {
                                <tr>
                                <td>{ i + 1 }</td>
                                <td>{ game.gameType.as_str() }</td>
                                <td>{ game.winner.as_str() }</td>
                                <td>{ game.player1.as_str() }</td>
                                <td>{ game.playedTime.as_str() }</td>
                                </tr>
                            }
                        }).collect::<Html>() }
                };
            } 
        }
        else if data_type == 3{
            if let Some(ref games) = self.gameData {

                let mut player_win_counts = HashMap::new();
                for game in games.iter().filter(|game| game.winner != "Draw") {
                    *player_win_counts.entry(game.winner.as_str()).or_insert(0) += 1;
                    *player_win_counts.entry(game.player1.as_str()).or_insert(0) += 0;
                    *player_win_counts.entry(game.player2.as_str()).or_insert(0) += 0;
                }
                for game in games.iter().filter(|game| game.winner == "Draw") {
                    *player_win_counts.entry(game.player1.as_str()).or_insert(0) += 0;
                    *player_win_counts.entry(game.player2.as_str()).or_insert(0) += 0;
                }
    
                let mut player_win_count_vec: Vec<_> = player_win_counts.iter().collect();
                player_win_count_vec.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
    
                return html! {
                    { player_win_count_vec.iter().enumerate().map(|(i, (player, count))| {
                            html! {
                                <tr>
                                <td>{ i + 1 }</td>
                                <td>{ player }</td>
                                <td>{ count }</td>
                                </tr>
                            }
                        }).collect::<Html>() }
                };
            }
        }
        return html!{};
    }
}





impl Component for ScoreBoard {
    type Message = Msg;
    type Properties = ();
    fn create(ctx: &Context<Self>) -> Self {

        ctx.link().send_future(async {
            match ScoreBoard::get_games().await {
                Ok(games) => {
                    Msg::gotGameData(games)
                },
                Err(err) => {
                    Msg::getGameDataFailed(err.to_string())
                }
            }
        });
        ScoreBoard{
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
            <h5 class="w3-xxxlarge w3-text-blue-gray"><b>{"Score Board"}</b></h5>
            <hr style="height:2px;border-width:0;opacity:0"/>
            <hr style="height:2px;border-width:0;color:gray;background-color:gray"/>
            <div id="game-stream">
            <h6 class="w3-text-white"><b>{"Games Won by Computer"}</b></h6>
            <table border=1 style="color: white" class="table-center">
                <tr>
                    <th style="color:white">{" Total Games Played "}</th>
                    <th style="color:white">{" Games Against Computer "}</th>
                    <th style="color:white">{" Games Computer Won "}</th>
                </tr>
                { self.stats_tables(1) }
            </table>

            <h6 class="w3-text-white"><b>{"Details of Games Won by Computer"}</b></h6>
            <table border=1 style="color: white" class="table-center">
                <tr>
                    <th style="color:white">{"Sl. No"}</th>
                    <th style="color:white">{" Games Type "}</th>
                    <th style="color:white">{" Winner "}</th>
                    <th style="color:white">{" Played Against "}</th>
                    <th style="color:white">{" When Played "}</th>
                </tr>
                { self.stats_tables(2) }
            </table>

            <h6 class="w3-text-white"><b>{"Details of Games Won by All Players"}</b></h6>
            <table border=1 style="color: white" class="table-center">
                <tr>
                    <th style="color:white">{"Rank"}</th>
                    <th style="color:white">{" Winner or Draw "}</th>
                    <th style="color:white">{" No. of Wins "}</th>
                </tr>
                { self.stats_tables(3) }
            </table>
            </div>
            </div>
        }
    }
}        