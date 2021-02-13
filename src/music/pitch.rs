use chess::cell::{Cell};

use crate::chess;

extern crate midir;

use std::{vec};


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pitch {
    pub base_midi: i32,
    pub adjustment: i32
}

impl Pitch {
    #[allow(dead_code)]
    fn new(file: char) -> Pitch {
        Pitch {
            base_midi: Pitch::file_to_midi(file),
            adjustment: 0
        }
    }
    fn new_with_cell(cell: &Cell) -> Pitch {
        Pitch {
            base_midi: Pitch::file_to_midi(cell.file),
            adjustment: cell.row - 1
        }
    }

    // transposition is x and y on a chess board
    fn new_with_cell_diff(&self, cell_diff: (i32, i32)) -> Pitch {
        let (x, y) = cell_diff;
         Pitch {
            base_midi: self.base_midi + (y * 2),
            adjustment: self.adjustment + x
        }
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

    pub fn get_pitches_from_cell_history(cell_history: &Vec<Cell>) -> Vec<Pitch> {
        let base_cell = cell_history[0];
        let base_pitch = Pitch::new_with_cell(&base_cell);
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
        let pitch = Pitch::new_with_cell(&Cell::new("a1"));
        assert_eq!(pitch.base_midi, 57);
        assert_eq!(pitch.adjustment, 0);
    }

    #[test]
    fn test_new_pitch_with_cell_diff() {
        let pitch = Pitch::new('a');
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
        let pitches = Pitch::get_pitches_from_cell_history(&cell_history);
        assert_eq!(pitches[0], Pitch {base_midi: 57, adjustment: 1});
        assert_eq!(pitches[1], Pitch {base_midi: 59, adjustment: 1});
        assert_eq!(pitches[2], Pitch {base_midi: 61, adjustment: 1});
    }
}
