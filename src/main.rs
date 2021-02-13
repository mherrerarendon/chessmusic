mod chessmusic;
mod lichess;

mod pgn;
mod chess;
mod music;

use std::{error::Error, env};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => println!("Playing game {}", args[1]),
        _ => panic!("Incorrect number of arguments.\n Usage: \"chessmusic <game_id>\"")
    }
    let game_str = lichess::get_game(&args[1]).await?; 
    println!("Game:\n\n{}", game_str);

    chessmusic::play_game(&game_str);

    Ok(())
}

