use super::types::{MoveType, PieceName};
use super::chess_move::Move;
use super::cell::Cell;
use super::board::Board;

use std::error::Error;

#[cfg(test)]
use super::types::{Role};

pub struct Game {
    pub board: Board
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::new()
        }
    }

    #[cfg(test)]
    pub fn new_test(role: Role) -> Game {
        match role {
            Role::Rook => Game {board: Board::new_rook_test()},
            Role::Bishop => Game {board: Board::new_bishop_test()},
            _ => panic!("no test board for specified role")
        }
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
            let rook_file = if the_move.move_type == MoveType::CastleKing {'f'} else {'d'};

            self.board.move_piece(PieceName::King, white, &Cell {file: king_file, row: row});
            self.board.move_piece(rook_name, white, &Cell {file: rook_file, row: row});
        }
        else {
            let name = match self.get_piece_for_move(white, &the_move) {
                Ok(name) => name,
                Err(the_error) => {
                    println!("Failed to get name for {:?} {:?}", white, the_move);
                    panic!("{}", the_error.to_string());
                }
            };
            self.board.move_piece(name, white, &the_move.cell);
        }
    }

    fn add_move_pair(&mut self, white_move: &Move, black_move: &Move) {
        self.add_one_move(true, white_move);

        // Black move might not exist if the last move of the game is white's move.
        if black_move.move_type != MoveType::None {
            self.add_one_move(false, black_move); 
        }
             
    }

    pub fn get_piece_history(name: PieceName, white: bool, game_moves: &Vec<(Move, Move)>) -> Vec<Cell> {
        let mut piece_history: Vec<Cell> = Vec::new();
        let mut game = Game::new();
        if let Some(piece) = game.board.get_piece_with_name(name, white) {
            piece_history.push(piece.get_curr_cell().clone());
        }

        for (white_move, black_move) in game_moves.iter() {
            game.add_move_pair(&white_move, &black_move);
            if let Some(piece) = game.board.get_piece_with_name(name, white) {
                piece_history.push(piece.get_curr_cell().clone());
            } else {
                break;
            }
        }

        piece_history
    }

    pub fn load_moves(&mut self, moves: &[&Move]) {
        self.board = Board::new();
        for (idx, the_move) in moves.iter().enumerate() {
            let white = idx % 2 == 0;
            self.add_one_move(white, the_move);
        }
    }
}

// impl Iterator for Game {
//     type Item = &'a Cell;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.cell_history.iter().next()
//     }
// }

#[cfg(test)]
mod tests {
    use core::panic;

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
        game.add_move_pair(&Move::parse("d4"), &Move::parse("e5"));
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

