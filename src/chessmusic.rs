use chess::{PieceName, board::Board, chess_move::Move, piece::Piece};

use super::pgn;
use super::chess;
use super::music::{MidiPlayer, Pitch};

use std::sync::mpsc;

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

    let (tx, rx): (mpsc::Sender<Vec<Pitch>>, mpsc::Receiver<Vec<Pitch>>) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);

    crossbeam::scope(|s| {
        let moves = &moves;
        
        for (piece_name, white) in PIECES.iter().cloned() {
            let tx1 = mpsc::Sender::clone(&tx);
            s.spawn(move |_| {
                let pitches = get_pitches_for_piece(piece_name, white, &moves);
                // let val = String::from("hi");
                tx1.send(pitches).unwrap();
            });
        }
    }).unwrap();

    let mut pitches_list = Vec::with_capacity(PIECES.len());
    for _ in 0..PIECES.len() {
        // The `recv` method picks a message from the channel
        // `recv` will block the current thread if there are no messages available
        match rx.recv() {
            Ok(pitches) => pitches_list.push(pitches),
            Err(the_error) => {
                println!("{}", the_error.to_string())
            }
        }
    }

    let mut midi_player = MidiPlayer::new();
    for piece in pitches_list.iter() {
        for pitch in piece.iter() {
            midi_player.play_note(pitch.as_midi());
        }
    }
    
}
