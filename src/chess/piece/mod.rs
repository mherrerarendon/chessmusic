pub mod bishop;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod queen;
pub mod rook;

use super::cell::Cell;
use super::types::{Role, PieceName};
use super::board::Board;
use super::chess_move::Move;

pub use bishop::Bishop as Bishop;
pub use king::King as King;
pub use knight::Knight as Knight;
pub use pawn::Pawn as Pawn;
pub use queen::Queen as Queen;
pub use rook::Rook as Rook;


pub trait Piece {
    // common data
    fn get_name(&self) -> PieceName;
    fn is_white(&self) -> bool;
    fn get_role(&self) -> Role;
    fn get_curr_cell(&self) -> &Cell;
    fn get_char_representation(&self) -> char;
    fn set_new_cell(&mut self, cell: &Cell); 

    fn has_moved(&self) -> bool;
    fn set_has_moved(&mut self);

    fn is_valid_move(&self, board: &Board, the_move: &Move) -> bool;
}
