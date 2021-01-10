use super::cell::Cell;
use super::types::{Role, PieceName};
use super::board::Board;

// pub trait ValidCells {
//     fn get_valid_cells(&self, board: &Board) -> Vec<Cell>;
// }

pub trait Piece {
    // common data
    fn get_name(&self) -> PieceName;
    fn is_white(&self) -> bool;
    fn get_role(&self) -> Role;
    fn get_curr_cell(&self) -> &Cell;
    fn has_moved(&self) -> bool;

    // functionaliry
    fn get_valid_cells(&self, board: &Board) -> Vec<Cell>;
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
    fn has_moved(&self) -> bool {!self.first_move}

    fn get_valid_cells(&self, _board: &Board) -> Vec<Cell> {
        vec![Cell::new()]
    }
}

pub struct Pawn {
    pub name: PieceName,
    pub white: bool, 
    pub role: Role,
    pub cell: Cell,
    pub first_move: bool
}

impl Pawn {
    fn add_cell_if_valid_and_empty(&self, board: &Board, cell: Option<Cell>, mut valid_cells: Vec<Cell>) -> Vec<Cell> {
        match cell {
            Some(cell) => {
                match board.get_piece_at_cell(&cell) {
                    Some(_piece) => (),
                    None => valid_cells.push(cell)
                }
            },
            None => ()
        }

        valid_cells
    }
}

impl Piece for Pawn {
    fn get_name(&self) -> PieceName {self.name}
    fn is_white(&self) -> bool {self.white}
    fn get_role(&self) -> Role {self.role}
    fn get_curr_cell(&self) -> &Cell {&self.cell}
    fn has_moved(&self) -> bool {!self.first_move}

    // Consider not using Board to avoid circular reference
    fn get_valid_cells(&self, board: &Board) -> Vec<Cell> {
        let mut valid_cells: Vec<Cell> = Vec::new();
        let direction = if self.white {1} else {-1};
        if self.first_move {
            match Cell::new_from_cell(&self.cell, 0, 2 * direction) {
                Some(cell) => {
                    match board.get_piece_at_cell(&cell) {
                        Some(_piece) => (),
                        None => valid_cells.push(cell)
                    }
                },
                None => ()
            }
        }
        else {
            // Take left
            match Cell::new_from_cell(&self.cell, -1, 1 * direction) {
                Some(cell) => {
                    match board.get_piece_at_cell(&cell) {
                        Some(_piece) => valid_cells.push(cell),
                        None => ()
                    }
                },
                None => ()
            }

            // Take right
            match Cell::new_from_cell(&self.cell, -1, 1 * direction) {
                Some(cell) => {
                    match board.get_piece_at_cell(&cell) {
                        Some(_piece) => valid_cells.push(cell),
                        None => ()
                    }
                },
                None => ()
            }
        }

        // Simple forward
        match Cell::new_from_cell(&self.cell, 0, 1 * direction) {
            Some(cell) => {
                match board.get_piece_at_cell(&cell) {
                    Some(_piece) => (),
                    None => valid_cells.push(cell)
                }
            },
            None => ()
        }

        valid_cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_valid_cells() {
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
}