use chess::cell::{Cell};

use crate::chess;

extern crate midir;

use std::{vec};


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Note {
    pub base_midi: i32,
    pub adjustment: i32,
    pub velocity: i32
}

impl Note {
    pub fn new(midi_note: i32) -> Note {
        Note {
            base_midi: midi_note,
            adjustment: 0,
            velocity: 80
        }
    }
    #[allow(dead_code)]
    fn new_with_file(file: char) -> Note {
        Note {
            base_midi: Note::file_to_midi(file),
            adjustment: 0,
            velocity: 80
        }
    }
    pub fn new_with_cell(cell: &Cell) -> Note {
        Note {
            base_midi: Note::file_to_midi(cell.file),
            adjustment: cell.row - 1,
            velocity: 80
        }
    }

    // transposition is x and y on a chess board
    fn new_with_cell_diff(&self, cell_diff: (i32, i32)) -> Note {
        let (x, y) = cell_diff;
         Note {
            base_midi: self.base_midi + (y * 2),
            adjustment: self.adjustment + x,
            velocity: 80
        }
    }

    pub fn new_with_transpsition(&self, whole_steps: i32, half_steps: i32) -> Note {
        return self.new_with_cell_diff((half_steps, whole_steps));
    }

    fn file_to_midi(file: char) -> i32 {
        let midi_note = match file {
            'a' => 57,
            'b' => 59,
            'c' => 60,
            'd' => 62,
            'e' => 64,
            'f' => 65,
            'g' => 67,
            'h' => 69,
            _ => panic!("unexpected file {}", file)
        };
        midi_note
    }

    pub fn as_midi(&self) -> u8 {
        (self.base_midi + self.adjustment) as u8
    }

    pub fn get_pitches_from_cell_history(cell_history: &Vec<Cell>) -> Vec<Note> {
        let base_cell = cell_history[0];
        let base_pitch = Note::new_with_cell(&base_cell);
        let mut pitches = vec![base_pitch];

        for cell in cell_history[1..].iter() {
            let cell_diff = base_cell.get_cell_diff(cell);
            let new_pitch = pitches[0].new_with_cell_diff(cell_diff);
            pitches.push(new_pitch);
        }
        pitches
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_pitch_with_cell() {
        let pitch = Note::new_with_cell(&Cell::new("a1"));
        assert_eq!(pitch.base_midi, 57);
        assert_eq!(pitch.adjustment, 0);
    }

    #[test]
    fn test_new_pitch_with_cell_diff() {
        let pitch = Note::new_with_file('a');
        let new_pitch = pitch.new_with_cell_diff((0, 1));
        assert_eq!(new_pitch.base_midi, 59);
        assert_eq!(new_pitch.adjustment, 0);

        let new_pitch = pitch.new_with_cell_diff((1, 0));
        assert_eq!(new_pitch.base_midi, 57);
        assert_eq!(new_pitch.adjustment, 1);
    }

    #[test]
    fn test_get_pitches_from_cell_history() {
        let cell_history = vec![Cell::new("a2"), Cell::new("a3"), Cell::new("a4")];
        let pitches = Note::get_pitches_from_cell_history(&cell_history);
        assert_eq!(pitches[0], Note {base_midi: 57, adjustment: 1, velocity: 80});
        assert_eq!(pitches[1], Note {base_midi: 59, adjustment: 1, velocity: 80});
        assert_eq!(pitches[2], Note {base_midi: 61, adjustment: 1, velocity: 80});
    }
}
