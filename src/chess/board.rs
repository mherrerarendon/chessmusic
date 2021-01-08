use std::collections::HashMap;
use std::{error::Error};
use super::types::{Role, MoveType, Cell, role_char_to_role};


struct Board {
    board: Vec<Vec<String>>,
}

struct Move {
    role: Role,
    move_type: MoveType,
    file_hint: String,
    check: bool,
    cell: Cell,
    secondary_cell: Cell
}

impl Move {
    fn new() -> Move {
        Move {
            role: Role::Pawn,
            move_type: MoveType::Simple,
            file_hint: String::from(""),
            check: false,
            cell: Cell::new(),
            secondary_cell: Cell::new()
        }
    }

    fn parse_castle_move(move_str: &str) -> Move {
        let clean_move_str: String = move_str.chars().filter(|&x| x != '+').collect();
        let mut the_move: Move = Move::new();
        the_move.check = move_str.contains("+");
        if clean_move_str == "O-O" {
            the_move.role = Role::King;
            the_move.move_type = MoveType::CastleKing;
        }
        else {
            // Assume else is "O-O-O"
            the_move.role = Role::Queen;
            the_move.move_type = MoveType::CastleQueen;
        }
        return the_move;
    }

    fn parse_non_castle_move(move_str: &str) -> Move {
        let clean_move_str: String = move_str.chars().filter(|&x| x != 'x' && x != '+').collect();
        let mut the_move: Move = Move::new();
        the_move.check = move_str.contains("+");
        if move_str.contains("x") {
            the_move.move_type = MoveType::Take;
        }
        
        let re = regex::Regex::new(r"(\D?)(\D)(\d)").unwrap();
        let caps = re.captures(&clean_move_str).unwrap();
        the_move.role = caps.get(1).map_or(Role::Pawn, |m| role_char_to_role(m.as_str()));
        the_move.cell = Cell {
            file: caps.get(2).map_or(String::from(""), |m| String::from(m.as_str())),
            row: caps.get(3).map_or(0, |m| m.as_str().parse::<u32>().unwrap())
        };
        return the_move;
    }

    // Assumed format of move_str "Nxd6" for Knight takes on d6
    fn parse_single_move(move_str: &str) -> Move {
        if move_str.contains("O-O") {
            return Move::parse_castle_move(move_str);
        }
        else {
            return Move::parse_non_castle_move(move_str);
        }
    }

    // fn parse_moves(moves_str: &Vec<(&str, &str)>) -> Vec<(Move, Move)> {
    //     let parsed_moves = moves_str.iter()
    //         .map(|m| )
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_move() {
        let the_move = Move::parse_single_move("d4");
        assert_eq!(the_move.role, Role::Pawn);
        assert_eq!(the_move.move_type, MoveType::Simple);
        assert_eq!(the_move.file_hint, "");
        assert_eq!(the_move.check, false);
        assert_eq!(the_move.cell.file, "d");
        assert_eq!(the_move.cell.row, 4);
    }

    #[test]
    fn test_parse_move_with_check() {
        let the_move = Move::parse_single_move("d4+");
        assert_eq!(the_move.role, Role::Pawn);
        assert_eq!(the_move.move_type, MoveType::Simple);
        assert_eq!(the_move.file_hint, "");
        assert_eq!(the_move.check, true);
        assert_eq!(the_move.cell.file, "d");
        assert_eq!(the_move.cell.row, 4);
    }

    #[test]
    fn test_parse_non_pawn_move() {
        let the_move = Move::parse_single_move("Be4");
        assert_eq!(the_move.role, Role::Bishop);
        assert_eq!(the_move.move_type, MoveType::Simple);
        assert_eq!(the_move.file_hint, "");
        assert_eq!(the_move.check, false);
        assert_eq!(the_move.cell.file, "e");
        assert_eq!(the_move.cell.row, 4);
    }

    #[test]
    fn test_parse_non_pawn_move_with_take() {
        let the_move = Move::parse_single_move("Qxg6");
        assert_eq!(the_move.role, Role::Queen);
        assert_eq!(the_move.move_type, MoveType::Take);
        assert_eq!(the_move.file_hint, "");
        assert_eq!(the_move.check, false);
        assert_eq!(the_move.cell.file, "g");
        assert_eq!(the_move.cell.row, 6);
    }

    #[test]
    fn test_parse_king_side_castle() {
        let the_move = Move::parse_single_move("O-O"); // These are capital "o"s
        assert_eq!(the_move.role, Role::King);
        assert_eq!(the_move.move_type, MoveType::CastleKing);
        assert_eq!(the_move.file_hint, "");
        assert_eq!(the_move.check, false);
    }

    #[test]
    fn test_parse_queen_side_castle() {
        let the_move = Move::parse_single_move("O-O-O"); // These are capital "o"s
        assert_eq!(the_move.role, Role::Queen);
        assert_eq!(the_move.move_type, MoveType::CastleQueen);
        assert_eq!(the_move.file_hint, "");
        assert_eq!(the_move.check, false);
    }
}

impl Board {
    fn new() -> Board {
        let files = vec!["a", "b", "c", "d", "e", "f", "g", "h"];
        let mut rows = Vec::new();
        for n in 1..=8 {
            let mut row = Vec::new();
            let row_str = n.to_string();
            for file in files.iter() {
                let cell_name = format!("{}{}", file, row_str);
                row.push(cell_name);
            }

            rows.push(row);
        }

        Board {board: rows}
    }

    // fn file_to_index(the_file: &str) -> u32 {
    //     let files = vec!["a", "b", "c", "d", "e", "f", "g", "h"];
    //     files.iter().position(|&r| r == the_file).unwrap()
    // }

    // fn get_possible_moves(piece: &str, curr_cell: &str) -> Vec<String> {

    // }
}