use super::super::types::{PieceName, Role};
use super::super::cell::Cell;
use super::{Piece, PieceState, PieceStateTrait};
use super::super::board::Board;
use super::super::chess_move::Move;

pub struct Pawn {
    pub state: PieceState
}

impl Piece for Pawn {
    fn get_state(&self) -> Box<&dyn PieceStateTrait> {Box::new(&self.state)}
    fn get_mut_state(&mut self) -> Box<&mut dyn PieceStateTrait> {Box::new(&mut self.state)}
    fn get_char_representation(&self) -> char {if self.is_white() {'P'} else {'p'}}
    fn is_valid_move(&self, board: &Board, the_move: &Move) -> bool {
        let valid_cells = self.get_valid_cells(board);
        return valid_cells.contains(&the_move.cell);
    }    
}

impl Pawn {
    pub fn new(white: bool, name: PieceName) -> Pawn {
        Pawn {
            state: PieceState {
                name: name, 
                white: white, 
                role: Role::Pawn, 
                first_move: true, 
                cell: Pawn::init_cell(white, name),
                cell_history: Vec::new()
            }
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

    fn add_cell_if_valid(&self, board: &Board, cell_opt: Option<Cell>, is_take: bool, mut valid_cells: Vec<Cell>) -> Vec<Cell> {
        if let Some(cell) = cell_opt {
            if let Some(piece) = board.get_piece_at_cell(&cell) {
                if is_take && piece.is_white() != self.is_white() {
                    valid_cells.push(cell)
                }
            } else {
                if !is_take {
                    valid_cells.push(cell)
                }
            }
        }
    
        valid_cells
    }

    fn get_valid_cells(&self, board: &Board) -> Vec<Cell> {
        let mut valid_cells: Vec<Cell> = Vec::new();
        let direction = if self.is_white() {1} else {-1};
        if !self.has_moved() {
            let double_forward_cell_option = Cell::new_from_cell(self.get_curr_cell(), 0, 2 * direction);
            valid_cells = self.add_cell_if_valid(board, double_forward_cell_option, false, valid_cells);
        }

        let take_left_cell_option = Cell::new_from_cell(self.get_curr_cell(), -1, 1 * direction);
        valid_cells = self.add_cell_if_valid(board, take_left_cell_option, true, valid_cells);

        let take_right_cell_option = Cell::new_from_cell(self.get_curr_cell(), 1, 1 * direction);
        valid_cells = self.add_cell_if_valid(board, take_right_cell_option, true, valid_cells);

        let single_forward_cell_option = Cell::new_from_cell(self.get_curr_cell(), 0, 1 * direction);
        valid_cells = self.add_cell_if_valid(board, single_forward_cell_option, false, valid_cells);

        valid_cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_simple_white_move() {
        let board = Board::new();
        match board.get_piece_at_cell(&Cell {file: 'a', row: 2}) {
            Some(pawn) => {
                assert!(pawn.is_valid_move(&board, &Move::parse("a3")));
                assert!(pawn.is_valid_move(&board, &Move::parse("a4")));
            },
            None => panic!("expected to find piece")
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
            None => panic!("expected to find piece")
        }
    }

    #[test]
    fn test_valid_take_white_and_black_move() {
        let mut board = Board::new();
        board.move_piece(PieceName::Dpawn, true, &Cell::new("d4"));
        board.move_piece(PieceName::Epawn, false, &Cell::new("e5"));
        match board.get_piece_at_cell(&Cell::new("d4")) {
            Some(pawn) => {
                assert!(pawn.is_valid_move(&board, &Move::parse("e5")));
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

    #[test]
    fn test_valid_take_white_and_black_move_opposite() {
        let mut board = Board::new();
        board.move_piece(PieceName::Dpawn, true, &Cell::new("e4"));
        board.move_piece(PieceName::Epawn, false, &Cell::new("d5"));

        let pawn = board.get_piece_at_cell(&Cell::new("e4")).expect("Expected to find pawn");
        assert!(pawn.is_valid_move(&board, &Move::parse("d5")));

        let pawn = board.get_piece_at_cell(&Cell::new("d5")).expect("Expected to find pawn");
        assert!(pawn.is_valid_move(&board, &Move::parse("e4")));
    }

    #[test]
    fn test_take_same_color_pawn_is_invalid() {
        let mut board = Board::new();
        board.move_piece(PieceName::Dpawn, true, &Cell::new("d3"));
        match board.get_piece_at_cell(&Cell::new("c2")) {
            Some(pawn) => {
                assert!(!pawn.is_valid_move(&board, &Move::parse("d3")));
            },
            None => panic!("Expected to find pawn")
        }
    }
}