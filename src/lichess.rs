use std::error::Error;

pub async fn get_game(game_id: String) -> Result<String, Box<dyn Error>> {
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

    #[test]
    fn test_get_game() {
        
        assert_eq!(4, 4);
    }
}