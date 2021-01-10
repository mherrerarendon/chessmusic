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

    // functionaliry
    fn get_valid_cells(&self, board: &Board) -> Vec<Cell>;
    fn is_valid_move(&self, board: &Board, the_move: &Move) -> bool;
}

pub struct NotPawn {
    pub name: PieceName,
    pub white: bool, 
    pub role: Role,
    pub cell: Cell,
    pub first_move: bool
}

impl Piece for NotPawn {
    fn get_name(&self) -> PieceName {self.name}
    fn is_white(&self) -> bool {self.white}
    fn get_role(&self) -> Role {self.role}
    fn get_curr_cell(&self) -> &Cell {&self.cell}
    fn set_new_cell(&mut self, cell: &Cell) {()}
    fn has_moved(&self) -> bool {!self.first_move}
    fn set_has_moved(&mut self) {()}

    fn get_valid_cells(&self, _board: &Board) -> Vec<Cell> {
        vec![Cell::new()]
    }

    fn is_valid_move(&self, board: &Board, the_move: &Move) -> bool {true}
}

pub struct Pawn {
    pub name: PieceName,
    pub white: bool, 
    pub role: Role,
    pub cell: Cell,
    pub first_move: bool
}

fn add_cell_if_valid(board: &Board, cell: Option<Cell>, needs_empty: bool, mut valid_cells: Vec<Cell>) -> Vec<Cell> {
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

    // Consider not using Board to avoid circular reference
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

    #[test]
    fn test_get_pawn_valid_cells_white() {
        let board = Board::new();
        match board.get_piece_at_cell(&Cell {file: 'a', row: 2}) {
            Some(piece) => {
                let valid_cells = piece.get_valid_cells(&board);
                assert_eq!(valid_cells.len(), 2);
                assert!(valid_cells.contains(&Cell {file: 'a', row: 3}));
                assert!(valid_cells.contains(&Cell {file: 'a', row: 4}));
            },
            None => ()
        }
    }

    #[test]
    fn test_get_pawn_valid_cells_black() {
        let board = Board::new();
        match board.get_piece_at_cell(&Cell {file: 'a', row: 7}) {
            Some(piece) => {
                let valid_cells = piece.get_valid_cells(&board);
                assert_eq!(valid_cells.len(), 2);
                assert!(valid_cells.contains(&Cell {file: 'a', row: 6}));
                assert!(valid_cells.contains(&Cell {file: 'a', row: 5}));
            },
            None => ()
        }
    }
}