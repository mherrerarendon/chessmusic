use crate::chess::types::MoveType;

use super::pgn;
use super::chess;
use super::music::{MidiPlayer, Note, Melody};

use std::sync::mpsc;

extern crate crossbeam;

static PIECES: &'static [(chess::PieceName, bool)] = &[
    // (chess::PieceName::Apawn, true),
    // (chess::PieceName::Bpawn, true),
    // (chess::PieceName::Cpawn, true),
    // (chess::PieceName::Dpawn, true),
    (chess::PieceName::Epawn, true),
    // (chess::PieceName::Fpawn, true),
    // (chess::PieceName::Gpawn, true),
    // (chess::PieceName::Hpawn, true),
    // (chess::PieceName::Qrook, true),
    (chess::PieceName::Qknight, true),
    // (chess::PieceName::Qbishop, true),
    (chess::PieceName::Queen, true),
    // (chess::PieceName::King, true),
    // (chess::PieceName::Kbishop, true),
    // (chess::PieceName::Kknight, true),

    // (chess::PieceName::Apawn, false),
    // (chess::PieceName::Bpawn, false),
    // (chess::PieceName::Cpawn, false),
    // (chess::PieceName::Dpawn, false),
    (chess::PieceName::Epawn, false),
    // (chess::PieceName::Fpawn, false),
    // (chess::PieceName::Gpawn, false),
    // (chess::PieceName::Hpawn, false),
    // (chess::PieceName::Krook, true),
    // (chess::PieceName::Qrook, false),
    // (chess::PieceName::Qknight, false),
    // (chess::PieceName::Qbishop, false),
    (chess::PieceName::Queen, false),
    // (chess::PieceName::King, false),
    // (chess::PieceName::Kbishop, false),
    (chess::PieceName::Kknight, false),
    // (chess::PieceName::Krook, false),
];

struct ChessMusic {
    game: chess::Game
}

fn generate_pitches_by_pieces(pieces: &[(chess::PieceName, bool)], tx: mpsc::Sender<Melody>, game: &chess::Game) {
    crossbeam::scope(|s| {
        for (piece_name, white) in pieces.iter() {
            let tx1 = mpsc::Sender::clone(&tx);
            s.spawn(move |_| {
                let move_history_with_captures = game.board.get_piece_with_name(*piece_name, *white).unwrap().get_move_history().iter()
                    .map(|move_| (move_.cell, move_.move_type == MoveType::Take)).collect::<Vec<_>>();
                let melody = Melody::new(&move_history_with_captures, Note::new(120));
                tx1.send(melody).unwrap();
            });
        }
    }).unwrap();
}

fn receive_pitches_by_piece(pieces: &[(chess::PieceName, bool)], rx: mpsc::Receiver<Melody>) -> Vec<Melody> {
    let mut melodies: Vec<Melody> = Vec::with_capacity(pieces.len());

    for _ in 0..pieces.len() {
        match rx.recv() {
            Ok(melody) => melodies.push(melody),
            Err(the_error) => {
                println!("{}", the_error.to_string())
            }
        }
    }

    melodies
}

fn chords_from_pitches_by_piece(melodies: &[Melody]) -> Vec<Vec<Note>> {
    let mut chords: Vec<Vec<Note>> = Vec::new();
    let longest_length = melodies.iter().max_by_key(|pitches| pitches.notes.len()).unwrap().notes.len();
    for n in 0..longest_length {
        let mut chord: Vec<Note> = Vec::new();
        for pitches in melodies.iter() {
            if let Some(pitch) =  pitches.notes.get(n) {
                chord.push(*pitch);
            }
        }

        chords.push(chord);
    }
    chords
}

pub fn play_game(game_str: &str) {
    let str_moves = pgn::parse_moves(game_str);
    let moves = chess::Move::parse_moves(&str_moves);
    let game = chess::Game::new_with_moves(&moves);

    let (tx, rx): (mpsc::Sender<Melody>, mpsc::Receiver<Melody>) = mpsc::channel();
    generate_pitches_by_pieces(PIECES, tx, &game);
    let pitches_by_piece = receive_pitches_by_piece(PIECES, rx);
    let chords = chords_from_pitches_by_piece(&pitches_by_piece);

    let mut midi_player = MidiPlayer::new();
    for chord in chords.iter() {
        let notes: Vec<u8> = chord.iter().map(|pitch|pitch.as_midi()).collect();
        midi_player.play_notes(&notes);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chords_from_pitches_by_piece_with_equal_length() {
        let cell_history1 = vec![chess::Cell::new("a2"), chess::Cell::new("a3"), chess::Cell::new("a4")];
        let pitches1 = Note::get_pitches_from_cell_history(&cell_history1);
        let cell_history2 = vec![chess::Cell::new("c2"), chess::Cell::new("c3"), chess::Cell::new("d4")];
        let pitches2 = Note::get_pitches_from_cell_history(&cell_history2);
        let pitches_by_piece = vec![pitches1, pitches2];
        let chords = chords_from_pitches_by_piece(&pitches_by_piece);

        assert_eq!(chords.len(), 3);
        assert_eq!(chords[0].len(), 2);
        assert_eq!(chords[1].len(), 2);
        assert_eq!(chords[2].len(), 2);

        assert_eq!(chords[0], vec![Note {base_midi: 57, adjustment: 1}, Note {base_midi: 60, adjustment: 1}]);
        assert_eq!(chords[1], vec![Note {base_midi: 59, adjustment: 1}, Note {base_midi: 62, adjustment: 1}]);
        assert_eq!(chords[2], vec![Note {base_midi: 61, adjustment: 1}, Note {base_midi: 64, adjustment: 2}]);
    }

    #[test]
    fn test_chords_from_pitches_by_piece_with_different_length() {
        let cell_history1 = vec![chess::Cell::new("a2"), chess::Cell::new("a3")];
        let pitches1 = Note::get_pitches_from_cell_history(&cell_history1);
        let cell_history2 = vec![chess::Cell::new("c2"), chess::Cell::new("c2"), chess::Cell::new("c2")];
        let pitches2 = Note::get_pitches_from_cell_history(&cell_history2);
        let pitches_by_piece = vec![pitches1, pitches2];
        let chords = chords_from_pitches_by_piece(&pitches_by_piece);

        assert_eq!(chords.len(), 3);
        assert_eq!(chords[0].len(), 2);
        assert_eq!(chords[1].len(), 2);
        assert_eq!(chords[2].len(), 1);

        assert_eq!(chords[0], vec![Note {base_midi: 57, adjustment: 1}, Note {base_midi: 60, adjustment: 1}]);
        assert_eq!(chords[1], vec![Note {base_midi: 59, adjustment: 1},  Note {base_midi: 60, adjustment: 1}]);
        assert_eq!(chords[2], vec![Note {base_midi: 60, adjustment: 1}]);
    }
}
