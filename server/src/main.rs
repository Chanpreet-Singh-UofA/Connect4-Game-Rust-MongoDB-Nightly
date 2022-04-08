//use mongodb::{Client, options::{ClientOptions, ResolverConfig}};
use mongodb::{options::ClientOptions, sync::Client, sync::Database};
//use mongodb::bson::{doc, Document};
use serde::{Deserialize, Serialize};
// This trait is required to use `try_next()` on the cursor
use futures::stream::TryStreamExt;
use mongodb::{bson::doc, options::FindOptions};


// #[derive(Debug, Serialize, Deserialize)]
// struct Player {
//     playerName: String,
//     playerScore: String,
// }
use bson::{bson, Bson};
use std::env;
use std::error::Error;
use tokio;


pub struct MyMongo {
   db: Database,
}

impl MyMongo {

    pub fn setup() -> Result<MyMongo, mongodb::error::Error> {

    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017")?;

    client_options.app_name = Some("Connect 4".to_string());

    let client = Client::with_options(client_options)?;

    //https://www.mongodb.com/developer/quickstart/rust-crud-tutorial/
    let player_collection =client.database("myMongoDB");

    Ok(MyMongo { db: player_collection })
    }


    pub fn addPlayer(&self, playerName: String, playerNewScore: String) -> Result<(), Box<dyn Error>> {

        let playerDoc = doc! {
            "playerName": playerName,
            "playerScore": playerNewScore
        };

        let playerCol = self.db.collection("player");
        playerCol.insert_one(playerDoc, None);


        // let player1 = playerCol.find_one(
        // doc! {
        //         "playerName": playerName
        // },
        // None,
        // )?.expect("Missing 'CX' player.");

        // println!("Player 1: {}", player1);

        Ok(())
    }

    pub fn update_score(&self, playerName: String, playerNewScore: String) -> Result<(), Box<dyn Error>> {
        let filter = doc!{"playerName":playerName.to_string()};
        let update = doc!{"$set": {"playerScore":playerNewScore.to_string()}};

        let playerCol = self.db.collection("player");
        playerCol.update_one(filter, update, None)?;

        Ok(())
    }


    pub fn get_score(&self, playerName: String) -> Result<(), Box<dyn Error>> {
        // let filter = doc!{"playerName":playerName.to_string()};
        // let update = doc!{"$set": {"playerScore":playerNewScore}};

        let playerCol = self.db.collection("player");
        //playerCol.update_one(filter, update, None)?;


        let player1 = playerCol.find_one(
        doc! {
                "playerName": playerName
        },
        None,
        )?.expect("Missing 'CX' player.");

        //let mut player1 = (player1).unwrap();

        println!("Score Found: {}", player1);

        if let Ok(score) = player1.get_str("playerScore") {
            println!("Score: {}", score);
         } else {
            println!("no score found");
         }


        Ok(())
    }

}


fn main() {
    let mut m = MyMongo::setup();


    match MyMongo::setup() {
		Ok(mut db) => match db.addPlayer("New Player - Chan".to_string(),"32".to_string()){
            Ok(res) => {
                println!("Player added");
            }
            Err(_) => {println!("Player not added");
            }
        },
        Err(_) => {println!("Player not added");
        }
    }

    match MyMongo::setup() {
		Ok(mut db) => match db.update_score("New Player - Chan".to_string(),"35".to_string()){
            Ok(res) => {
                println!("Player score updated");
            }
            Err(_) => {println!("Player not score updated");
            }
        },
        Err(_) => {println!("Player not score updated");
        }
    }



    match MyMongo::setup() {
		Ok(mut db) => match db.get_score("New Player - Chan".to_string()){
            Ok(res) => {
                println!("Player score printed");
            }
            Err(_) => {println!("Player not score printed");
            }
        },
        Err(_) => {println!("Player not score printed");
        }
    }




}
