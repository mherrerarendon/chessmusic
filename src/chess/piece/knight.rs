use super::super::types::{PieceName, Role};
use super::super::cell::Cell;
use super::{Piece, PieceState, PieceStateTrait};
use super::super::board::Board;
use super::super::chess_move::Move;
use super::piece_utils;

pub struct Knight {
    pub state: PieceState
}

impl Piece for Knight {
    fn get_state(&self) -> Box<&dyn PieceStateTrait> {Box::new(&self.state)}
    fn get_mut_state(&mut self) -> Box<&mut dyn PieceStateTrait> {Box::new(&mut self.state)}
    fn get_char_representation(&self) -> char {if self.is_white() {'N'} else {'n'}}
    fn is_valid_move(&self, board: &Board, the_move: &Move) -> bool {
        let valid_cells = self.get_valid_cells(board);
        return valid_cells.contains(&the_move.cell);
    }    
}

impl Knight {
    pub fn new(white: bool, name: PieceName) -> Knight {
        Knight {
            state: PieceState {
                name: name, 
                white: white, 
                role: Role::Knight, 
                first_move: true, 
                cell: Knight::init_cell(white, name),
                cell_history: Vec::new()
            }
        }
    }

    fn init_cell(white: bool, name: PieceName) -> Cell {
        let row = if white {1} else {8};
        let file = match name {
            PieceName::Kknight => 'g',
            PieceName::Qknight => 'b',
            _ => panic!("Not a knight")
        };
        Cell {file: file, row: row}
    }

    fn get_valid_cells(&self, board: &Board) -> Vec<Cell> {
        let mut valid_cells: Vec<Cell> = Vec::new();
        let is_white = self.is_white();
        piece_utils::attempt_to_add_as_valid_cell(Cell::new_from_cell(self.get_curr_cell(), 1, 2), &board, &mut valid_cells, is_white);
        piece_utils::attempt_to_add_as_valid_cell(Cell::new_from_cell(self.get_curr_cell(), 2, 1), &board, &mut valid_cells, is_white);
        piece_utils::attempt_to_add_as_valid_cell(Cell::new_from_cell(self.get_curr_cell(), 2, -1), &board, &mut valid_cells, is_white);
        piece_utils::attempt_to_add_as_valid_cell(Cell::new_from_cell(self.get_curr_cell(), 1, -2), &board, &mut valid_cells, is_white);
        piece_utils::attempt_to_add_as_valid_cell(Cell::new_from_cell(self.get_curr_cell(), -1, -2), &board, &mut valid_cells, is_white);
        piece_utils::attempt_to_add_as_valid_cell(Cell::new_from_cell(self.get_curr_cell(), -2, -1), &board, &mut valid_cells, is_white);
        piece_utils::attempt_to_add_as_valid_cell(Cell::new_from_cell(self.get_curr_cell(), -2, 1), &board, &mut valid_cells, is_white);
        piece_utils::attempt_to_add_as_valid_cell(Cell::new_from_cell(self.get_curr_cell(), -1, 2), &board, &mut valid_cells, is_white);

        valid_cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_white_q_knight_moves() {
        let mut board = Board::new();
        
        match board.get_piece_at_cell(&Cell::new("b1")) {
            Some(knight) => {
                assert!(knight.is_valid_move(&board, &Move::parse("a3")));
                assert!(knight.is_valid_move(&board, &Move::parse("c3")));
            },
            None => panic!("expected to find piece")
        }

        let new_cell = Cell::new("a3");
        board.move_piece(PieceName::Qknight, true, &new_cell);
        match board.get_piece_at_cell(&new_cell) {
            Some(knight) => {
                assert!(knight.is_valid_move(&board, &Move::parse("c4")));
                assert!(knight.is_valid_move(&board, &Move::parse("b1")));
                assert!(!knight.is_valid_move(&board, &Move::parse("c3")));
            },
            None => panic!("expected to find piece")
        }
    }

    #[test]
    fn test_black_k_knight_moves() {
        let mut board = Board::new();
        
        match board.get_piece_at_cell(&Cell::new("g8")) {
            Some(knight) => {
                assert!(knight.is_valid_move(&board, &Move::parse("h6")));
                assert!(knight.is_valid_move(&board, &Move::parse("f6")));
            },
            None => panic!("expected to find piece")
        }

        let new_cell = Cell::new("h6");
        board.move_piece(PieceName::Kknight, false, &new_cell);
        match board.get_piece_at_cell(&new_cell) {
            Some(knight) => {
                assert!(knight.is_valid_move(&board, &Move::parse("g8")));
                assert!(knight.is_valid_move(&board, &Move::parse("f5")));
                assert!(!knight.is_valid_move(&board, &Move::parse("g6")));
            },
            None => panic!("expected to find piece")
        }
    }
}