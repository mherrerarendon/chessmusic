use std::char;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell {
    pub file: char,
    pub row: i32
}

impl Cell {
    pub fn new() -> Cell {
        Cell {
            file: ' ',
            row: 0
        }
    }

    pub fn new_with_values(file: char, row: i32) -> Cell {
        Cell {
            file: file,
            row: row
        }
    }

    pub fn new_from_cell(cell: &Cell, file_offset: i32, row_offset: i32) -> Option<Cell> {
        let new_row = cell.row + file_offset;
        if new_row  < 1 || new_row > 8 {
            return None;
        }
        let char_as_digit = match cell.file.to_digit(10) {
            Some(char_as_digit) => char_as_digit,
            None => return None
        };
        
        let char_with_offset = (char_as_digit as i32 + file_offset) as u32;
        let new_file = match char::from_digit(char_with_offset, 10) {
            Some(new_char) => new_char,
            None=> return None
        };

        if ! ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'].contains(&new_file) {
            return None;
        }

        Some(Cell {file: new_file, row: new_row})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_from_cell() {
        let cell = Cell::new_with_values('a', 2);
        assert_eq!(cell.file, 'a');
        assert_eq!(cell.row, 2);

        let new_cell = Cell::new_from_cell(&cell, 0, 1);
        assert_ne!(new_cell.is_none());
        assert_eq!(new_cell.file, 'a');
        assert_eq!(new_cell.row, 3);
    }
}