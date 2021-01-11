use super::types::{PieceName, Role};
use super::cell::Cell;
use super::piece::{Piece, add_cell_if_valid};
use super::board::Board;
use super::chess_move::Move;


pub struct Rook {
    pub name: PieceName,
    pub white: bool, 
    pub role: Role,
    pub cell: Cell,
    pub first_move: bool
}

impl Piece for Rook {
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

impl Rook {
    fn get_valid_cells(&self, board: &Board) -> Vec<Cell> {
        vec![Cell::new("a1")]
    }
}
