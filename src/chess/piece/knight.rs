use super::super::types::{PieceName, Role};
use super::super::cell::Cell;
use super::Piece;
use super::super::board::Board;
use super::super::chess_move::Move;


pub struct Knight {
    pub name: PieceName,
    pub white: bool, 
    pub role: Role,
    pub cell: Cell,
    pub first_move: bool
}

impl Piece for Knight {
    fn get_name(&self) -> PieceName {self.name}
    fn is_white(&self) -> bool {self.white}
    fn get_role(&self) -> Role {self.role}
    fn get_char_representation(&self) -> char {if self.white {'N'} else {'n'}}
    fn get_curr_cell(&self) -> &Cell {&self.cell}
    fn set_new_cell(&mut self, cell: &Cell) {self.cell = cell.clone()}
    fn has_moved(&self) -> bool {!self.first_move}
    fn set_has_moved(&mut self) {self.first_move = false}

    fn is_valid_move(&self, board: &Board, the_move: &Move) -> bool {
        let valid_cells = self.get_valid_cells(board);
        return valid_cells.contains(&the_move.cell);
    }    
}

impl Knight {
    pub fn new(white: bool, name: PieceName) -> Knight {
        Knight {
            name: name, 
            white: white, 
            role: Role::Knight, 
            first_move: true, 
            cell: Knight::init_cell(white, name)
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

    fn attempt_to_add_as_valid_cell(&self, cell_opt: Option<Cell>, board: &Board, valid_cells: &mut Vec<Cell>) {
        match cell_opt {
            Some(cell) => {
                match board.get_piece_at_cell(&cell) {
                    Some(piece) => {
                        if piece.is_white() != self.is_white() {
                            valid_cells.push(cell.clone());
                        }
                    },
                    None => {
                        valid_cells.push(cell.clone());
                    }
                }
            },
            None => ()
    
        }
    }

    fn get_valid_cells(&self, board: &Board) -> Vec<Cell> {
        let mut valid_cells: Vec<Cell> = Vec::new();

        self.attempt_to_add_as_valid_cell(Cell::new_from_cell(&self.cell, 1, 2), &board, &mut valid_cells);
        self.attempt_to_add_as_valid_cell(Cell::new_from_cell(&self.cell, 2, 1), &board, &mut valid_cells);
        self.attempt_to_add_as_valid_cell(Cell::new_from_cell(&self.cell, 2, -1), &board, &mut valid_cells);
        self.attempt_to_add_as_valid_cell(Cell::new_from_cell(&self.cell, 1, -2), &board, &mut valid_cells);
        self.attempt_to_add_as_valid_cell(Cell::new_from_cell(&self.cell, -1, -2), &board, &mut valid_cells);
        self.attempt_to_add_as_valid_cell(Cell::new_from_cell(&self.cell, -2, -1), &board, &mut valid_cells);
        self.attempt_to_add_as_valid_cell(Cell::new_from_cell(&self.cell, -2, 1), &board, &mut valid_cells);
        self.attempt_to_add_as_valid_cell(Cell::new_from_cell(&self.cell, -1, 2), &board, &mut valid_cells);

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