    #[test]
    fn test_real_moves() {
        // 1. d4 Nf6 2. Bf4 Nc6 3. e3 d5 4. Nf3 Bf5 5. Nbd2 e6 6. c3 Bd6 7. Bg5 h6 8. Bh4 g5 9. Bg3 Ne4 10. Nxe4 Bxe4 
        let mut game = Game::new();

        game.add_move_pair(&Move::parse("d4"), &Move::parse("Nf6"));
        let white_piece = game.board.get_piece_with_name(PieceName::Dpawn, true).unwrap();
        let black_piece = game.board.get_piece_with_name(PieceName::Kknight, false).unwrap();
        assert_eq!(*white_piece.get_curr_cell(), Cell::new("d4"));
        assert_eq!(*black_piece.get_curr_cell(), Cell::new("f6"));

        game.add_move_pair(&Move::parse("Bf4"), &Move::parse("Nc6"));
        let white_piece = game.board.get_piece_with_name(PieceName::Qbishop, true).unwrap();
        let black_piece = game.board.get_piece_with_name(PieceName::Qknight, false).unwrap();
        assert_eq!(*white_piece.get_curr_cell(), Cell::new("f4"));
        assert_eq!(*black_piece.get_curr_cell(), Cell::new("c6"));

        game.add_move_pair(&Move::parse("e3"), &Move::parse("d5"));
        let white_piece = game.board.get_piece_with_name(PieceName::Epawn, true).unwrap();
        let black_piece = game.board.get_piece_with_name(PieceName::Dpawn, false).unwrap();
        assert_eq!(*white_piece.get_curr_cell(), Cell::new("e3"));
        assert_eq!(*black_piece.get_curr_cell(), Cell::new("d5"));

        game.add_move_pair(&Move::parse("Nf3"), &Move::parse("Bf5"));
        let white_piece = game.board.get_piece_with_name(PieceName::Kknight, true).unwrap();
        let black_piece = game.board.get_piece_with_name(PieceName::Qbishop, false).unwrap();
        assert_eq!(*white_piece.get_curr_cell(), Cell::new("f3"));
        assert_eq!(*black_piece.get_curr_cell(), Cell::new("f5"));

        game.add_move_pair(&Move::parse("Nbd2"), &Move::parse("e6"));
        let white_piece = game.board.get_piece_with_name(PieceName::Qknight, true).unwrap();
        let black_piece = game.board.get_piece_with_name(PieceName::Epawn, false).unwrap();
        assert_eq!(*white_piece.get_curr_cell(), Cell::new("d2"));
        assert_eq!(*black_piece.get_curr_cell(), Cell::new("e6"));

        game.add_move_pair(&Move::parse("c3"), &Move::parse("Bd6"));
        let white_piece = game.board.get_piece_with_name(PieceName::Cpawn, true).unwrap();
        let black_piece = game.board.get_piece_with_name(PieceName::Kbishop, false).unwrap();
        assert_eq!(*white_piece.get_curr_cell(), Cell::new("c3"));
        assert_eq!(*black_piece.get_curr_cell(), Cell::new("d6"));

        game.add_move_pair(&Move::parse("Bg5"), &Move::parse("h6"));
        let white_piece = game.board.get_piece_with_name(PieceName::Qbishop, true).unwrap();
        let black_piece = game.board.get_piece_with_name(PieceName::Hpawn, false).unwrap();
        assert_eq!(*white_piece.get_curr_cell(), Cell::new("g5"));
        assert_eq!(*black_piece.get_curr_cell(), Cell::new("h6"));

        game.add_move_pair(&Move::parse("Bh4"), &Move::parse("g5"));
        let white_piece = game.board.get_piece_with_name(PieceName::Qbishop, true).unwrap();
        let black_piece = game.board.get_piece_with_name(PieceName::Gpawn, false).unwrap();
        assert_eq!(*white_piece.get_curr_cell(), Cell::new("h4"));
        assert_eq!(*black_piece.get_curr_cell(), Cell::new("g5"));

        game.add_move_pair(&Move::parse("Bg3"), &Move::parse("Ne4"));
        let white_piece = game.board.get_piece_with_name(PieceName::Qbishop, true).unwrap();
        let black_piece = game.board.get_piece_with_name(PieceName::Kknight, false).unwrap();
        assert_eq!(*white_piece.get_curr_cell(), Cell::new("g3"));
        assert_eq!(*black_piece.get_curr_cell(), Cell::new("e4"));

        game.add_move_pair(&Move::parse("Nxe4"), &Move::parse("Bxe4"));
        match game.board.get_piece_with_name(PieceName::Qknight, true) {
            Some(_piece) => panic!("This piece was just taken, did not expect to find it."),
            None => ()
        }
        let black_piece = game.board.get_piece_with_name(PieceName::Qbishop, false).unwrap();
        assert_eq!(*black_piece.get_curr_cell(), Cell::new("e4"));
    }

    #[test]
    fn test_bishop_history() {
        let moves = vec![(Move::parse("d4"), Move::parse("Nf6")),   // 1
                                         (Move::parse("Bf4"), Move::parse("Nc6")),   // 2
                                         (Move::parse("e3"), Move::parse("d5")),   // 3
                                         (Move::parse("Nf3"), Move::parse("Bf5")),   // 4
                                         (Move::parse("Nbd2"), Move::parse("e6")),   // 5
                                         (Move::parse("c3"), Move::parse("Bd6")),   // 6
                                         (Move::parse("Bg5"), Move::parse("h6")),   // 7
                                         (Move::parse("Bh4"), Move::parse("g5")),   // 8
                                         (Move::parse("Bg3"), Move::parse("Ne4")),   // 9
                                         (Move::parse("Nxe4"), Move::parse("Bxe4")),]; // 10
        let white_q_bishop_history = Game::get_piece_history(PieceName::Qbishop, true, &moves);
        assert_eq!(white_q_bishop_history.len(), 11);
        assert_eq!(white_q_bishop_history[0], Cell::new("c1"));
        assert_eq!(white_q_bishop_history[1], Cell::new("c1"));
        assert_eq!(white_q_bishop_history[2], Cell::new("f4"));
        assert_eq!(white_q_bishop_history[3], Cell::new("f4"));
        assert_eq!(white_q_bishop_history[4], Cell::new("f4"));
        assert_eq!(white_q_bishop_history[5], Cell::new("f4"));
        assert_eq!(white_q_bishop_history[6], Cell::new("f4"));
        assert_eq!(white_q_bishop_history[7], Cell::new("g5"));
        assert_eq!(white_q_bishop_history[8], Cell::new("h4"));
        assert_eq!(white_q_bishop_history[9], Cell::new("g3"));
        assert_eq!(white_q_bishop_history[10], Cell::new("g3"));
    }
}