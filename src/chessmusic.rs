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

fn generate_pitches_by_pieces(pieces: &[(chess::PieceName, bool)], tx: mpsc::Sender<Vec<Pitch>>, moves: &Vec<(Move, Move)>) {
    crossbeam::scope(|s| {
        for (piece_name, white) in pieces.iter() {
            let tx1 = mpsc::Sender::clone(&tx);
            s.spawn(move |_| {
                let pitches = get_pitches_for_piece(*piece_name, *white, &moves);
                tx1.send(pitches).unwrap();
            });
        }
    }).unwrap();
}

fn receive_pitches_by_piece(pieces: &[(chess::PieceName, bool)], rx: mpsc::Receiver<Vec<Pitch>>) -> Vec<Vec<Pitch>> {
    let mut pitches_by_piece: Vec<Vec<Pitch>> = Vec::with_capacity(pieces.len());

    for _ in 0..pieces.len() {
        match rx.recv() {
            Ok(pitches) => pitches_by_piece.push(pitches),
            Err(the_error) => {
                println!("{}", the_error.to_string())
            }
        }
    }

    pitches_by_piece
}

fn chords_from_pitches_by_piece(pitches_by_piece: &Vec<Vec<Pitch>>) -> Vec<Vec<Pitch>> {
    let mut chords: Vec<Vec<Pitch>> = Vec::new();
    let longest_length = pitches_by_piece.iter().max_by_key(|pitches| pitches.len()).unwrap().len();
    for n in 0..longest_length {
        let mut chord: Vec<Pitch> = Vec::new();
        for pitches in pitches_by_piece.iter() {
            match pitches.get(n) {
                Some(pitch) => chord.push(*pitch),
                None => () // Nothing to add
            }
        }

        chords.push(chord);
    }
    chords
}

pub fn play_game(game_str: &str) {
    let str_moves = match pgn::parse_moves(game_str) {
        Ok(str_moves) => str_moves,
        Err(_the_error) => panic!("failed to parse moves")
    };
    let moves = chess::chess_move::Move::parse_moves(&str_moves);

    let (tx, rx): (mpsc::Sender<Vec<Pitch>>, mpsc::Receiver<Vec<Pitch>>) = mpsc::channel();
    generate_pitches_by_pieces(PIECES, tx, &moves);
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
        let pitches1 = Pitch::get_pitches_from_cell_history(&cell_history1);
        let cell_history2 = vec![chess::Cell::new("c2"), chess::Cell::new("c3"), chess::Cell::new("d4")];
        let pitches2 = Pitch::get_pitches_from_cell_history(&cell_history2);
        let pitches_by_piece = vec![pitches1, pitches2];
        let chords = chords_from_pitches_by_piece(&pitches_by_piece);

        assert_eq!(chords.len(), 3);
        assert_eq!(chords[0].len(), 2);
        assert_eq!(chords[1].len(), 2);
        assert_eq!(chords[2].len(), 2);

        assert_eq!(chords[0], vec![Pitch {base_midi: 57, adjustment: 1}, Pitch {base_midi: 60, adjustment: 1}]);
        assert_eq!(chords[1], vec![Pitch {base_midi: 59, adjustment: 1}, Pitch {base_midi: 62, adjustment: 1}]);
        assert_eq!(chords[2], vec![Pitch {base_midi: 61, adjustment: 1}, Pitch {base_midi: 64, adjustment: 2}]);
    }

    #[test]
    fn test_chords_from_pitches_by_piece_with_different_length() {
        let cell_history1 = vec![chess::Cell::new("a2"), chess::Cell::new("a3")];
        let pitches1 = Pitch::get_pitches_from_cell_history(&cell_history1);
        let cell_history2 = vec![chess::Cell::new("c2"), chess::Cell::new("c2"), chess::Cell::new("c2")];
        let pitches2 = Pitch::get_pitches_from_cell_history(&cell_history2);
        let pitches_by_piece = vec![pitches1, pitches2];
        let chords = chords_from_pitches_by_piece(&pitches_by_piece);

        assert_eq!(chords.len(), 3);
        assert_eq!(chords[0].len(), 2);
        assert_eq!(chords[1].len(), 2);
        assert_eq!(chords[2].len(), 1);

        assert_eq!(chords[0], vec![Pitch {base_midi: 57, adjustment: 1}, Pitch {base_midi: 60, adjustment: 1}]);
        assert_eq!(chords[1], vec![Pitch {base_midi: 59, adjustment: 1},  Pitch {base_midi: 60, adjustment: 1}]);
        assert_eq!(chords[2], vec![Pitch {base_midi: 60, adjustment: 1}]);
    }
}
