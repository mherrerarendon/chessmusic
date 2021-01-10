use super::types::{Role, MoveType, role_char_to_role};
use super::cell::Cell;

pub struct Move {
    pub role: Role,
    pub move_type: MoveType,
    pub file_hint: char,
    check: bool,
    pub cell: Cell,
    secondary_cell: Cell
}

impl Move {
    fn new() -> Move {
        Move {
            role: Role::Pawn,
            move_type: MoveType::Simple,
            file_hint: ' ',
            check: false,
            cell: Cell::new(),
            secondary_cell: Cell::new()
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
            the_move.role = Role::Queen;
            the_move.move_type = MoveType::CastleQueen;
        }
        return the_move;
    }

    fn parse_non_castle_move(move_str: &str) -> Move {
        let clean_move_str: String = move_str.chars().filter(|&x| x != 'x' && x != '+' && x != '#').collect();
        let mut the_move: Move = Move::new();
        the_move.check = move_str.contains("+");
        if move_str.contains("x") {
            the_move.move_type = MoveType::Take;
        }
        
        let re = regex::Regex::new(r"(\D?)(\D)(\d)").unwrap();
        let caps = re.captures(&clean_move_str).unwrap();
        the_move.role = caps.get(1).map_or(Role::Pawn, |m| role_char_to_role(m.as_str()));
        the_move.cell = Cell {
            file: caps.get(2).map_or(' ', |m| m.as_str().chars().next().unwrap()),
            row: caps.get(3).map_or(0, |m| m.as_str().parse::<i32>().unwrap())
        };
        return the_move;
    }

    // Assumed format of move_str "Nxd6" for Knight takes on d6
    pub fn parse(move_str: &str) -> Move {
        if move_str.contains("O-O") {
            return Move::parse_castle_move(move_str);
        }
        else {
            return Move::parse_non_castle_move(move_str);
        }
    }

    fn parse_moves(moves_str: &Vec<(&str, &str)>) -> Vec<(Move, Move)> {
        vec![(Move::new(), Move::new())]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_move() {
        let the_move = Move::parse("d4");
        assert_eq!(the_move.role, Role::Pawn);
        assert_eq!(the_move.move_type, MoveType::Simple);
        assert_eq!(the_move.file_hint, ' ');
        assert_eq!(the_move.check, false);
        assert_eq!(the_move.cell.file, 'd');
        assert_eq!(the_move.cell.row, 4);
    }

    #[test]
    fn test_parse_move_with_check() {
        let the_move = Move::parse("d4+");
        assert_eq!(the_move.role, Role::Pawn);
        assert_eq!(the_move.move_type, MoveType::Simple);
        assert_eq!(the_move.file_hint, ' ');
        assert_eq!(the_move.check, true);
        assert_eq!(the_move.cell.file, 'd');
        assert_eq!(the_move.cell.row, 4);
    }

    #[test]
    fn test_parse_non_pawn_move() {
        let the_move = Move::parse("Be4");
        assert_eq!(the_move.role, Role::Bishop);
        assert_eq!(the_move.move_type, MoveType::Simple);
        assert_eq!(the_move.file_hint, ' ');
        assert_eq!(the_move.check, false);
        assert_eq!(the_move.cell.file, 'e');
        assert_eq!(the_move.cell.row, 4);
    }

    #[test]
    fn test_parse_non_pawn_move_with_take() {
        let the_move = Move::parse("Qxg6");
        assert_eq!(the_move.role, Role::Queen);
        assert_eq!(the_move.move_type, MoveType::Take);
        assert_eq!(the_move.file_hint, ' ');
        assert_eq!(the_move.check, false);
        assert_eq!(the_move.cell.file, 'g');
        assert_eq!(the_move.cell.row, 6);
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
        assert_eq!(the_move.role, Role::Queen);
        assert_eq!(the_move.move_type, MoveType::CastleQueen);
        assert_eq!(the_move.file_hint, ' ');
        assert_eq!(the_move.check, false);
    }

    #[test]
    fn test_parse_moves() {
        // from game id tVRT2qs7
        let moves = "1. d4 Nf6 2. Bf4 Nc6 3. e3 d5 4. Nf3 Bf5 5. Nbd2 e6 6. \
            c3 Bd6 7. Bg5 h6 8. Bh4 g5 9. Bg3 Ne4 10. Nxe4 Bxe4 11. Ne5 Nxe5 12. \
            dxe5 Be7 13. f3 Bg6 14. f4 Qd7 15. Be2 O-O-O 16. O-O h5 17. a4 g4 18. \
            h4 gxh3 19. gxh3 h4 20. Bh2 Rdg8 21. Kh1 Qc6 22. Bb5 Be4+ 23. Rf3 Qb6 \
            24. Be2 Qxb2 25. Bf1 Qf2 26. Qe2 Bxf3+ 27. Qxf3 Qxf3+ 28. Bg2 Qxg2# 0-1";

    }
}