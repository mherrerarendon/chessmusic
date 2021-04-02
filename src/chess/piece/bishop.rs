use super::super::types::{PieceName, Role};
use super::super::cell::Cell;
use super::{Piece, PieceState, PieceStateTrait};
use super::super::board::Board;
use super::super::chess_move::Move;
use super::piece_utils;


pub struct Bishop {
    pub state: PieceState
}

impl Piece for Bishop {
    fn get_state(&self) -> Box<&dyn PieceStateTrait> {Box::new(&self.state)}
    fn get_mut_state(&mut self) -> Box<&mut dyn PieceStateTrait> {Box::new(&mut self.state)}
    fn get_char_representation(&self) -> char {if self.is_white() {'B'} else {'b'}}
    fn is_valid_move(&self, board: &Board, the_move: &Move) -> bool {
        let valid_cells = self.get_valid_cells(board);
        return valid_cells.contains(&the_move.cell);
    }    
}

impl Bishop {
    pub fn new(white: bool, name: PieceName) -> Bishop {
        Bishop {
            state: PieceState {
                name: name, 
                white: white, 
                role: Role::Bishop, 
                first_move: true, 
                cell: Bishop::init_cell(white, name), 
                cell_history: Vec::new()
            }
        }
    }

    fn init_cell(white: bool, name: PieceName) -> Cell {
        let row = if white {1} else {8};
        let file = match name {
            PieceName::Kbishop => 'f',
            PieceName::Qbishop => 'c',
            _ => panic!("Not a bishop")
        };
        Cell {file: file, row: row}
    }

    pub fn valid_bishop_cells(board: &Board, curr_cell: &Cell) -> Vec<Cell> {
        let white = board.get_piece_at_cell(curr_cell).unwrap().is_white();
        let mut valid_cells: Vec<Cell> = Vec::new();
        let mut stop = false;

        // north east
        for offset in 1..=7 {
            if stop {break;}
            let cell_opt = Cell::new_from_cell(curr_cell, offset, offset);
            stop = !piece_utils::attempt_to_add_as_valid_cell(cell_opt, &board, &mut valid_cells, white);
        } 

        // south east
        stop = false;
        for offset in 1..=7 {
            let reversed_offset = offset * -1;
            if stop {break;}
            let cell_opt = Cell::new_from_cell(curr_cell, offset, reversed_offset);
            stop = !piece_utils::attempt_to_add_as_valid_cell(cell_opt, &board, &mut valid_cells, white);
        } 
        
        // south west
        stop = false;
        for offset in 1..=7 {
            let reversed_offset = offset * -1;
            if stop {break;}
            let cell_opt = Cell::new_from_cell(curr_cell, reversed_offset, reversed_offset);
            stop = !piece_utils::attempt_to_add_as_valid_cell(cell_opt, &board, &mut valid_cells, white);
        } 

        // north west
        stop = false;
        for offset in 1..=7 {
            let reversed_offset = offset * -1;
            if stop {break;}
            let cell_opt = Cell::new_from_cell(curr_cell, reversed_offset, offset);
            stop = !piece_utils::attempt_to_add_as_valid_cell(cell_opt, &board, &mut valid_cells, white);
        } 

        valid_cells
    }

    fn get_valid_cells(&self, board: &Board) -> Vec<Cell> {
        Bishop::valid_bishop_cells(board, self.get_curr_cell())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_white_q_bishop_moves() {
        let board = Board::new_bishop_test();
        match board.get_piece_at_cell(&Cell::new("c1")) {
            Some(bishop) => {
                // self 
                assert!(!bishop.is_valid_move(&board, &Move::parse("c1")));

                // valid
                assert!(bishop.is_valid_move(&board, &Move::parse("d2")));
                assert!(bishop.is_valid_move(&board, &Move::parse("e3")));
                assert!(bishop.is_valid_move(&board, &Move::parse("f4")));
                assert!(bishop.is_valid_move(&board, &Move::parse("g5")));
                assert!(bishop.is_valid_move(&board, &Move::parse("h6")));

                assert!(bishop.is_valid_move(&board, &Move::parse("b2")));
                assert!(bishop.is_valid_move(&board, &Move::parse("a3")));

                // invalid
                assert!(!bishop.is_valid_move(&board, &Move::parse("c2")));
                assert!(!bishop.is_valid_move(&board, &Move::parse("c3")));

            },
            None => panic!("expected to find piece")
        }
    }

    #[test]
    fn test_black_k_bishop_moves() {
        let board = Board::new_bishop_test();
        match board.get_piece_at_cell(&Cell::new("f8")) {
            Some(bishop) => {
                // self 
                assert!(!bishop.is_valid_move(&board, &Move::parse("f8")));

                // valid
                assert!(bishop.is_valid_move(&board, &Move::parse("e7")));
                assert!(bishop.is_valid_move(&board, &Move::parse("d6")));
                assert!(bishop.is_valid_move(&board, &Move::parse("c5")));
                assert!(bishop.is_valid_move(&board, &Move::parse("b4")));
                assert!(bishop.is_valid_move(&board, &Move::parse("a3")));

                assert!(bishop.is_valid_move(&board, &Move::parse("g7")));
                assert!(bishop.is_valid_move(&board, &Move::parse("h6")));

                // invalid
                assert!(!bishop.is_valid_move(&board, &Move::parse("f7")));
                assert!(!bishop.is_valid_move(&board, &Move::parse("f6")));

            },
            None => panic!("expected to find piece")
        }
    }
}