use std::error::Error;

pub async fn get_game(game_id: &str) -> Result<String, Box<dyn Error>> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::runtime::Runtime;

    #[test]
    fn test_get_game() -> Result<(), Box<dyn Error>> {
        Runtime::new()
            .expect("Failed to create Tokio runtime")
            .block_on(get_game("tzUJbFEX"))?;
        Ok(())
    }
}