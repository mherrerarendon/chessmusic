use super::super::cell::Cell;
use super::super::board::Board;

pub fn attempt_to_add_as_valid_cell(cell_opt: Option<Cell>, board: &Board, valid_cells: &mut Vec<Cell>, white: bool) -> bool {
    let mut cont = true;
    if let Some(cell) = cell_opt {
        if let Some(piece) = board.get_piece_at_cell(&cell) {
            if piece.is_white() != white {
                valid_cells.push(cell.clone());
            }
            cont = false
        } else {
            valid_cells.push(cell.clone());
        }
    } else {
        cont = false;
    }

    cont
}