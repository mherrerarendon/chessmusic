
use std::{error::Error, vec};

pub fn get_moves(game: &String) -> Result<String, Box<dyn Error>> {
    let lines = game.lines();
    let mut iter = lines.filter(|&line| (*line).starts_with("1"));
    let moves = iter.next().unwrap();
    Ok(String::from(moves))
}

pub(crate) fn parse_moves(game: &String) -> Result<Vec<String>, Box<dyn Error>> {

    Ok(vec!["test".to_string()])
}