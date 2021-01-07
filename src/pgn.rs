// #[macro_use] extern crate lazy_static;
// extern crate regex;
use regex::Regex;

use std::{error::Error, vec};

// get_moves returns a vector where each element is a vector of length 2. These elements
// in the sub vector represent a move for white and a move for black
fn parse_moves(game: &String) -> Result<Vec<(&str, &str)>, Box<dyn Error>> {
    // Err("Bad request")?;
    let lines = game.lines();

    // Assumed format for moves_line 1. d4 Nf6 2. Bf4 Nc6 3. e3 d5 etc...
    let moves_line = lines.filter(|&line| (*line).starts_with("1")).next().unwrap(); 

    let re = regex::Regex::new(r"\d+\. ").unwrap();
    let mut split = re.split(moves_line).collect::<Vec<_>>();
    split.remove(0);
    let split2 = split.iter().map(|the_move| the_move.splitn(2, " ").collect::<Vec<&str>>()).collect::<Vec<_>>();
    let split3 = split2.iter().map(|v| (v[0].trim(), v[1].trim())).collect::<Vec<_>>();

    Ok(split3)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_moves() -> Result<(), Box<dyn Error>> {
        let moves_str = String::from("1. d4 Nf6 2. Bf4 Nc6 3. e3 d5");
        let moves = parse_moves(&moves_str)?;
        assert_eq!(moves.len(), 3);
        Ok(())
    }
}