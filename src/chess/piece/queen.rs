use super::super::types::{PieceName, Role};
use super::super::cell::Cell;
use super::Piece;
use super::super::board::Board;
use super::super::chess_move::Move;


pub struct Queen {
    pub name: PieceName,
    pub white: bool, 
    pub role: Role,
    pub cell: Cell,
    pub first_move: bool
}

impl Piece for Queen {
    fn get_name(&self) -> PieceName {self.name}
    fn is_white(&self) -> bool {self.white}
    fn get_role(&self) -> Role {self.role}
    fn get_char_representation(&self) -> char {if self.white {'Q'} else {'q'}}
    fn get_curr_cell(&self) -> &Cell {&self.cell}
    fn set_new_cell(&mut self, cell: &Cell) {self.cell = cell.clone()}
    fn has_moved(&self) -> bool {!self.first_move}
    fn set_has_moved(&mut self) {self.first_move = false}

    fn is_valid_move(&self, board: &Board, the_move: &Move) -> bool {
        let valid_cells = self.get_valid_cells(board);
        return valid_cells.contains(&the_move.cell);
    }    
}

impl Queen {
    pub fn new(white: bool) -> Queen {
        Queen {
            name: PieceName::Queen, 
            white: white, 
            role: Role::Queen, 
            first_move: true, 
            cell: Queen::init_cell(white)
        }
    }

