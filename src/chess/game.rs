use std::collections::HashMap;
use super::types::Cell;

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

    fn get_position_for_move(&self, piece: &Piece, move_num: &usize) -> Cell {
        self.moves_by_piece[piece][*move_num]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_game() {
        let game = Game::new();
        let cell = game.get_position_for_move(&Piece::a2, &0);
        assert_eq!(cell.file, 'a');
        assert_eq!(cell.row, 2);
    }
}