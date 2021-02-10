use super::types::{Role, PieceName};
use super::cell::Cell;
use super::piece::{Piece, Bishop, King, Knight, Pawn, Queen, Rook};


pub struct Board {
    pub pieces: Vec<Box<dyn Piece>>,
}

impl Board {
    #[allow(dead_code)]
    pub fn dump(&self) {
        for row in 1..=8 {
            let chess_row = 9-row;
            let mut the_line = String::new();
            for file in ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'].iter() {
                let cell = Cell {file: *file, row: chess_row};
                match self.get_piece_at_cell(&cell) {
                    Some(piece) => the_line.push(piece.get_char_representation()),
                    None => the_line.push('.')
                }
            }
            println!("{:?} {}", chess_row, the_line);
        }
        println!("\n  abcdefgh");
    }

    #[cfg(test)]
    pub fn new_king_test() -> Board {
        Board {
            pieces: vec![
                Box::new(King::new(true)),
                Box::new(King::new(false)),
            ]
        }
    }

    #[cfg(test)]
    pub fn new_queen_test() -> Board {
        Board {
            pieces: vec![
                Box::new(Queen::new(true)),
                Box::new(Queen::new(false)),
            ]
        }
    }

    #[cfg(test)]
    pub fn new_rook_test() -> Board {
        Board {
            pieces: vec![
                Box::new(Rook::new(true, PieceName::Qrook)),
                Box::new(Rook::new(true, PieceName::Krook)),
                Box::new(Rook::new(false, PieceName::Qrook)),
                Box::new(Rook::new(false, PieceName::Krook)),
            ]
        }
    }

    #[cfg(test)]
    pub fn new_bishop_test() -> Board {
        Board {
            pieces: vec![
                Box::new(Bishop::new(true, PieceName::Qbishop)),
                Box::new(Bishop::new(true, PieceName::Kbishop)),
                Box::new(Bishop::new(false, PieceName::Qbishop)),
                Box::new(Bishop::new(false, PieceName::Kbishop)),
            ]
        }
    }

    pub fn new() -> Board {
        Board {
            pieces: vec![
                // White pieces
                Box::new(Pawn::new(true, PieceName::Apawn)),
                Box::new(Pawn::new(true, PieceName::Bpawn)),
                Box::new(Pawn::new(true, PieceName::Cpawn)),
                Box::new(Pawn::new(true, PieceName::Dpawn)),
                Box::new(Pawn::new(true, PieceName::Epawn)),
                Box::new(Pawn::new(true, PieceName::Fpawn)),
                Box::new(Pawn::new(true, PieceName::Gpawn)),
                Box::new(Pawn::new(true, PieceName::Hpawn)),
                Box::new(Rook::new(true, PieceName::Qrook)),
                Box::new(Knight::new(true, PieceName::Qknight)),
                Box::new(Bishop::new(true, PieceName::Qbishop)),
                Box::new(Queen::new(true)),
                Box::new(King::new(true)),
                Box::new(Bishop::new(true, PieceName::Kbishop)),
                Box::new(Knight::new(true, PieceName::Kknight)),
                Box::new(Rook::new(true, PieceName::Krook)),
                
                // Black pieces
                Box::new(Pawn::new(false, PieceName::Apawn)),
                Box::new(Pawn::new(false, PieceName::Bpawn)),
                Box::new(Pawn::new(false, PieceName::Cpawn)),
                Box::new(Pawn::new(false, PieceName::Dpawn)),
                Box::new(Pawn::new(false, PieceName::Epawn)),
                Box::new(Pawn::new(false, PieceName::Fpawn)),
                Box::new(Pawn::new(false, PieceName::Gpawn)),
                Box::new(Pawn::new(false, PieceName::Hpawn)),
                Box::new(Rook::new(false, PieceName::Qrook)),
                Box::new(Knight::new(false, PieceName::Qknight)),
                Box::new(Bishop::new(false, PieceName::Qbishop)),
                Box::new(Queen::new(false)),
                Box::new(King::new(false)),
                Box::new(Bishop::new(false, PieceName::Kbishop)),
                Box::new(Knight::new(false, PieceName::Kknight)),
                Box::new(Rook::new(false, PieceName::Krook)),
                
            ]
        }
    }

    pub fn get_piece_at_cell(&self, cell: &Cell) -> Option<&Box<dyn Piece>> {
        self.pieces.iter().find(|piece| piece.get_curr_cell() == cell)
    }

    pub fn get_piece_with_name(&self, name: PieceName, white: bool) -> Option<&Box<dyn Piece>> {
        self.pieces.iter().find(|piece| piece.get_name() == name && piece.is_white() == white)
    }

    fn get_mut_piece_with_name(&mut self, name: PieceName, white: bool) -> Option<&mut Box<dyn Piece>> {
        self.pieces.iter_mut().find(|piece| piece.get_name() == name && piece.is_white() == white)
    }

    pub fn get_pieces_with_role(&self, role: Role, white: bool) -> Vec<&Box<dyn Piece>> {
        return self.pieces.iter().filter(|piece| piece.get_role() == role && piece.is_white() == white).collect();
    }

    fn remove_piece_at_cell(&mut self, cell: &Cell) {
        match self.get_piece_at_cell(&cell) {
            Some(piece) => {
                let index = self.pieces.iter()
                    .position(|x| (x.get_name() == piece.get_name() && x.is_white() == piece.is_white())).unwrap();
                self.pieces.remove(index);
            },
            None => () // Cell was empty, nothing to do
        }
    }

    pub fn move_piece(&mut self, name: PieceName, white: bool, cell: &Cell) {
        self.remove_piece_at_cell(cell);
        match self.get_mut_piece_with_name(name, white) {
            Some(piece) => {
                
                piece.set_new_cell(&cell);
                piece.set_has_moved();
            },
            None => println!("unable to move piece")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_piece_at_cell() {
        let board = Board::new();
        match board.get_piece_at_cell(&Cell {file: 'a', row: 2}) {
            Some(piece) => {
                assert_eq!(piece.get_name(), PieceName::Apawn);
                assert!(piece.is_white());
                assert!(!piece.has_moved());
            },
            None => assert!(false)
        }
    }

    #[test]
    fn test_get_white_pawns() {
        let board = Board::new();
        let white_pawns = board.get_pieces_with_role(Role::Pawn, true);
        assert_eq!(white_pawns.len(), 8);
        assert_eq!(white_pawns[0].get_role(), Role::Pawn);
        assert!(white_pawns[0].is_white());
    }

    #[test]
    fn test_get_black_pawns() {
        let board = Board::new();
        let white_pawns = board.get_pieces_with_role(Role::Pawn, false);
        assert_eq!(white_pawns.len(), 8);
        assert_eq!(white_pawns[0].get_role(), Role::Pawn);
        assert!(!white_pawns[0].is_white());
    }

    #[test]
    fn test_move_pawn() {
        let mut board = Board::new();
        let new_cell = Cell::new("b3");
        board.move_piece(PieceName::Bpawn, true, &new_cell);
        match board.get_piece_at_cell(&new_cell) {
            Some(piece) => {
                assert_eq!(piece.get_name(), PieceName::Bpawn);
                assert!(piece.is_white());
                assert!(piece.has_moved());
            },
            None => assert!(false)
        }
    }

    #[test]
    fn test_move_knight() {
        let mut board = Board::new();
        let new_cell = Cell::new("a3");
        board.move_piece(PieceName::Qknight, true, &new_cell);
        match board.get_piece_at_cell(&new_cell) {
            Some(piece) => {
                assert_eq!(piece.get_name(), PieceName::Qknight);
                assert!(piece.is_white());
                assert!(piece.has_moved());
            },
            None => assert!(false)
        }

        match board.get_piece_at_cell(&Cell::new("b1")) {
            Some(_piece) => {
                panic!("Did not expect to find a piece")
            },
            None => assert!(true)
        }
    }

    #[test]
    fn test_remove_piece() {
        let mut board = Board::new();
        let cell = Cell::new("a1");
        board.remove_piece_at_cell(&cell);
        match board.get_piece_at_cell(&cell) {
            Some(_piece) => panic!("Did not expect to find piece"),
            None => assert!(true)
        }
    }
}