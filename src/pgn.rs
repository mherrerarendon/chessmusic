// #[macro_use] extern crate lazy_static;
// extern crate regex;
use regex::Regex;

use std::{error::Error, vec};

// get_moves returns a vector where each element is a vector of length 2. These elements
// in the sub vector represent a move for white and a move for black
fn parse_moves(game: &String) -> Result<Vec<(&str, &str)>, Box<dyn Error>> {
    let lines = game.lines();

    // Assumed format for moves_line 1. d4 Nf6 2. Bf4 Nc6 3. e3 d5 etc...
    let moves_line = lines.filter(|&line| (*line).starts_with("1")).next().unwrap(); 

    let re = regex::Regex::new(r"\d+\. ").unwrap();
    let mut split = re.split(moves_line).collect::<Vec<_>>();

    // The first item is always an empty string, because the game line starts with the split regex
    split.remove(0);

    let split2 = split.iter().map(|the_move| the_move.splitn(2, " ").collect::<Vec<&str>>())
        .map(|v| (v[0].trim(), v[1].trim()))
        .collect();

    Ok(split2)
}

fn get_white_moves(moves: &Vec<(&str, &str)>) -> Vec<String> {
    moves.iter().map(|m| String::from(m.0)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_moves() -> Result<(), Box<dyn Error>> {
        let moves_str = String::from("1. d4 Nf6 2. Bf4 Nc6 3. e3 d5");
        let moves = parse_moves(&moves_str)?;
        assert_eq!(moves.len(), 3);

        assert_eq!(moves[0].0, "d4");
        assert_eq!(moves[1].0, "Bf4");
        assert_eq!(moves[2].0, "e3");

        assert_eq!(moves[0].1, "Nf6");
        assert_eq!(moves[1].1, "Nc6");
        assert_eq!(moves[2].1, "d5");

        Ok(())
    }

    #[test]
    fn test_get_white_moves() {
        let moves = vec![("d4", "Nf6"), ("Bf4", "Nc6")];
        let white_moves = get_white_moves(&moves);

        assert_eq!(white_moves.len(), 2);
        assert_eq!(white_moves[0], String::from("d4"));
        assert_eq!(white_moves[1], String::from("Bf4"));
    }
}