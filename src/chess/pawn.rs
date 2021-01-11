use super::types::{PieceName, Role};
use super::cell::Cell;
use super::piece::{Piece, add_cell_if_valid};
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
    fn new(white: bool, name: PieceName) -> Pawn {
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

    fn get_valid_cells(&self, board: &Board) -> Vec<Cell> {
        let mut valid_cells: Vec<Cell> = Vec::new();
        let direction = if self.white {1} else {-1};
        if self.first_move {
            let double_forward_cell_option = Cell::new_from_cell(&self.cell, 0, 2 * direction);
            valid_cells = add_cell_if_valid(board, double_forward_cell_option, true, valid_cells);
        }
        else {
            let take_left_cell_option = Cell::new_from_cell(&self.cell, -1, 1 * direction);
            valid_cells = add_cell_if_valid(board, take_left_cell_option, false, valid_cells);

            let take_right_cell_option = Cell::new_from_cell(&self.cell, -1, 1 * direction);
            valid_cells = add_cell_if_valid(board, take_right_cell_option, false, valid_cells);
        }

        let double_forward_cell_option = Cell::new_from_cell(&self.cell, 0, 1 * direction);
        valid_cells = add_cell_if_valid(board, double_forward_cell_option, true, valid_cells);

        valid_cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::game::Game;

    #[test]
    fn test_valid_simple_white_move() {
        let board = Board::new();
        match board.get_piece_at_cell(&Cell {file: 'a', row: 2}) {
            Some(pawn) => {
                assert!(pawn.is_valid_move(&board, &Move::parse("a3")));
                assert!(pawn.is_valid_move(&board, &Move::parse("a4")));
            },
            None => ()
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
            None => ()
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
}