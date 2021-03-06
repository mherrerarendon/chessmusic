pub mod bishop;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod queen;
pub mod rook;
mod piece_utils;

use super::cell::Cell;
use super::types::{Role, PieceName};
use super::board::Board;
use super::chess_move::Move;
use super::types::MoveType;

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
    pub cell: Option<Cell>,
    pub move_history: Vec<Option<Move>>
}

impl PieceStateTrait for PieceState {
    fn get_name(&self) -> PieceName {self.name}
    fn is_white(&self) -> bool {self.white}
    fn get_role(&self) -> Role {self.role}
    fn get_curr_cell(&self) -> Option<Cell> {self.cell}
    fn move_(&mut self, the_move: Option<&Move>) {
        if let Some(the_move) = the_move {
            self.cell = Some(the_move.cell);
        }
        
        self.move_history.push(the_move.map(|x|x.clone()));
    }
    fn has_moved(&self) -> bool {self.move_history.iter().filter(|x| x.is_some()).count() > 0}
    fn get_move_history(&self) -> &[Option<Move>] {&self.move_history}
    fn set_captured(&mut self) {
        self.cell = None
    }
}

pub trait PieceStateTrait {
    fn get_name(&self) -> PieceName;
    fn is_white(&self) -> bool;
    fn get_role(&self) -> Role;
    fn get_curr_cell(&self) -> Option<Cell>;
    fn move_(&mut self, the_move: Option<&Move>); 
    fn has_moved(&self) -> bool;
    fn get_move_history(&self) -> &[Option<Move>];
    fn set_captured(&mut self);
}


pub trait Piece {
    // common data
    fn get_name(&self) -> PieceName {self.get_state().get_name()}
    fn is_white(&self) -> bool {self.get_state().is_white()}
    fn get_role(&self) -> Role {self.get_state().get_role()}
    fn get_curr_cell(&self) -> Option<Cell> {self.get_state().get_curr_cell()}
    fn has_moved(&self) -> bool {self.get_state().has_moved()}
    fn move_(&mut self, the_move: Option<&Move>) {self.get_mut_state().move_(the_move)}
    fn get_move_history(&self) -> &[Option<Move>] {self.get_state().get_move_history()}

    // Note: does not include starting cell
    fn get_cell_and_capture_history(&self) -> Vec<(Cell, bool)> {
        let mut history = Vec::new();
        let mut last_cell = self.first_cell();
        for move_ in self.get_move_history() {
            let mut move_is_capture = false;
            if let Some(move_) = move_ {
                last_cell = move_.cell;
                move_is_capture = move_.move_type == MoveType::Take;
            }

            history.push((last_cell, move_is_capture));
        }

        history
    }
    fn set_captured(&mut self) {
        self.get_mut_state().set_captured()
    }

    fn get_char_representation(&self) -> char;
    fn get_state(&self) -> Box<&dyn PieceStateTrait>;
    fn get_mut_state(&mut self) -> Box<&mut dyn PieceStateTrait>;
    fn is_valid_move(&self, board: &Board, the_move: &Move) -> bool;
    fn first_cell(&self) -> Cell;
    fn is_live(&self) -> bool {self.get_state().get_curr_cell().is_some()}
}

// impl PartialEq for dyn Piece + '_ {
//     fn eq(&self, other: &Self) -> bool {
//         self.get_state().get_name() == other.get_state().get_name()
//         && self.get_state().is_white() == other.get_state().is_white()
//     }
// }

