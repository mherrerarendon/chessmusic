use super::cell::Cell;
use super::types::{Role, PieceName};
use super::board::Board;
use super::chess_move::Move;

use std::any::Any;

pub trait Piece {
    // common data
    fn get_name(&self) -> PieceName;
    fn is_white(&self) -> bool;
    fn get_role(&self) -> Role;
    fn get_curr_cell(&self) -> &Cell;
    fn set_new_cell(&mut self, cell: &Cell); 

    fn has_moved(&self) -> bool;
    fn set_has_moved(&mut self);

    fn is_valid_move(&self, board: &Board, the_move: &Move) -> bool;
}

pub fn add_cell_if_valid(board: &Board, cell: Option<Cell>, needs_empty: bool, mut valid_cells: Vec<Cell>) -> Vec<Cell> {
    match cell {
        Some(cell) => {
            match board.get_piece_at_cell(&cell) {
                Some(_piece) => {
                    if !needs_empty {
                        valid_cells.push(cell)
                    }
                },
                None => {
                    if needs_empty {
                        valid_cells.push(cell)
                    }
                }
            }
        },
        None => ()

    }

    valid_cells
}
