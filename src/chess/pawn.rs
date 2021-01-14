use super::types::{PieceName, Role};
use super::cell::Cell;
use super::piece::{Piece};
use super::board::Board;
use super::chess_move::Move;

use std::any::Any;

pub struct Pawn {
    pub name: PieceName,
    pub white: bool, 
    pub role: Role,
    pub cell: Cell,
    pub first_move: bool
}

impl Piece for Pawn {
    fn get_name(&self) -> PieceName {self.name}
    fn is_white(&self) -> bool {self.white}
    fn get_role(&self) -> Role {self.role}
    fn get_char_representation(&self) -> char {if self.white {'P'} else {'p'}}
    fn get_curr_cell(&self) -> &Cell {&self.cell}
    fn set_new_cell(&mut self, cell: &Cell) {self.cell = cell.clone()}
    fn has_moved(&self) -> bool {!self.first_move}
    fn set_has_moved(&mut self) {self.first_move = false}

    fn is_valid_move(&self, board: &Board, the_move: &Move) -> bool {
        let valid_cells = self.get_valid_cells(board);
        return valid_cells.contains(&the_move.cell);
    }    
}

impl Pawn {
    pub fn new(white: bool, name: PieceName) -> Pawn {
        Pawn {
            name: name, 
            white: white, 
            role: Role::Pawn, 
            first_move: true, 
            cell: Pawn::init_cell(white, name)
        }
    }

    fn init_cell(white: bool, name: PieceName) -> Cell {
        let mut cell = Cell {file: ' ', row: 0};
        cell.row = if white {2} else {7};
        match name {
            PieceName::Apawn => cell.file = 'a',
            PieceName::Bpawn => cell.file = 'b',
            PieceName::Cpawn => cell.file = 'c',
            PieceName::Dpawn => cell.file = 'd',
            PieceName::Epawn => cell.file = 'e',
            PieceName::Fpawn => cell.file = 'f',
            PieceName::Gpawn => cell.file = 'g',
            PieceName::Hpawn => cell.file = 'h',
            _ => println!("not a pawn")
        }
        cell
    }

    fn add_cell_if_valid(&self, board: &Board, cell: Option<Cell>, is_take: bool, mut valid_cells: Vec<Cell>) -> Vec<Cell> {
        match cell {
            Some(cell) => {
                match board.get_piece_at_cell(&cell) {
                    Some(piece) => {
                        if is_take && piece.is_white() != self.is_white() {
                            valid_cells.push(cell)
                        }
                    },
                    None => {
                        if !is_take {
                            valid_cells.push(cell)
                        }
                    }
                }
            },
            None => ()
    
        }
    
        valid_cells
    }

    fn get_valid_cells(&self, board: &Board) -> Vec<Cell> {
        let mut valid_cells: Vec<Cell> = Vec::new();
        let direction = if self.white {1} else {-1};
        if self.first_move {
            let double_forward_cell_option = Cell::new_from_cell(&self.cell, 0, 2 * direction);
            valid_cells = self.add_cell_if_valid(board, double_forward_cell_option, false, valid_cells);
        }
        else {
            let take_left_cell_option = Cell::new_from_cell(&self.cell, -1, 1 * direction);
            valid_cells = self.add_cell_if_valid(board, take_left_cell_option, true, valid_cells);

            let take_right_cell_option = Cell::new_from_cell(&self.cell, -1, 1 * direction);
            valid_cells = self.add_cell_if_valid(board, take_right_cell_option, true, valid_cells);
        }

        let double_forward_cell_option = Cell::new_from_cell(&self.cell, 0, 1 * direction);
        valid_cells = self.add_cell_if_valid(board, double_forward_cell_option, false, valid_cells);

        valid_cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_simple_white_move() {
        let board = Board::new();
        match board.get_piece_at_cell(&Cell {file: 'a', row: 2}) {
            Some(pawn) => {
                assert!(pawn.is_valid_move(&board, &Move::parse("a3")));
                assert!(pawn.is_valid_move(&board, &Move::parse("a4")));
            },
            None => panic!("expected to find piece")
        }
    }

    #[test]
    fn test_valid_simple_black_move() {
        let board = Board::new();
        match board.get_piece_at_cell(&Cell {file: 'a', row: 7}) {
            Some(pawn) => {
                assert!(pawn.is_valid_move(&board, &Move::parse("a6")));
                assert!(pawn.is_valid_move(&board, &Move::parse("a5")));
            },
            None => panic!("expected to find piece")
        }
    }

    #[test]
    fn test_valid_take_white_and_black_move() {
        let mut board = Board::new();
        board.move_piece(PieceName::Dpawn, true, &Cell::new("d4"));
        board.move_piece(PieceName::Epawn, false, &Cell::new("e5"));
        match board.get_piece_at_cell(&Cell::new("d4")) {
            Some(pawn) => {
                assert!(pawn.is_valid_move(&board, &Move::parse("d5")));
            },
            None => panic!("Expected to find pawn")
        }

        match board.get_piece_at_cell(&Cell::new("e5")) {
            Some(pawn) => {
                assert!(pawn.is_valid_move(&board, &Move::parse("d4")));
            },
            None => panic!("Expected to find pawn")
        }
    }

    #[test]
    fn test_take_same_color_pawn_is_invalid() {
        let mut board = Board::new();
        board.move_piece(PieceName::Dpawn, true, &Cell::new("d3"));
        match board.get_piece_at_cell(&Cell::new("c2")) {
            Some(pawn) => {
                assert!(!pawn.is_valid_move(&board, &Move::parse("d3")));
            },
            None => panic!("Expected to find pawn")
        }
    }
}