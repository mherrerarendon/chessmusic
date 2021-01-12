use super::types::{PieceName, Role};
use super::cell::Cell;
use super::piece::{Piece};
use super::board::Board;
use super::chess_move::Move;


pub struct Rook {
    pub name: PieceName,
    pub white: bool, 
    pub role: Role,
    pub cell: Cell,
    pub first_move: bool
}

impl Piece for Rook {
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

impl Rook {
    pub fn new(white: bool, name: PieceName) -> Rook {
        Rook {
            name: name, 
            white: white, 
            role: Role::Rook, 
            first_move: true, 
            cell: Rook::init_cell(white, name)
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
        let mut stop = false;

        // right
        for offset in 1..=7 {
            if stop {break;}
            let cell_opt = Cell::new_from_cell(&self.cell, offset, 0);
            stop = !self.attempt_to_add_as_valid_cell(cell_opt, &board, &mut valid_cells);
        } 

        // left
        stop = false;
        for offset in 1..=7 {
            let reversed_offset = offset * -1;
            if stop {break;}
            let cell_opt = Cell::new_from_cell(&self.cell, reversed_offset, 0);
            stop = !self.attempt_to_add_as_valid_cell(cell_opt, &board, &mut valid_cells);
        } 

        // up
        stop = false;
        for offset in 1..=7 {
            if stop {break;}
            let cell_opt = Cell::new_from_cell(&self.cell, 0, offset);
            stop = !self.attempt_to_add_as_valid_cell(cell_opt, &board, &mut valid_cells);
        }

        // down 
        stop = false;
        for offset in 1..=7 {
            let reversed_offset = offset * -1;
            if stop {break;}
            let cell_opt = Cell::new_from_cell(&self.cell, 0, reversed_offset);
            stop = !self.attempt_to_add_as_valid_cell(cell_opt, &board, &mut valid_cells);
        } 

        valid_cells
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