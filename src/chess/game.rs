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

    pub fn new_test(role: Role) -> Game {
        match role {
            Role::Rook => Game {board: Board::new_rook_test()},
            Role::Bishop => Game {board: Board::new_bishop_test()},
            _ => panic!("no test board for specified role")
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

    fn add_one_move(&mut self, white: bool, the_move: &Move) {
        if the_move.move_type == MoveType::CastleKing || the_move.move_type == MoveType::CastleQueen {
            let rook_name = if the_move.move_type == MoveType::CastleKing {PieceName::Krook} else {PieceName::Qrook};
            let row = if white {1} else {8};
            let king_file = if the_move.move_type == MoveType::CastleKing {'g'} else {'c'};
            let rook_file = if the_move.move_type == MoveType::CastleKing {'e'} else {'d'};

            self.board.move_piece(PieceName::King, white, &Cell {file: king_file, row: row});
            self.board.move_piece(rook_name, white, &Cell {file: rook_file, row: row});
        }
        else {
            let name = self.get_piece_for_move(white, &the_move).unwrap();
            self.board.move_piece(name, white, &the_move.cell);
        }
    }

    fn add_move(&mut self, white_move: &Move, black_move: &Move) {
        self.add_one_move(true, white_move);
        self.add_one_move(false, black_move);      
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
    fn test_get_piece_for_bishop_move() {
        let game = Game::new_test(Role::Bishop);
        let name = game.get_piece_for_move(true, &Move::parse("Bf4")).unwrap();
        assert_eq!(name, PieceName::Qbishop);

        let name = game.get_piece_for_move(true, &Move::parse("Bb5")).unwrap();
        assert_eq!(name, PieceName::Kbishop);

        let name = game.get_piece_for_move(false, &Move::parse("Ba6")).unwrap();
        assert_eq!(name, PieceName::Qbishop);

        let name = game.get_piece_for_move(false, &Move::parse("Ba3")).unwrap();
        assert_eq!(name, PieceName::Kbishop);
    }

    #[test]
    fn test_get_piece_for_knight_move() {
        let game = Game::new();
        let name = game.get_piece_for_move(true, &Move::parse("Nc3")).unwrap();
        assert_eq!(name, PieceName::Qknight);

        let name = game.get_piece_for_move(true, &Move::parse("Nf3")).unwrap();
        assert_eq!(name, PieceName::Kknight);

        let name = game.get_piece_for_move(false, &Move::parse("Nc6")).unwrap();
        assert_eq!(name, PieceName::Qknight);

        let name = game.get_piece_for_move(false, &Move::parse("Nh6")).unwrap();
        assert_eq!(name, PieceName::Kknight);
    }

    #[test]
    fn test_get_piece_for_rook_move() {
        let game = Game::new_test(Role::Rook);
        let name = game.get_piece_for_move(true, &Move::parse("Ra2")).unwrap();
        assert_eq!(name, PieceName::Qrook);

        // doesn't fail on ambiguous moves yet.
        // match game.get_piece_for_move(true, &Move::parse("Rb1")) {
        //     Ok(_piece) => panic!("Rb1 should be ambiguous in test board for rook"),
        //     Err(_the_error) => ()
        // }

        let name = game.get_piece_for_move(true, &Move::parse("Rh2")).unwrap();
        assert_eq!(name, PieceName::Krook);

        let name = game.get_piece_for_move(false, &Move::parse("Rh2")).unwrap();
        assert_eq!(name, PieceName::Krook);

        let name = game.get_piece_for_move(false, &Move::parse("Ra2")).unwrap();
        assert_eq!(name, PieceName::Qrook);
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