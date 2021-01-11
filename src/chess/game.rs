use std::collections::HashMap;
use super::types::{Role, MoveType, PieceName};
use super::chess_move::Move;
use super::cell::Cell;
use super::board::Board;
use super::piece::Piece;

use std::error::Error;

pub struct Game {
    pub board: Board
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::new()
        }
    }

    fn new_with_pgn(pgn: &str) -> Game {
        // TODO: 
        Game::new()
    } 

    fn new_with_game_id(game_id: &str) -> Game {
        // TODO: 
        Game::new()
    } 

    fn get_piece_for_move(&self, white: bool, the_move: &Move) -> Result<PieceName, Box<dyn Error>> {
        let role = the_move.role;
        let pieces_with_role = self.board.get_pieces_with_role(role, white);
        if the_move.file_hint != ' ' {
            let piece = pieces_with_role.iter().filter(|piece| piece.get_curr_cell().file == the_move.file_hint).next().unwrap();
            return Ok(piece.get_name());
        }
        else {
            for piece in pieces_with_role.iter() {
                if piece.is_valid_move(&self.board, the_move) {
                    return Ok(piece.get_name());
                }
            }
        }

        return Err("Bad request")?;
    }

    fn add_move(&mut self, white_move: &Move, black_move: &Move) {
        let white_name = self.get_piece_for_move(true, &white_move).unwrap();
        self.board.move_piece(white_name, true, &white_move.cell);

        let black_name = self.get_piece_for_move(false, &black_move).unwrap();
        self.board.move_piece(black_name, false, &black_move.cell);
    }

    pub fn get_piece_history(name: PieceName, white: bool, game_moves: &Vec<(Move, Move)>) -> Vec<Cell> {
        let mut piece_history: Vec<Cell> = Vec::new();
        let mut game = Game::new();
        match game.board.get_piece_with_name(name, white) {
            Some(piece) => piece_history.push(piece.get_curr_cell().clone()),
            None => ()
        }
        for (move_num, (white_move, black_move)) in game_moves.iter().enumerate() {
            game.add_move(&white_move, &black_move);
            match game.board.get_piece_with_name(name, white) {
                Some(piece) => piece_history.push(piece.get_curr_cell().clone()),
                None => ()
            }
        }

        piece_history
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_piece_for_white_pawn_simple_move() {
        let game = Game::new();
        let name = game.get_piece_for_move(true, &Move::parse("a3")).unwrap();
        assert_eq!(name, PieceName::Apawn);
    }

    #[test]
    fn test_get_piece_for_black_pawn_simple_move() {
        let game = Game::new();
        let name = game.get_piece_for_move(false, &Move::parse("a6")).unwrap();
        assert_eq!(name, PieceName::Apawn);
    }

    #[test]
    fn test_get_piece_for_pawn_take() {
        let mut game = Game::new();
        game.add_move(&Move::parse("d4"), &Move::parse("e5"));
        let name = game.get_piece_for_move(true, &Move::parse("dxe5")).unwrap();
        assert_eq!(name, PieceName::Dpawn);
    }

    #[test]
    fn test_pawn_history() {
        let game_moves = vec![(Move::parse("a3"), Move::parse("a6"))];
        let white_pawn_history = Game::get_piece_history(PieceName::Apawn, true, &game_moves);
        assert_eq!(white_pawn_history.len(), 2);
        assert_eq!(white_pawn_history[0], Cell {file: 'a', row: 2});
        assert_eq!(white_pawn_history[1], Cell {file: 'a', row: 3});

        let black_pawn_history = Game::get_piece_history(PieceName::Apawn, false, &game_moves);
        assert_eq!(black_pawn_history.len(), 2);
        assert_eq!(black_pawn_history[0], Cell {file: 'a', row: 7});
        assert_eq!(black_pawn_history[1], Cell {file: 'a', row: 6});
    }
}