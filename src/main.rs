mod chessmusic;
mod lichess;

mod pgn;
mod chess;
mod music;

use std::{error::Error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let game_str = lichess::get_game(String::from("tzUJbFEX")).await?; 
    println!("Game:\n\n{}", game_str);

    chessmusic::play_game(&game_str);

    Ok(())
}

