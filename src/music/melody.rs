use super::Note;
use super::super::chess::Cell;

pub struct Melody {
    pub notes: Vec<Note>
}

impl Melody {
    pub fn new(move_history_with_captures: &[(Cell, bool)], initial_note: Note) -> Melody {
        Melody {
            notes: Melody::compose_with_cell_history(move_history_with_captures, initial_note)
        }
    }

    fn compose_with_cell_history(move_history_with_captures: &[(Cell, bool)], initial_note: Note) -> Vec<Note> {
        let mut notes = vec![initial_note];
        let mut last_cell_opt: Option<Cell> = None;
        for (cell, capture) in move_history_with_captures {
            if let Some(last_cell) = last_cell_opt {
                let (x_diff, y_diff) = last_cell.get_cell_diff(cell);
                let new_note = notes.last().unwrap().new_with_transpsition(y_diff, x_diff);
                notes.push(new_note);
                last_cell_opt = Some(*cell);
            } else {
                last_cell_opt = Some(*cell);
            }
        }

        notes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compose_with_cell_history_vertical() {
        let starting_note = Note::new(50);
        let cell_history = &[Cell::new("a2"), Cell::new("a3"), Cell::new("a4")];
        let melody = Melody::compose_with_cell_history(cell_history, starting_note);
        assert_eq!(melody.len(), 3);
        assert_eq!(melody[0].as_midi(), 50);
        assert_eq!(melody[1].as_midi(), 52);
        assert_eq!(melody[2].as_midi(), 54);
    }

    #[test]
    fn test_compose_with_cell_history_horizontal() {
        let starting_note = Note::new(50);
        let cell_history = &[Cell::new("a2"), Cell::new("b2"), Cell::new("c2")];
        let melody = Melody::compose_with_cell_history(cell_history, starting_note);
        assert_eq!(melody.len(), 3);
        assert_eq!(melody[0].as_midi(), 50);
        assert_eq!(melody[1].as_midi(), 51);
        assert_eq!(melody[2].as_midi(), 52);
    }

    #[test]
    fn test_compose_with_cell_history_diagonal() {
        let starting_note = Note::new(50);
        let cell_history = &[Cell::new("a2"), Cell::new("b3"), Cell::new("c4")];
        let melody = Melody::compose_with_cell_history(cell_history, starting_note);
        assert_eq!(melody.len(), 3);
        assert_eq!(melody[0].as_midi(), 50);
        assert_eq!(melody[1].as_midi(), 53);
        assert_eq!(melody[2].as_midi(), 56);
    }
}