use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Role {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King
}

pub fn role_char_to_role(role_char: &str) -> Role {
    let char_to_role: HashMap<&str, Role> = [("", Role::Pawn), ("B", Role::Bishop), ("N", Role::Knight), ("R", Role::Rook), ("Q", Role::Queen), ("K", Role::King)]
        .iter().cloned().collect();
    char_to_role[role_char]
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum MoveType {
    Simple,
    Take,
    CastleKing,
    CastleQueen
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell {
    pub file: char,
    pub row: u32
}

impl Cell {
    pub fn new() -> Cell {
        Cell {
            file: ' ',
            row: 0
        }
    }

    pub fn new_with_values(file: &char, row: &u32) -> Cell {
        Cell {
            file: *file,
            row: *row
        }
    }
}

struct Piece {
    white: bool, 
    role: Role
}