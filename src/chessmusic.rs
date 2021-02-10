use chess::{PieceName, board::Board, chess_move::Move, piece::Piece};

use super::pgn;
use super::chess;
use super::music::{MidiPlayer, Pitch};

extern crate crossbeam;

static PIECES: &'static [(chess::PieceName, bool)] = &[
    (chess::PieceName::Qknight, true)
    ];

fn get_pitches_for_piece(piece_name: chess::PieceName, white: bool, moves: &Vec<(Move, Move)>) -> Vec<Pitch> {
    let piece_history = chess::Game::get_piece_history(piece_name, white, &moves);
    Pitch::get_pitches_from_cell_history(&piece_history)
}

pub fn play_game(game_str: &str) {
    let str_moves = match pgn::parse_moves(game_str) {
        Ok(str_moves) => str_moves,
        Err(_the_error) => panic!("failed to parse moves")
    };
    let moves = chess::chess_move::Move::parse_moves(&str_moves);

    crossbeam::scope(|s| {
        let moves = &moves;
        for (piece_name, white) in PIECES.iter().cloned() {
            s.spawn(move |_| {
                let _pitches = get_pitches_for_piece(piece_name, white, &moves);
            });
        }
    }).unwrap();

    let mut midi_player = MidiPlayer::new();
    // for pitch in pitches.iter() {
    //     midi_player.play_note(pitch.as_midi());
    // }
}