    fn init_cell(white: bool) -> Cell {
        let row = if white {1} else {8};
        let file = 'd';
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

        // bishop moves
        for offset in 1..=7 {
            if stop {break;}
            let cell_opt = Cell::new_from_cell(&self.cell, offset, offset);
            stop = !self.attempt_to_add_as_valid_cell(cell_opt, &board, &mut valid_cells);
        } 
        stop = false;
        for offset in 1..=7 {
            let reversed_offset = offset * -1;
            if stop {break;}
            let cell_opt = Cell::new_from_cell(&self.cell, offset, reversed_offset);
            stop = !self.attempt_to_add_as_valid_cell(cell_opt, &board, &mut valid_cells);
        } 
        stop = false;
        for offset in 1..=7 {
            let reversed_offset = offset * -1;
            if stop {break;}
            let cell_opt = Cell::new_from_cell(&self.cell, reversed_offset, reversed_offset);
            stop = !self.attempt_to_add_as_valid_cell(cell_opt, &board, &mut valid_cells);
        } 
        stop = false;
        for offset in 1..=7 {
            let reversed_offset = offset * -1;
            if stop {break;}
            let cell_opt = Cell::new_from_cell(&self.cell, reversed_offset, offset);
            stop = !self.attempt_to_add_as_valid_cell(cell_opt, &board, &mut valid_cells);
        } 

        // rook moves
        stop = false;
        for offset in 1..=7 {
            if stop {break;}
            let cell_opt = Cell::new_from_cell(&self.cell, offset, 0);
            stop = !self.attempt_to_add_as_valid_cell(cell_opt, &board, &mut valid_cells);
        } 
        stop = false;
        for offset in 1..=7 {
            let reversed_offset = offset * -1;
            if stop {break;}
            let cell_opt = Cell::new_from_cell(&self.cell, reversed_offset, 0);
            stop = !self.attempt_to_add_as_valid_cell(cell_opt, &board, &mut valid_cells);
        } 
        stop = false;
        for offset in 1..=7 {
            if stop {break;}
            let cell_opt = Cell::new_from_cell(&self.cell, 0, offset);
            stop = !self.attempt_to_add_as_valid_cell(cell_opt, &board, &mut valid_cells);
        }
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
    fn test_white_queen_moves() {
        let board = Board::new_queen_test();
        match board.get_piece_at_cell(&Cell::new("d1")) {
            Some(queen) => {
                // self 
                assert!(!queen.is_valid_move(&board, &Move::parse("d1")));

                // up
                assert!(queen.is_valid_move(&board, &Move::parse("d2")));
                assert!(queen.is_valid_move(&board, &Move::parse("d3")));
                assert!(queen.is_valid_move(&board, &Move::parse("d4")));
                assert!(queen.is_valid_move(&board, &Move::parse("d5")));
                assert!(queen.is_valid_move(&board, &Move::parse("d6")));
                assert!(queen.is_valid_move(&board, &Move::parse("d7")));
                assert!(queen.is_valid_move(&board, &Move::parse("d8")));

                // left
                assert!(queen.is_valid_move(&board, &Move::parse("c1")));
                assert!(queen.is_valid_move(&board, &Move::parse("b1")));
                assert!(queen.is_valid_move(&board, &Move::parse("a1")));

                // right
                assert!(queen.is_valid_move(&board, &Move::parse("e1")));
                assert!(queen.is_valid_move(&board, &Move::parse("f1")));
                assert!(queen.is_valid_move(&board, &Move::parse("g1")));
                assert!(queen.is_valid_move(&board, &Move::parse("h1")));

                // north east
                assert!(queen.is_valid_move(&board, &Move::parse("e2")));
                assert!(queen.is_valid_move(&board, &Move::parse("f3")));
                assert!(queen.is_valid_move(&board, &Move::parse("g4")));
                assert!(queen.is_valid_move(&board, &Move::parse("h5")));

                // north west
                assert!(queen.is_valid_move(&board, &Move::parse("c2")));
                assert!(queen.is_valid_move(&board, &Move::parse("b3")));
                assert!(queen.is_valid_move(&board, &Move::parse("a4")));

                // invalid
                assert!(!queen.is_valid_move(&board, &Move::parse("c3")));
                assert!(!queen.is_valid_move(&board, &Move::parse("e3")));
            },
            None => panic!("expected to find piece")
        }
    }

    #[test]
    fn test_black_queen_moves() {
        let board = Board::new_queen_test();
        match board.get_piece_at_cell(&Cell::new("d8")) {
            Some(queen) => {
                // self 
                assert!(!queen.is_valid_move(&board, &Move::parse("d8")));

                // down
                assert!(queen.is_valid_move(&board, &Move::parse("d7")));
                assert!(queen.is_valid_move(&board, &Move::parse("d6")));
                assert!(queen.is_valid_move(&board, &Move::parse("d5")));
                assert!(queen.is_valid_move(&board, &Move::parse("d4")));
                assert!(queen.is_valid_move(&board, &Move::parse("d3")));
                assert!(queen.is_valid_move(&board, &Move::parse("d2")));
                assert!(queen.is_valid_move(&board, &Move::parse("d1")));

                // left
                assert!(queen.is_valid_move(&board, &Move::parse("c8")));
                assert!(queen.is_valid_move(&board, &Move::parse("b8")));
                assert!(queen.is_valid_move(&board, &Move::parse("a8")));

                // right
                assert!(queen.is_valid_move(&board, &Move::parse("e8")));
                assert!(queen.is_valid_move(&board, &Move::parse("f8")));
                assert!(queen.is_valid_move(&board, &Move::parse("g8")));
                assert!(queen.is_valid_move(&board, &Move::parse("h8")));

                // south east
                assert!(queen.is_valid_move(&board, &Move::parse("e7")));
                assert!(queen.is_valid_move(&board, &Move::parse("f6")));
                assert!(queen.is_valid_move(&board, &Move::parse("g5")));
                assert!(queen.is_valid_move(&board, &Move::parse("h4")));

                // south west
                assert!(queen.is_valid_move(&board, &Move::parse("c7")));
                assert!(queen.is_valid_move(&board, &Move::parse("b6")));
                assert!(queen.is_valid_move(&board, &Move::parse("a5")));

                // invalid
                assert!(!queen.is_valid_move(&board, &Move::parse("c6")));
                assert!(!queen.is_valid_move(&board, &Move::parse("e6")));
            },
            None => panic!("expected to find piece")
        }
    }
}