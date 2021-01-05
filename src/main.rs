mod lichess;
mod pgn;

use std::{error::Error, vec};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let game = lichess::get_game(String::from("tzUJbFEX")).await?; 
    println!("Game:\n\n{}", game);

    let moves = pgn::get_moves(&game)?;
    println!("Moves:\n\n{}", moves);

    let parsed_moves = pgn::parse_moves(&moves)?;
    println!("Parsed moves:\n\n{:?}", parsed_moves);
    Ok(())
}

