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

pub struct PieceState {
    pub name: PieceName,
    pub white: bool, 
    pub role: Role,
    pub cell: Cell,
    pub first_move: bool
}

impl PieceStateTrait for PieceState {
    fn get_name(&self) -> PieceName {self.name}
    fn is_white(&self) -> bool {self.white}
    fn get_role(&self) -> Role {self.role}
    fn get_curr_cell(&self) -> &Cell {&self.cell}
    fn set_new_cell(&mut self, cell: &Cell) {
        self.cell = cell.clone();
        self.first_move = false;
    }
    fn has_moved(&self) -> bool {!self.first_move}
}

pub trait PieceStateTrait {
    fn get_name(&self) -> PieceName;
    fn is_white(&self) -> bool;
    fn get_role(&self) -> Role;
    fn get_curr_cell(&self) -> &Cell;
    fn set_new_cell(&mut self, cell: &Cell); 
    fn has_moved(&self) -> bool;
}


pub trait Piece {
    // common data
    fn get_name(&self) -> PieceName {self.get_state().get_name()}
    fn is_white(&self) -> bool {self.get_state().is_white()}
    fn get_role(&self) -> Role {self.get_state().get_role()}
    fn get_curr_cell(&self) -> &Cell {self.get_state().get_curr_cell()}
    fn has_moved(&self) -> bool {self.get_state().has_moved()}
    fn set_new_cell(&mut self, cell: &Cell) {self.get_mut_state().set_new_cell(cell)}

    fn get_char_representation(&self) -> char;
    fn get_state(&self) -> Box<&dyn PieceStateTrait>;
    fn get_mut_state(&mut self) -> Box<&mut dyn PieceStateTrait>;
    fn is_valid_move(&self, board: &Board, the_move: &Move) -> bool;
}
