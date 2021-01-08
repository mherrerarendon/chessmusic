use std::collections::HashMap;
use super::types::{Cell, Role};
use super::chess_move::Move;

// struct MovesByPiece {
//     pieces: HashMap<uint32, Vec<String>>,
// }

// impl MovesByPiece {
//     fn add_move(&self, the_move: &str) {
//         self.pieces.insert(1, String::from(the_move));
//     }
// }
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Piece {
    a2, b2, c2, d2, e2, f2, g2, h2, // white pawns
    a1, b1, c1, d1, e1, f1, g1, h1, // white pieces
    a7, b7, c7, d7, e7, f7, g7, h7, // black pawns
    a8, b8, c8, d8, e8, f8, g8, h8, // black pieces
}

struct Game {
    moves_by_piece: HashMap<Piece, Vec<Cell>>
}

impl Game {
    fn new() -> Game {
        Game {
            moves_by_piece: [(Piece::a2, vec![Cell {file: 'a', row: 2}]),
                             (Piece::b2, vec![Cell {file: 'b', row: 2}])]
                .iter().cloned().collect(),
        }
    }

    fn get_position_for_move(&self, piece: &Piece, move_num: usize) -> Cell {
        self.moves_by_piece[piece][move_num]
    }

    fn get_pieces_with_role(white: &bool, role: &Role) -> Vec<Piece> {
        if *role == Role::Pawn {
            return vec![Piece::a2, Piece::b2, Piece::c2, Piece::d2, Piece::e2, Piece::f2, Piece::g2, Piece::h2];
        }
        else {
            return vec![Piece::a7, Piece::b7, Piece::c7, Piece::d7, Piece::e7, Piece::f7, Piece::g7, Piece::h7];
        }
    }

    fn get_surviving_pieces_last_cell_with_role(&self, white: &bool, role: &Role) -> HashMap<Piece, Cell> {
        let mut surviving_pieces_with_role: HashMap<Piece, Cell> = HashMap::new();
        let pieces_with_role = Game::get_pieces_with_role(white, role);
        for piece in pieces_with_role.iter() {
            let piece_moves: &Vec<Cell> = &self.moves_by_piece[piece];
            let last_cell = piece_moves.last().unwrap().clone();
            if last_cell.file != ' ' {
                surviving_pieces_with_role.insert(*piece, last_cell);
            }
        }

        surviving_pieces_with_role
    }

    fn determine_piece_for_move(&self, white: &bool, the_move: &Move) -> Piece {
        let role = the_move.role;
        let surviving_pieces_last_cell = self.get_surviving_pieces_last_cell_with_role(white, &role);
        if the_move.file_hint != ' ' {
            return *surviving_pieces_last_cell.iter().filter(|(_piece, cell)| cell.file == the_move.file_hint).next().unwrap().0;
        }
        else {
            for (piece, cell) in surviving_pieces_last_cell {
                if self.is_valid_move_for_role(&role, &cell) {
                    return piece;
                }
            }
        }

        // TODO: return a Result object
        return Piece::a2;
    }

    fn is_valid_move_for_role(&self, role: &Role, curr_cell: &Cell) -> bool {
        // TODO
        true
    }

    fn add_move(&mut self, white: &bool, the_move: &Move) {
        let piece = self.determine_piece_for_move(white, the_move);
        let mut moves = self.moves_by_piece.get_mut(&piece).unwrap(); //.push(the_move.cell);
        moves.push(the_move.cell);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_position_for_move() {
        let game = Game::new();
        let cell = game.get_position_for_move(&Piece::a2, 0);
        assert_eq!(cell.file, 'a');
        assert_eq!(cell.row, 2);
    }

    #[test]
    fn test_add_move() {
        let mut game = Game::new();
        let the_move = Move::parse_single_move("a3");
        game.add_move(&true, &the_move);
        let cell = game.get_position_for_move(&Piece::a2, 0);
        assert_eq!(cell.file, 'a');
        assert_eq!(cell.row, 3);
    }
}