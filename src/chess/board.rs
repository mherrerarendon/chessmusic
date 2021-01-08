use std::collections::HashMap;
use std::{error::Error};


struct Board {
    board: Vec<Vec<String>>,
}


impl Board {
    fn new() -> Board {
        let files = vec!["a", "b", "c", "d", "e", "f", "g", "h"];
        let mut rows = Vec::new();
        for n in 1..=8 {
            let mut row = Vec::new();
            let row_str = n.to_string();
            for file in files.iter() {
                let cell_name = format!("{}{}", file, row_str);
                row.push(cell_name);
            }

            rows.push(row);
        }

        Board {board: rows}
    }

    // fn file_to_index(the_file: &str) -> u32 {
    //     let files = vec!["a", "b", "c", "d", "e", "f", "g", "h"];
    //     files.iter().position(|&r| r == the_file).unwrap()
    // }

    // fn get_possible_moves(piece: &str, curr_cell: &str) -> Vec<String> {

    // }
}