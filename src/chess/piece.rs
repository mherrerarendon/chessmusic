use super::cell::Cell;
use super::types::{Role, PieceName};
use super::board::Board;
use super::chess_move::Move;

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
