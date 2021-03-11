use super::super::types::{PieceName, Role};
use super::super::cell::Cell;
use super::Piece;
use super::super::board::Board;
use super::super::chess_move::Move;


pub struct Bishop {
    pub name: PieceName,
    pub white: bool, 
    pub role: Role,
    pub cell: Cell,
    pub first_move: bool
}

impl Piece for Bishop {
    fn get_name(&self) -> PieceName {self.name}
    fn is_white(&self) -> bool {self.white}
    fn get_role(&self) -> Role {self.role}
    fn get_char_representation(&self) -> char {if self.white {'B'} else {'b'}}
    fn get_curr_cell(&self) -> &Cell {&self.cell}
    fn set_new_cell(&mut self, cell: &Cell) {self.cell = cell.clone()}
    fn has_moved(&self) -> bool {!self.first_move}
    fn set_has_moved(&mut self) {self.first_move = false}

    fn is_valid_move(&self, board: &Board, the_move: &Move) -> bool {
        let valid_cells = self.get_valid_cells(board);
        return valid_cells.contains(&the_move.cell);
    }    
}

impl Bishop {
    pub fn new(white: bool, name: PieceName) -> Bishop {
        Bishop {
            name: name, 
            white: white, 
            role: Role::Bishop, 
            first_move: true, 
            cell: Bishop::init_cell(white, name)
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

    fn attempt_to_add_as_valid_cell(&self, cell_opt: Option<Cell>, board: &Board, valid_cells: &mut Vec<Cell>) -> bool {
        let mut cont = true;
        if let Some(cell) = cell_opt {
            if let Some(piece) = board.get_piece_at_cell(&cell) {
                if piece.is_white() != self.is_white() {
                    valid_cells.push(cell.clone());
                }
                cont = false
            } else {
                valid_cells.push(cell.clone());
            }
        } else {
            cont = false;
        }

        cont
    }

    fn get_valid_cells(&self, board: &Board) -> Vec<Cell> {
        let mut valid_cells: Vec<Cell> = Vec::new();
        let mut stop = false;

        // north east
        for offset in 1..=7 {
            if stop {break;}
            let cell_opt = Cell::new_from_cell(&self.cell, offset, offset);
            stop = !self.attempt_to_add_as_valid_cell(cell_opt, &board, &mut valid_cells);
        } 

        // south east
        stop = false;
        for offset in 1..=7 {
            let reversed_offset = offset * -1;
            if stop {break;}
            let cell_opt = Cell::new_from_cell(&self.cell, offset, reversed_offset);
            stop = !self.attempt_to_add_as_valid_cell(cell_opt, &board, &mut valid_cells);
        } 
        
        // south west
        stop = false;
        for offset in 1..=7 {
            let reversed_offset = offset * -1;
            if stop {break;}
            let cell_opt = Cell::new_from_cell(&self.cell, reversed_offset, reversed_offset);
            stop = !self.attempt_to_add_as_valid_cell(cell_opt, &board, &mut valid_cells);
        } 

        // north west
        stop = false;
        for offset in 1..=7 {
            let reversed_offset = offset * -1;
            if stop {break;}
            let cell_opt = Cell::new_from_cell(&self.cell, reversed_offset, offset);
            stop = !self.attempt_to_add_as_valid_cell(cell_opt, &board, &mut valid_cells);
        } 

        valid_cells
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