use super::super::types::{PieceName, Role};
use super::super::cell::Cell;
use super::{Piece, PieceState, PieceStateTrait};
use super::super::board::Board;
use super::super::chess_move::Move;
use super::piece_utils;

pub struct King {
    pub state: PieceState
}

impl Piece for King {
    fn get_state(&self) -> Box<&dyn PieceStateTrait> {Box::new(&self.state)}
    fn get_mut_state(&mut self) -> Box<&mut dyn PieceStateTrait> {Box::new(&mut self.state)}
    fn get_char_representation(&self) -> char {if self.is_white() {'K'} else {'k'}}
    fn is_valid_move(&self, board: &Board, the_move: &Move) -> bool {
        let valid_cells = self.get_valid_cells(board);
        return valid_cells.contains(&the_move.cell);
    }    
}

impl King {
    pub fn new(white: bool) -> King {
        King {
            state: PieceState {
                name: PieceName::King, 
                white: white, 
                role: Role::King, 
                first_move: true, 
                cell: King::init_cell(white)
            }
        }
    }

    fn init_cell(white: bool) -> Cell {
        let row = if white {1} else {8};
        let file = 'e';
        Cell {file: file, row: row}
    }

    fn get_valid_cells(&self, board: &Board) -> Vec<Cell> {
        let mut valid_cells: Vec<Cell> = Vec::new();
        let is_white = self.is_white();
        piece_utils::attempt_to_add_as_valid_cell(Cell::new_from_cell(self.get_curr_cell(), 0, 1), &board, &mut valid_cells, is_white);
        piece_utils::attempt_to_add_as_valid_cell(Cell::new_from_cell(self.get_curr_cell(), 1, 1), &board, &mut valid_cells, is_white);
        piece_utils::attempt_to_add_as_valid_cell(Cell::new_from_cell(self.get_curr_cell(), 1, 0), &board, &mut valid_cells, is_white);
        piece_utils::attempt_to_add_as_valid_cell(Cell::new_from_cell(self.get_curr_cell(), 1, -1), &board, &mut valid_cells, is_white);
        piece_utils::attempt_to_add_as_valid_cell(Cell::new_from_cell(self.get_curr_cell(), 0, -1), &board, &mut valid_cells, is_white);
        piece_utils::attempt_to_add_as_valid_cell(Cell::new_from_cell(self.get_curr_cell(), -1, -1), &board, &mut valid_cells, is_white);
        piece_utils::attempt_to_add_as_valid_cell(Cell::new_from_cell(self.get_curr_cell(), -1, 0), &board, &mut valid_cells, is_white);
        piece_utils::attempt_to_add_as_valid_cell(Cell::new_from_cell(self.get_curr_cell(), -1, 1), &board, &mut valid_cells, is_white);

        valid_cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_white_king_moves() {
        let board = Board::new_king_test();
        match board.get_piece_at_cell(&Cell::new("e1")) {
            Some(king) => {
                // self 
                assert!(!king.is_valid_move(&board, &Move::parse("e1")));

                assert!(king.is_valid_move(&board, &Move::parse("e2")));
                assert!(king.is_valid_move(&board, &Move::parse("f2")));
                assert!(king.is_valid_move(&board, &Move::parse("f1")));
                assert!(king.is_valid_move(&board, &Move::parse("d1")));
                assert!(king.is_valid_move(&board, &Move::parse("d2")));
                

                // invalid
                assert!(!king.is_valid_move(&board, &Move::parse("c3")));
                assert!(!king.is_valid_move(&board, &Move::parse("e3")));
            },
            None => panic!("expected to find piece")
        }
    }

    #[test]
    fn test_black_king_moves() {
        let board = Board::new_king_test();
        match board.get_piece_at_cell(&Cell::new("e8")) {
            Some(king) => {
                // self 
                assert!(!king.is_valid_move(&board, &Move::parse("e8")));

                assert!(king.is_valid_move(&board, &Move::parse("d8")));
                assert!(king.is_valid_move(&board, &Move::parse("f8")));
                assert!(king.is_valid_move(&board, &Move::parse("d7")));
                assert!(king.is_valid_move(&board, &Move::parse("e7")));
                assert!(king.is_valid_move(&board, &Move::parse("f7")));

                // invalid
                assert!(!king.is_valid_move(&board, &Move::parse("c6")));
                assert!(!king.is_valid_move(&board, &Move::parse("e6")));
            },
            None => panic!("expected to find piece")
        }
    }
}