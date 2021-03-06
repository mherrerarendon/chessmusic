use super::super::types::{PieceName, Role};
use super::super::cell::Cell;
use super::{Piece, PieceState, PieceStateTrait};
use super::super::board::Board;
use super::super::chess_move::Move;
use super::piece_utils;

pub struct Rook {
    pub state: PieceState
}

impl Piece for Rook {
    fn get_state(&self) -> Box<&dyn PieceStateTrait> {Box::new(&self.state)}
    fn get_mut_state(&mut self) -> Box<&mut dyn PieceStateTrait> {Box::new(&mut self.state)}
    fn get_char_representation(&self) -> char {if self.is_white() {'R'} else {'r'}}
    fn is_valid_move(&self, board: &Board, the_move: &Move) -> bool {
        let valid_cells = self.get_valid_cells(board);
        return valid_cells.contains(&the_move.cell);
    }
    fn first_cell(&self) -> Cell {
        Rook::init_cell(self.is_white(), self.get_name())
    }
}

impl Rook {
    pub fn new(white: bool, name: PieceName) -> Rook {
        Rook {
            state: PieceState {
                name: name, 
                white: white, 
                role: Role::Rook, 
                cell: Some(Rook::init_cell(white, name)),
                move_history: Vec::new()
            }
        }
    }

    fn init_cell(white: bool, name: PieceName) -> Cell {
        let row = if white {1} else {8};
        let file = match name {
            PieceName::Krook => 'h',
            PieceName::Qrook => 'a',
            _ => panic!("Not a rook")
        };
        Cell {file: file, row: row}
    }

    pub fn valid_rook_cells(board: &Board, curr_cell: &Cell) -> Vec<Cell> {
        let white = board.get_piece_at_cell(curr_cell).unwrap().is_white();
        let mut valid_cells: Vec<Cell> = Vec::new();
        let mut stop = false;

        // right
        for offset in 1..=7 {
            if stop {break;}
            let cell_opt = Cell::new_from_cell(curr_cell, offset, 0);
            stop = !piece_utils::attempt_to_add_as_valid_cell(cell_opt, &board, &mut valid_cells, white);
        } 

        // left
        stop = false;
        for offset in 1..=7 {
            let reversed_offset = offset * -1;
            if stop {break;}
            let cell_opt = Cell::new_from_cell(curr_cell, reversed_offset, 0);
            stop = !piece_utils::attempt_to_add_as_valid_cell(cell_opt, &board, &mut valid_cells, white);
        } 

        // up
        stop = false;
        for offset in 1..=7 {
            if stop {break;}
            let cell_opt = Cell::new_from_cell(curr_cell, 0, offset);
            stop = !piece_utils::attempt_to_add_as_valid_cell(cell_opt, &board, &mut valid_cells, white);
        }

        // down 
        stop = false;
        for offset in 1..=7 {
            let reversed_offset = offset * -1;
            if stop {break;}
            let cell_opt = Cell::new_from_cell(curr_cell, 0, reversed_offset);
            stop = !piece_utils::attempt_to_add_as_valid_cell(cell_opt, &board, &mut valid_cells, white);
        } 

        valid_cells
    }

    fn get_valid_cells(&self, board: &Board) -> Vec<Cell> {
        Rook::valid_rook_cells(board, &self.get_curr_cell().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_white_q_rook_moves() {
        let board = Board::new_rook_test();
        match board.get_piece_at_cell(&Cell {file: 'a', row: 1}) {
            Some(rook) => {
                // self rook is at a1, so should be invalid
                assert!(!rook.is_valid_move(&board, &Move::parse("a1")));

                // valid
                assert!(rook.is_valid_move(&board, &Move::parse("b1")));
                assert!(rook.is_valid_move(&board, &Move::parse("c1")));
                assert!(rook.is_valid_move(&board, &Move::parse("d1")));
                assert!(rook.is_valid_move(&board, &Move::parse("e1")));
                assert!(rook.is_valid_move(&board, &Move::parse("f1")));
                assert!(rook.is_valid_move(&board, &Move::parse("g1")));

                // Other rook is at h1, so this should be an invalid move
                assert!(!rook.is_valid_move(&board, &Move::parse("h1")));

                assert!(rook.is_valid_move(&board, &Move::parse("a2")));
                assert!(rook.is_valid_move(&board, &Move::parse("a3")));
                assert!(rook.is_valid_move(&board, &Move::parse("a4")));
                assert!(rook.is_valid_move(&board, &Move::parse("a5")));
                assert!(rook.is_valid_move(&board, &Move::parse("a6")));
                assert!(rook.is_valid_move(&board, &Move::parse("a7")));
                assert!(rook.is_valid_move(&board, &Move::parse("a8")));

            },
            None => panic!("expected to find piece")
        }
    }

    #[test]
    fn test_white_k_rook_moves() {
        let board = Board::new_rook_test();
        match board.get_piece_at_cell(&Cell {file: 'h', row: 1}) {
            Some(rook) => {
                // other rook is at a1, so should be invalid
                assert!(!rook.is_valid_move(&board, &Move::parse("a1")));

                // valid
                assert!(rook.is_valid_move(&board, &Move::parse("b1")));
                assert!(rook.is_valid_move(&board, &Move::parse("c1")));
                assert!(rook.is_valid_move(&board, &Move::parse("d1")));
                assert!(rook.is_valid_move(&board, &Move::parse("e1")));
                assert!(rook.is_valid_move(&board, &Move::parse("f1")));
                assert!(rook.is_valid_move(&board, &Move::parse("g1")));

                // self rook is at h1, so this should be an invalid move
                assert!(!rook.is_valid_move(&board, &Move::parse("h1")));
            },
            None => panic!("expected to find piece")
        }
    }

    #[test]
    fn test_black_q_rook_moves() {
        let board = Board::new_rook_test();
        match board.get_piece_at_cell(&Cell {file: 'a', row: 8}) {
            Some(rook) => {
                // self rook is at a1, so should be invalid
                assert!(!rook.is_valid_move(&board, &Move::parse("a8")));

                // valid
                assert!(rook.is_valid_move(&board, &Move::parse("b8")));
                assert!(rook.is_valid_move(&board, &Move::parse("c8")));
                assert!(rook.is_valid_move(&board, &Move::parse("d8")));
                assert!(rook.is_valid_move(&board, &Move::parse("e8")));
                assert!(rook.is_valid_move(&board, &Move::parse("f8")));
                assert!(rook.is_valid_move(&board, &Move::parse("g8")));

                // Other rook is at h1, so this should be an invalid move
                assert!(!rook.is_valid_move(&board, &Move::parse("h8")));

                assert!(rook.is_valid_move(&board, &Move::parse("a1")));
                assert!(rook.is_valid_move(&board, &Move::parse("a2")));
                assert!(rook.is_valid_move(&board, &Move::parse("a3")));
                assert!(rook.is_valid_move(&board, &Move::parse("a4")));
                assert!(rook.is_valid_move(&board, &Move::parse("a5")));
                assert!(rook.is_valid_move(&board, &Move::parse("a6")));
                assert!(rook.is_valid_move(&board, &Move::parse("a7")));

            },
            None => panic!("expected to find piece")
        }
    }
}