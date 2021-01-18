use std::{error::Error};

pub fn parse_moves(game: &String) -> Result<Vec<(&str, &str)>, Box<dyn Error>> {
    let lines = game.lines();

    // Assumed format for moves_line 1. d4 Nf6 2. Bf4 Nc6 3. e3 d5 etc...
    let moves_line = lines.filter(|&line| (*line).starts_with("1")).next().unwrap(); 

    let re = regex::Regex::new(r"\d+\. ").unwrap();
    let mut split = re.split(moves_line).collect::<Vec<_>>();

    // The first item is always an empty string, because the game line starts with the split regex
    split.remove(0);

    let split2 = split.iter().map(|the_move| parse_move(the_move)).collect();

    Ok(split2)
}

fn parse_move(the_move: &str) -> (&str, &str) {
    let moves = the_move.split(" ").collect::<Vec<&str>>();
    let white_move: &str = moves[0].trim();
    let black_move: &str = match moves.len() {
        2 => "",
        3 => moves[1].trim(),
        _ => panic!("Did not expect to get here")
    };
    // let black_move: &str = if moves.len() > 1 {moves[1].trim()} else {""};
    (white_move, black_move)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_moves() -> Result<(), Box<dyn Error>> {
        let moves_str = String::from("1. d4 Nf6 2. Bf4 Nc6 3. e3 1-0");
        let moves = parse_moves(&moves_str)?;
        assert_eq!(moves.len(), 3);

        assert_eq!(moves[0].0, "d4");
        assert_eq!(moves[1].0, "Bf4");
        assert_eq!(moves[2].0, "e3");

        assert_eq!(moves[0].1, "Nf6");
        assert_eq!(moves[1].1, "Nc6");
        assert_eq!(moves[2].1, "");

        Ok(())
    }

    #[test]
    fn test_parse_move_with_both_sides() {
        let (white_move, black_move) = parse_move("Bg2 Qxg2# 0-1");

        assert_eq!(white_move, "Bg2");
        assert_eq!(black_move, "Qxg2#");
    }

    #[test]
    fn test_parse_move_with_one_side() {
        let (white_move, black_move) = parse_move("Qxc4 1-0");

        assert_eq!(white_move, "Qxc4");
        assert_eq!(black_move, "");
    }

    #[test]
    fn test_parse_real_moves() {
        let game_str = String::from(
"[Event \"Casual Correspondence game\"]
[Site \"https://lichess.org/tVRT2qs7\"]
[Date \"2021.01.05\"]
[White \"mcubos\"]
[Black \"interfaceLayer\"]
[Result \"0-1\"]
[UTCDate \"2021.01.05\"]
[UTCTime \"03:00:32\"]
[WhiteElo \"1500\"]
[BlackElo \"1500\"]
[Variant \"Standard\"]
[TimeControl \"-\"]
[ECO \"A45\"]
[Opening \"Indian Game\"]
[Termination \"Normal\"]

1. d4 Nf6 2. Bf4 Nc6 3. e3 d5 4. Nf3 Bf5 5. Nbd2 e6 6. c3 Bd6 7. Bg5 h6 8. Bh4 g5 9. Bg3 Ne4 10. Nxe4 Bxe4 11. Ne5 Nxe5 12. dxe5 Be7 13. f3 Bg6 14. f4 Qd7 15. Be2 O-O-O 16. O-O h5 17. a4 g4 18. h4 gxh3 19. gxh3 h4 20. Bh2 Rdg8 21. Kh1 Qc6 22. Bb5 Be4+ 23. Rf3 Qb6 24. Be2 Qxb2 25. Bf1 Qf2 26. Qe2 Bxf3+ 27. Qxf3 Qxf3+ 28. Bg2 Qxg2# 0-1


");
        
        let str_moves = parse_moves(&game_str).unwrap();
        assert_eq!(str_moves.len(), 28);
        let (last_white_move, last_black_move) = str_moves[27];
        assert_eq!(last_white_move, "Bg2");
        assert_eq!(last_black_move, "Qxg2#");
    }

    #[test]
    fn test_real_game_2() {
        let game_str = String::from(
"[Event \"Rated Rapid game\"]
[Site \"https://lichess.org/tzUJbFEX\"]
[Date \"2020.12.23\"]
[White \"mcubos\"]
[Black \"Arogo\"]
[Result \"1-0\"]
[UTCDate \"2020.12.23\"]
[UTCTime \"03:19:06\"]
[WhiteElo \"1500\"]
[BlackElo \"1590\"]
[WhiteRatingDiff \"+329\"]
[BlackRatingDiff \"-8\"]
[Variant \"Standard\"]
[TimeControl \"600+0\"]
[ECO \"B01\"]
[Opening \"Scandinavian Defense\"]
[Termination \"Normal\"]

1. e4 d5 2. Nc3 d4 3. Nd5 f5 4. f3 Nf6 5. d3 Nxd5 6. exd5 Qxd5 7. f4 e5 8. Be2 e4 9. dxe4 Qxe4 10. Nf3 c5 11. c3 Nc6 12. O-O Bd6 13. Bd3 Qd5 14. Re1+ Be6 15. c4 Qxc4 16. Bxc4 O-O-O 17. Rxe6 Na5 18. Qd3 Nxc4 19. Qxc4 1-0"
        );

        let str_moves = parse_moves(&game_str).unwrap();
        assert_eq!(str_moves.len(), 19);
        let (last_white_move, last_black_move) = str_moves[18];
        assert_eq!(last_white_move, "Qxc4");
        assert_eq!(last_black_move, "");
    }
}