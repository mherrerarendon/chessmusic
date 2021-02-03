use super::pgn;
use super::chess;
use super::music;

use music::{MidiPlayer, Pitch};

pub fn play_game(game_str: &str) {
    let str_moves = match pgn::parse_moves(game_str) {
        Ok(str_moves) => str_moves,
        Err(_the_error) => panic!("failed to parse moves")
    };
    let moves = chess::chess_move::Move::parse_moves(&str_moves);
    let piece_history = chess::Game::get_piece_history(chess::PieceName::Qknight, true, &moves);
    let white_qknight_pitches = Pitch::get_pitches_from_cell_history(&piece_history);
    let mut midi_player = MidiPlayer::new();
    for pitch in white_qknight_pitches.iter() {
        midi_player.play_note(pitch.as_midi());
    }
}