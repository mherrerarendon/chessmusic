use super::types::{Role, MoveType, role_char_to_role};
use super::cell::Cell;

pub struct Move {
    pub role: Role,
    pub move_type: MoveType,
    pub file_hint: char,
    check: bool,
    pub cell: Cell
}

impl Move {
    fn new() -> Move {
        Move {
            role: Role::Pawn,
            move_type: MoveType::Simple,
            file_hint: ' ',
            check: false,
            cell: Cell {file: ' ', row: 0}
        }
    }

    fn parse_castle_move(move_str: &str) -> Move {
        let clean_move_str: String = move_str.chars().filter(|&x| x != '+' && x != '#').collect();
        let mut the_move: Move = Move::new();
        the_move.check = move_str.contains("+");
        if clean_move_str == "O-O" {
            the_move.role = Role::King;
            the_move.move_type = MoveType::CastleKing;
        }
        else {
            // Assume else is "O-O-O"
            the_move.role = Role::King;
            the_move.move_type = MoveType::CastleQueen;
        }
        return the_move;
    }

    // TODO: promotion ("=") and check mate ("#")
    fn parse_non_castle_move(move_str: &str) -> Move {
        let clean_move_str: String = move_str.chars().filter(|&x| x != 'x' && x != '+' && x != '#').collect();
        let mut the_move: Move = Move::new();
        the_move.check = move_str.contains("+");
        if move_str.contains("x") {
            the_move.move_type = MoveType::Take;
        }
        
        // move with file hint "Nbd2"
        let re = regex::Regex::new(r"([RNBQK]?)([a-h]?)([a-h])(\d)").unwrap();
        let caps = re.captures(&clean_move_str).unwrap();
        the_move.role = caps.get(1).map_or(Role::Pawn, |m| role_char_to_role(m.as_str()));
        // if the_move.role == Role::Pawn {
            match caps.get(2) {
                Some(m) => {
                    let file_hint_as_str = m.as_str();
                    if file_hint_as_str.len() > 0 {
                        the_move.file_hint = file_hint_as_str.chars().into_iter().next().unwrap();
                    }
                },
                None => ()
            }
        // }
        the_move.cell = Cell {
            file: caps.get(3).map_or(' ', |m| m.as_str().chars().next().unwrap()),
            row: caps.get(4).map_or(0, |m| m.as_str().parse::<i32>().unwrap())
        };
        return the_move;
    }

    pub fn parse(move_str: &str) -> Move {
        if move_str.contains("O-O") {
            return Move::parse_castle_move(move_str);
        }
        else {
            return Move::parse_non_castle_move(move_str);
        }
    }

    pub fn parse_moves(moves: &Vec<(&str, &str)>) -> Vec<(Move, Move)> {
        let new_moves: Vec<(Move, Move)> = moves.iter().map(|x| (Move::parse(x.0), Move::parse(x.1))).collect();
        new_moves
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::super::pgn;

    #[test]
    fn test_parse_simple_pawn_move() {
        let the_move = Move::parse("a4");
        assert_eq!(the_move.role, Role::Pawn);
        assert_eq!(the_move.move_type, MoveType::Simple);
        assert_eq!(the_move.file_hint, ' ');
        assert_eq!(the_move.check, false);
        assert_eq!(the_move.cell, Cell {file: 'a', row: 4});

        let the_move = Move::parse("h4");
        assert_eq!(the_move.role, Role::Pawn);
        assert_eq!(the_move.move_type, MoveType::Simple);
        assert_eq!(the_move.file_hint, ' ');
        assert_eq!(the_move.check, false);
        assert_eq!(the_move.cell, Cell {file: 'h', row: 4});
    }

    #[test]
    fn test_parse_pawn_move_with_take() {
        let the_move = Move::parse("dxe5");
        assert_eq!(the_move.role, Role::Pawn);
        assert_eq!(the_move.move_type, MoveType::Take);
        assert_eq!(the_move.file_hint, 'd');
        assert_eq!(the_move.check, false);
        assert_eq!(the_move.cell, Cell {file: 'e', row: 5});
    }

    #[test]
    fn test_parse_pawn_move_with_check() {
        let the_move = Move::parse("d4+");
        assert_eq!(the_move.role, Role::Pawn);
        assert_eq!(the_move.move_type, MoveType::Simple);
        assert_eq!(the_move.file_hint, ' ');
        assert_eq!(the_move.check, true);
        assert_eq!(the_move.cell, Cell {file: 'd', row: 4});
    }

    #[test]
    fn test_parse_bishop_move() {
        let the_move = Move::parse("Be4");
        assert_eq!(the_move.role, Role::Bishop);
        assert_eq!(the_move.move_type, MoveType::Simple);
        assert_eq!(the_move.file_hint, ' ');
        assert_eq!(the_move.check, false);
        assert_eq!(the_move.cell, Cell {file: 'e', row: 4});
    }

    #[test]
    fn test_parse_knight_move_with_file_hint() {
        let the_move = Move::parse("Nbd2");
        assert_eq!(the_move.role, Role::Knight);
        assert_eq!(the_move.move_type, MoveType::Simple);
        assert_eq!(the_move.file_hint, 'b');
        assert_eq!(the_move.check, false);
        assert_eq!(the_move.cell, Cell::new("d2"));
    }

    #[test]
    fn test_parse_queen_move_with_take() {
        let the_move = Move::parse("Qxg6");
        assert_eq!(the_move.role, Role::Queen);
        assert_eq!(the_move.move_type, MoveType::Take);
        assert_eq!(the_move.file_hint, ' ');
        assert_eq!(the_move.check, false);
        assert_eq!(the_move.cell, Cell {file: 'g', row: 6});
    }

    #[test]
    fn test_parse_king_side_castle() {
        let the_move = Move::parse("O-O"); // These are capital "o"s
        assert_eq!(the_move.role, Role::King);
        assert_eq!(the_move.move_type, MoveType::CastleKing);
        assert_eq!(the_move.file_hint, ' ');
        assert_eq!(the_move.check, false);
    }

    #[test]
    fn test_parse_queen_side_castle() {
        let the_move = Move::parse("O-O-O"); // These are capital "o"s
        assert_eq!(the_move.role, Role::King);
        assert_eq!(the_move.move_type, MoveType::CastleQueen);
        assert_eq!(the_move.file_hint, ' ');
        assert_eq!(the_move.check, false);
    }

}