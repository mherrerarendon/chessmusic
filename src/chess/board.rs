use std::collections::HashMap;
use std::{error::Error};
use super::types::{Role, PieceName};
use super::cell::Cell;
use super::piece::{Piece};
use super::pawn::Pawn;


pub struct Board {
    pub pieces: Vec<Box<dyn Piece>>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            pieces: vec![
                Box::new(Pawn {name: PieceName::Apawn, white: true, role: Role::Pawn, cell: Cell {file: 'a', row: 2}, first_move: true}),
                Box::new(Pawn {name: PieceName::Bpawn, white: true, role: Role::Pawn, cell: Cell {file: 'b', row: 2}, first_move: true}),
                Box::new(Pawn {name: PieceName::Cpawn, white: true, role: Role::Pawn, cell: Cell {file: 'c', row: 2}, first_move: true}),
                Box::new(Pawn {name: PieceName::Dpawn, white: true, role: Role::Pawn, cell: Cell {file: 'd', row: 2}, first_move: true}),
                Box::new(Pawn {name: PieceName::Epawn, white: true, role: Role::Pawn, cell: Cell {file: 'e', row: 2}, first_move: true}),
                Box::new(Pawn {name: PieceName::Fpawn, white: true, role: Role::Pawn, cell: Cell {file: 'f', row: 2}, first_move: true}),
                Box::new(Pawn {name: PieceName::Gpawn, white: true, role: Role::Pawn, cell: Cell {file: 'g', row: 2}, first_move: true}),
                Box::new(Pawn {name: PieceName::Hpawn, white: true, role: Role::Pawn, cell: Cell {file: 'h', row: 2}, first_move: true}),
                // Box::new(NotPawn {name: PieceName::Qrook, white: true, role: Role::Rook, cell: Cell {file: 'a', row: 1}, first_move: true}),
                // Box::new(Piece {name: PieceName::Qknight, white: true, role: Role::Knight, cell: Cell {file: 'b', row: 1}},
                // Box::new(Piece {name: PieceName::Qbishop, white: true, role: Role::Bishop, cell: Cell {file: 'c', row: 1}},
                // Box::new(Piece {name: PieceName::Queen, white: true, role: Role::Queen, cell: Cell {file: 'd', row: 1}},
                // Box::new(Piece {name: PieceName::King, white: true, role: Role::King, cell: Cell {file: 'e', row: 1}},
                // Box::new(Piece {name: PieceName::Kbishop, white: true, role: Role::Bishop, cell: Cell {file: 'f', row: 1}},
                // Box::new(Piece {name: PieceName::Kknight, white: true, role: Role::Knight, cell: Cell {file: 'g', row: 1}},
                // Box::new(Piece {name: PieceName::Krook, white: true, role: Role::Rook, cell: Cell {file: 'h', row: 1}},
                Box::new(Pawn {name: PieceName::Apawn, white: false, role: Role::Pawn, cell: Cell {file: 'a', row: 7}, first_move: true}),
                Box::new(Pawn {name: PieceName::Bpawn, white: false, role: Role::Pawn, cell: Cell {file: 'b', row: 7}, first_move: true}),
                Box::new(Pawn {name: PieceName::Cpawn, white: false, role: Role::Pawn, cell: Cell {file: 'c', row: 7}, first_move: true}),
                Box::new(Pawn {name: PieceName::Dpawn, white: false, role: Role::Pawn, cell: Cell {file: 'd', row: 7}, first_move: true}),
                Box::new(Pawn {name: PieceName::Epawn, white: false, role: Role::Pawn, cell: Cell {file: 'e', row: 7}, first_move: true}),
                Box::new(Pawn {name: PieceName::Fpawn, white: false, role: Role::Pawn, cell: Cell {file: 'f', row: 7}, first_move: true}),
                Box::new(Pawn {name: PieceName::Gpawn, white: false, role: Role::Pawn, cell: Cell {file: 'g', row: 7}, first_move: true}),
                Box::new(Pawn {name: PieceName::Hpawn, white: false, role: Role::Pawn, cell: Cell {file: 'h', row: 7}, first_move: true}),
                // Box::new(Piece {name: PieceName::Qrook, white: false, role: Role::Rook, cell: Cell {file: 'a', row: 8}},
                // Box::new(Piece {name: PieceName::Qknight, white: false, role: Role::Knight, cell: Cell {file: 'b', row: 8}},
                // Box::new(Piece {name: PieceName::Qbishop, white: false, role: Role::Bishop, cell: Cell {file: 'c', row: 8}},
                // Box::new(Piece {name: PieceName::Queen, white: false, role: Role::Queen, cell: Cell {file: 'd', row: 8}},
                // Box::new(Piece {name: PieceName::King, white: false, role: Role::King, cell: Cell {file: 'e', row: 8}},
                // Box::new(Piece {name: PieceName::Kbishop, white: false, role: Role::Bishop, cell: Cell {file: 'f', row: 8}},
                // Box::new(Piece {name: PieceName::Kknight, white: false, role: Role::Knight, cell: Cell {file: 'g', row: 8}},
                // Box::new(Piece {name: PieceName::Krook, white: false, role: Role::Rook, cell: Cell {file: 'h', row: 8}}
            ]
        }
    }

    pub fn get_piece_at_cell(&self, cell: &Cell) -> Option<&Box<dyn Piece>> {
        for piece in self.pieces.iter() {
            if piece.get_curr_cell() == cell {
                return Some(piece);
            }
        }

        return None;
    }

    pub fn get_piece_with_name(&self, name: PieceName, white: bool) -> Option<&Box<dyn Piece>> {
        for piece in self.pieces.iter() {
            if piece.get_name() == name && piece.is_white() == white {
                return Some(piece);
            }
        }

        return None;
    }

    fn get_mut_piece_with_name(&mut self, name: PieceName, white: bool) -> Option<&mut Box<dyn Piece>> {
        for piece in self.pieces.iter_mut() {
            if piece.get_name() == name && piece.is_white() == white {
                return Some(piece);
            }
        }

        return None;
    }

    pub fn get_pieces_with_role(&self, role: Role, white: bool) -> Vec<&Box<dyn Piece>> {
        return self.pieces.iter().filter(|piece| piece.get_role() == role && piece.is_white() == white).collect();
    }

    pub fn move_piece(&mut self, name: PieceName, white: bool, cell: &Cell) {
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
    fn test_move_piece() {
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
}