#![feature(decl_macro)]
//https://github.com/SergioBenitez/Rocket/issues/1134

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

//Add a new player.
#[post("/addPlayer", format = "json", data = "<playerInfo>")]   
pub fn addPlayer(playerInfo: Json<Player>) -> Json<String> {

    match MyMongo::setup() {
        Ok(mut db) =>
            {
                let playerDoc = doc! {
                    "playerName": &playerInfo.playerName,
                    "playerWins": &playerInfo.playerWins
                };
        
                let playerCollection = db.db.collection("player");
                playerCollection.insert_one(playerDoc, None);
            }
                Err(_) => {println!("Player not added");
                }
    }
    return Json(String::from("Update success"));
}

//Returns the score of a particular player
#[get("/getScore/<playerName>")]    
pub fn getScore(playerName: String) -> Result<(), Box<dyn Error>> {

    match MyMongo::setup() {
        Ok(mut db) =>
            {
                let playerCol = db.db.collection("player"); 

                let player1 = playerCol.find_one(
                doc! {
                        "playerName": playerName
                },
                None,
                )?.expect("Missing Player.");
        
                println!("Score Found: {}", player1);
            }
            Err(_) => {println!("Player score not updated");
            }
    }
    Ok(())
}

//Return the information of all the players. 
#[get("/getAllPlayers")]    
pub fn getAllPlayers() -> Json<Vec<bson::Document>>  {

    match MyMongo::setup() {
    Ok(mut db) =>
        {
            let playerCollection = db.db.collection("player"); 
            let mut players = Vec::new();
            players.clear();

            let cursor = playerCollection.find(doc!{},None,).unwrap();

            for game in cursor {
                if let Ok(game_doc) = game {
                    players.push(game_doc);         
                }
            }
            return Json(players)
        }
        Err(_) => {println!("getAllGame failed");
        return Json(Vec::new())
        }
    }
}

//Add a new game. Add Plyers before adding a new game. 
#[post("/addGame", format = "json", data = "<gameInfo>")]   
pub fn addGame(gameInfo: Json<Game>) -> Json<String> {

    match MyMongo::setup() {
        Ok(mut db) =>{
            let gameDoc = doc! {
                "gameID": &gameInfo.gameID,
                "gameType": &gameInfo.gameType,
                "player1": &gameInfo.player1,
                "player2": &gameInfo.player2,
                "winner": &gameInfo.winner,
                "playedTime": &gameInfo.playedTime
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
    return Json(String::from("Update success"));
}

//Return a game with a particular game ID
#[get("/getGame/<gameID>")]    
pub fn getGame(gameID: String) -> Result<Json<Game>, mongodb::error::Error> {

    match MyMongo::setup() {
    Ok(mut db) =>
        {
            let gamesCollection = db.db.collection("game"); 

            let game = gamesCollection.find_one(doc! {"gameID": gameID},None,)?.expect("Game not found");
            println!("Game Found: {}", game);

            //https://www.mongodb.com/developer/quickstart/rust-crud-tutorial/
            let game22: Game = bson::from_bson(Bson::Document(game))?;

            return Ok(Json(game22));
    
        }
        Err(_) => {println!("Game not found");
        }
    }
    let gameDoc = Game {
        gameID: "101".to_string(),
        gameType: "101".to_string(),
        player1: "101".to_string(),
        player2: "101".to_string(),
        winner: "101".to_string(),
        playedTime: "101".to_string()
    };
    Ok(Json(gameDoc))
}

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

//Return the records of all the games when computer was player 1 or 2
#[get("/getComputerGame")]    
pub fn getComputerGame() -> Json<Vec<bson::Document>> {

    match MyMongo::setup() {
    Ok(mut db) =>
        {
            let gamesCollection = db.db.collection("game"); 
            let gamePlayer1 = gamesCollection.find(doc! {"player1": "Computer"},None).unwrap();
            let gamePlayer2 = gamesCollection.find(doc! {"player2": "Computer"},None).unwrap();
            let mut games = Vec::new();

            for game in gamePlayer1 {
                if let Ok(game_doc) = game {
                    games.push(game_doc);         
                }
            }
            for game in gamePlayer2 {
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

//Return the record of all the games when computer won. 
#[get("/getComputerWins")]    
pub fn getComputerWins() -> Json<Vec<bson::Document>> {

    match MyMongo::setup() {
    Ok(mut db) =>
        {
            let gamesCollection = db.db.collection("game"); 
            let gameWinner = gamesCollection.find(doc! {"winner": "Computer"},None).unwrap();

            let mut games = Vec::new();
            games.clear();

            for game in gameWinner {
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


fn main() {
    let mut m = MyMongo::setup();
    rocket::ignite().mount("/", routes![addPlayer, getScore,getGamesCount, getAllPlayers, addGame, getGame, getComputerWins, getAllGame, getComputerGame]).launch();
}
