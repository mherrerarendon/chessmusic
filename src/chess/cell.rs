use std::char;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell {
    pub file: char,
    pub row: i32
}

impl Cell {
    pub fn new(cell_name: &str) -> Cell {
        Cell {
            file: cell_name.chars().nth(0).unwrap(),
            row: cell_name.chars().nth(1).unwrap().to_digit(10).unwrap() as i32
        }
    }

    pub fn new_with_name(cell_name: &str) -> Cell {
        Cell {
            file: cell_name.chars().nth(0).unwrap(),
            row: cell_name.chars().nth(0).unwrap().to_digit(10).unwrap() as i32
        }
    }

    fn file_with_offset(file: char, offset: i32) -> Option<char> {
        let char_as_digit = file as i32;
        let char_with_offset = char_as_digit + offset;
        let new_file = if char_with_offset > 0 {char_with_offset as u8 as char} else {return None};

        if ! ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'].contains(&new_file) {
            return None;
        }

        Some(new_file)
    }

    pub fn new_from_cell(cell: &Cell, file_offset: i32, row_offset: i32) -> Option<Cell> {
        let new_row = cell.row + row_offset;
        if new_row  < 1 || new_row > 8 {
            return None;
        }
       
        let new_file_option = if file_offset == 0 {Some(cell.file)} else {Cell::file_with_offset(cell.file, file_offset)};
        let new_file = match new_file_option {
            Some(file) => file,
            None => return None
        };

        Some(Cell {file: new_file, row: new_row})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_with_values() {
        let cell = Cell::new("a2");
        assert_eq!(cell.file, 'a');
        assert_eq!(cell.row, 2);
    }

    #[test]
    fn test_new_from_cell_with_row_offset() {
        let cell = Cell::new("a2");
        match Cell::new_from_cell(&cell, 0, 1) {
            Some(new_cell) => {
                assert_eq!(new_cell.file, 'a');
                assert_eq!(new_cell.row, 3);
            },
            None => assert!(false)
        };
    }

    #[test]
    fn test_new_from_cell_with_file_offset() {
        let cell = Cell::new("a2");
        match Cell::new_from_cell(&cell, 1, 0) {
            Some(new_cell) => {
                assert_eq!(new_cell.file, 'b');
                assert_eq!(new_cell.row, 2);
            },
            None => assert!(false)
        };
    }

    #[test]
    fn test_new_from_cell_with_file_and_row_offsets() {
        let cell = Cell::new("a2");
        match Cell::new_from_cell(&cell, 1, 1) {
            Some(new_cell) => {
                assert_eq!(new_cell.file, 'b');
                assert_eq!(new_cell.row, 3);
            },
            None => assert!(false)
        };
    }

    #[test]
    fn test_new_from_cell_with_negative_offsets() {
        let cell = Cell::new("b2");
        match Cell::new_from_cell(&cell, -1, -1) {
            Some(new_cell) => {
                assert_eq!(new_cell.file, 'a');
                assert_eq!(new_cell.row, 1);
            },
            None => assert!(false)
        };
    }

    #[test]
    fn test_new_from_cell_to_invalid_row() {
        let cell = Cell::new("a1");
        match Cell::new_from_cell(&cell, 0, -1) {
            Some(_new_cell) => {
                assert!(false)
            },
            None => assert!(true)
        };

        match Cell::new_from_cell(&cell, 0, 8) {
            Some(_new_cell) => {
                assert!(false)
            },
            None => assert!(true)
        };
    }

    #[test]
    fn test_new_from_cell_to_invalid_file() {
        let cell = Cell::new("a1");
        match Cell::new_from_cell(&cell, -1, 0) {
            Some(_new_cell) => {
                assert!(false)
            },
            None => assert!(true)
        };

        match Cell::new_from_cell(&cell, 8, 0) {
            Some(_new_cell) => {
                assert!(false)
            },
            None => assert!(true)
        };
    }

    #[test]
    fn test_new_cell() {
        let cell = Cell::new("a1");
        assert_eq!(cell, Cell {file: 'a', row: 1});
    }
}