// use reqwest::Result;
// use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let resp = reqwest::get("https://lichess.org/game/export/tzUJbFEX?clocks=false&evals=false").await?;
    println!("Status: {}", resp.status());

    let body = resp.text().await?;
    println!("Body:\n\n{}", body);
    Ok(())
}

// async fn get_game(game_id: u32) -> Result<str, reqwest::Error> {
//     Ok("test")
// }