#![feature(decl_macro)]
//https://github.com/SergioBenitez/Rocket/issues/1134
// https://stackoverflow.com/questions/62412361/how-to-set-up-cors-or-options-for-rocket-rs for learning how to attach CORS headers to rocket
use mongodb::{options::ClientOptions, sync::Client, sync::Database};
use serde::{Deserialize, Serialize};
use futures::stream::TryStreamExt;
use mongodb::{bson::doc, options::FindOptions};
#[macro_use] extern crate rocket;
use rocket_contrib::json::Json;
use bson::{bson, Bson};
use std::env;
use std::error::Error;
use tokio;
use mongodb::sync::Collection;
use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use chrono::{Datelike, Timelike, Utc};

pub struct CORS;

impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    gameID: String,
    gameType: String,
    player1: String,
    player2: String,
    winner: String,
    playedTime: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    playerName: String,
    playerWins: u32,
}

pub struct MyMongo {
   pub db: Database,
}

impl MyMongo {

    pub fn setup() -> Result<MyMongo, mongodb::error::Error> {

    let mut client_options = ClientOptions::parse("mongodb://localhost:27017")?;

    client_options.app_name = Some("Connect 4".to_string());

    let client = Client::with_options(client_options)?;

    //https://www.mongodb.com/developer/quickstart/rust-crud-tutorial/
    let player_collection =client.database("myMongoDB");

    Ok(MyMongo { db: player_collection })
    }
}


//Add a new game. Add Plyers before adding a new game. 
#[post("/addGame", format = "text/plain", data = "<gameInfo>")]   
pub fn addGame(gameInfo: Json<Game>) {

    match MyMongo::setup() {
        Ok(mut db) =>{
            let time = chrono::offset::Local::now();
            let (is_pm, hour) = time.hour12();
            let gameDoc = doc! {
                "gameID": &gameInfo.gameID,
                "gameType": &gameInfo.gameType,
                "player1": &gameInfo.player1,
                "player2": &gameInfo.player2,
                "winner": &gameInfo.winner,
                "playedTime":  format!("{}-{:02}-{:02} at {:02}:{:02} {}", time.year(), time.month(), time.day(), hour, time.minute(), if is_pm { "PM" } else { "AM" })
            };
    
            let gamesCollection = db.db.collection("game");
            gamesCollection.insert_one(gameDoc, None);

            let playerCollection = db.db.collection("player");
                
            let filter = doc!{"playerName":&gameInfo.winner};
            let update = doc!{"$inc": {"playerWins":(1)}};
    
            playerCollection.update_one(filter, update, None);
        }
        Err(_) => {println!("Game not added");
        }
    }
}

//Return a game with a particular game ID

//Return all the games. 
#[get("/getAllGame")]    
pub fn getAllGame() -> Json<Vec<bson::Document>>  {

    match MyMongo::setup() {
    Ok(mut db) =>
        {
            let gamesCollection = db.db.collection("game"); 
            let mut games = Vec::new();

            let cursor = gamesCollection.find(doc!{},None,).unwrap();

            for game in cursor {
                if let Ok(game_doc) = game {
                    games.push(game_doc);         
                }
            }
            return Json(games)
        }
        Err(_) => {println!("getAllGame failed");
        return Json(Vec::new())
        }
    }
}

//Return number of games. 
#[get("/getGamesCount")]  
pub fn getGamesCount() -> Json<String>{

    match MyMongo::setup() {
    Ok(mut db) =>
        {
            let gamesCollection = db.db.collection("game"); 
            let gamesCount = gamesCollection.find(doc!{},None,).unwrap();
            return Json(String::from(gamesCount.count().to_string()));
        }
        Err(_) => {println!("getAllGame failed");
            return Json(String::from("Count couldnot be found"));
        }
    }
}




fn main() {
    let mut m = MyMongo::setup();
    rocket::ignite().attach(CORS).mount("/", routes![addGame, getAllGame]).launch();
}
