use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let game = get_game(String::from("tzUJbFEX")).await?; 
    println!("Game:\n\n{}", game);
    let moves = get_moves(&game)?;
    println!("Moves:\n\n{}", moves);
    Ok(())
}

async fn get_game(game_id: String) -> Result<String, Box<dyn Error>> {
    let url = format!("https://lichess.org/game/export/{}?clocks=false&evals=false", game_id);
    let response = reqwest::get(&url).await?;
    let success = response.status().is_success();
    if !success {
        println!("Request was not successful.");
        Err("Bad request")?;
    }
    let body = response.text().await?;
    Ok(body)
}

fn get_moves(game: &String) -> Result<String, Box<dyn Error>> {
    let lines = game.lines();
    let mut iter = lines.filter(|&line| (*line).starts_with("1"));
    let moves = iter.next().unwrap();
    Ok(String::from(moves))
}