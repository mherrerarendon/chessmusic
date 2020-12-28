// use reqwest::Result;
// use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let game = get_game(String::from("tzUJbFEX")).await?; 

    println!("Game:\n\n{}", game);
    Ok(())
}

async fn get_game(game_id: String) -> Result<String, reqwest::Error> {
    let url = format!("https://lichess.org/game/export/{}?clocks=false&evals=false", game_id);
    let response = reqwest::get(&url).await?;
    let body = response.text().await?;
    Ok(body)
}