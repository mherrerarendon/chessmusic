use std::collections::HashMap;
use std::{error::Error};
use super::types::{Role, PieceName};
use super::cell::Cell;
use super::piece::{NotPawn, Pawn, Piece};


// pub struct Board {
//     pieces: Vec<Piece>,
// }

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
                Box::new(NotPawn {name: PieceName::Qrook, white: true, role: Role::Rook, cell: Cell {file: 'a', row: 1}, first_move: true}),
                // Box::new(Piece {name: PieceName::Qknight, white: true, role: Role::Knight, cell: Cell {file: 'b', row: 1}},
                // Box::new(Piece {name: PieceName::Qbishop, white: true, role: Role::Bishop, cell: Cell {file: 'c', row: 1}},
                // Box::new(Piece {name: PieceName::Queen, white: true, role: Role::Queen, cell: Cell {file: 'd', row: 1}},
                // Box::new(Piece {name: PieceName::King, white: true, role: Role::King, cell: Cell {file: 'e', row: 1}},
                // Box::new(Piece {name: PieceName::Kbishop, white: true, role: Role::Bishop, cell: Cell {file: 'f', row: 1}},
                // Box::new(Piece {name: PieceName::Kknight, white: true, role: Role::Knight, cell: Cell {file: 'g', row: 1}},
                // Box::new(Piece {name: PieceName::Krook, white: true, role: Role::Rook, cell: Cell {file: 'h', row: 1}},
                // Box::new(Piece {name: PieceName::Apawn, white: false, role: Role::Pawn, cell: Cell {file: 'a', row: 7}},
                // Box::new(Piece {name: PieceName::Bpawn, white: false, role: Role::Pawn, cell: Cell {file: 'b', row: 7}},
                // Box::new(Piece {name: PieceName::Cpawn, white: false, role: Role::Pawn, cell: Cell {file: 'c', row: 7}},
                // Box::new(Piece {name: PieceName::Dpawn, white: false, role: Role::Pawn, cell: Cell {file: 'd', row: 7}},
                // Box::new(Piece {name: PieceName::Epawn, white: false, role: Role::Pawn, cell: Cell {file: 'e', row: 7}},
                // Box::new(Piece {name: PieceName::Fpawn, white: false, role: Role::Pawn, cell: Cell {file: 'f', row: 7}},
                // Box::new(Piece {name: PieceName::Gpawn, white: false, role: Role::Pawn, cell: Cell {file: 'g', row: 7}},
                // Box::new(Piece {name: PieceName::Hpawn, white: false, role: Role::Pawn, cell: Cell {file: 'h', row: 7}},
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

    // fn get_valid_cells_for_pawn(&self, piece: Piece) -> Vec<Cell> {
    //     let mut valid_cells: Vec<Cell> = Vec::new();
    //     let direction = if piece.white { 1 } else { -1 };
    //     valid_cells
    // }

    // fn get_valid_cells(piece: &Piece) -> Vec<Cell> {
    //     match piece {
    //         Piece::a1(cell) => vec![Cell {file: 'a', row: 1}],
    //         _ => vec![Cell {file: 'a', row: 1}]
    //     }

    // }
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
}