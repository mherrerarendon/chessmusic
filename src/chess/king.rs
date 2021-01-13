use super::types::{PieceName, Role};
use super::cell::Cell;
use super::piece::{Piece};
use super::board::Board;
use super::chess_move::Move;


pub struct King {
    pub name: PieceName,
    pub white: bool, 
    pub role: Role,
    pub cell: Cell,
    pub first_move: bool
}

impl Piece for King {
    fn get_name(&self) -> PieceName {self.name}
    fn is_white(&self) -> bool {self.white}
    fn get_role(&self) -> Role {self.role}
    fn get_curr_cell(&self) -> &Cell {&self.cell}
    fn set_new_cell(&mut self, cell: &Cell) {self.cell = cell.clone()}
    fn has_moved(&self) -> bool {!self.first_move}
    fn set_has_moved(&mut self) {self.first_move = false}

    fn is_valid_move(&self, board: &Board, the_move: &Move) -> bool {
        let valid_cells = self.get_valid_cells(board);
        return valid_cells.contains(&the_move.cell);
    }    
}

impl King {
    pub fn new(white: bool) -> King {
        King {
            name: PieceName::King, 
            white: white, 
            role: Role::King, 
            first_move: true, 
            cell: King::init_cell(white)
        }
    }

    fn init_cell(white: bool) -> Cell {
        let row = if white {1} else {8};
        let file = 'e';
        Cell {file: file, row: row}
    }

    fn attempt_to_add_as_valid_cell(&self, cell_opt: Option<Cell>, board: &Board, valid_cells: &mut Vec<Cell>) -> bool {
        let mut cont = true;
        match cell_opt {
            Some(cell) => {
                match board.get_piece_at_cell(&cell) {
                    Some(piece) => {
                        if piece.is_white() != self.is_white() {
                            valid_cells.push(cell.clone());
                        }
                        cont = false
                    },
                    None => {
                        valid_cells.push(cell.clone());
                    }
                }
            },
            None => cont = false
    
        }

        cont
    }

    fn get_valid_cells(&self, board: &Board) -> Vec<Cell> {
        let mut valid_cells: Vec<Cell> = Vec::new();
        
        self.attempt_to_add_as_valid_cell(Cell::new_from_cell(&self.cell, 0, 1), &board, &mut valid_cells);
        self.attempt_to_add_as_valid_cell(Cell::new_from_cell(&self.cell, 1, 1), &board, &mut valid_cells);
        self.attempt_to_add_as_valid_cell(Cell::new_from_cell(&self.cell, 1, 0), &board, &mut valid_cells);
        self.attempt_to_add_as_valid_cell(Cell::new_from_cell(&self.cell, 1, -1), &board, &mut valid_cells);
        self.attempt_to_add_as_valid_cell(Cell::new_from_cell(&self.cell, 0, -1), &board, &mut valid_cells);
        self.attempt_to_add_as_valid_cell(Cell::new_from_cell(&self.cell, -1, -1), &board, &mut valid_cells);
        self.attempt_to_add_as_valid_cell(Cell::new_from_cell(&self.cell, -1, 0), &board, &mut valid_cells);
        self.attempt_to_add_as_valid_cell(Cell::new_from_cell(&self.cell, -1, 1), &board, &mut valid_cells);

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