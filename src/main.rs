mod lichess;
mod pgn;
mod chess;
mod music;

use std::{error::Error, vec};

use music::{MidiPlayer, Pitch};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let game_str = lichess::get_game(String::from("tzUJbFEX")).await?; 
    println!("Game:\n\n{}", game_str);

    let str_moves = pgn::parse_moves(&game_str)?;
    let moves = chess::chess_move::Move::parse_moves(&str_moves);
    let piece_history = chess::Game::get_piece_history(chess::PieceName::Qknight, true, &moves);
    let white_qknight_pitches = Pitch::get_pitches_from_cell_history(&piece_history);
    let mut midi_player = MidiPlayer::new();
    for pitch in white_qknight_pitches.iter() {
        midi_player.play_note(pitch.as_midi() as u8)
    }

    Ok(())
}

