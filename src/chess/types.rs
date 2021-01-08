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

pub struct Cell {
    pub file: String,
    pub row: u32
}

impl Cell {
    pub fn new() -> Cell {
        Cell {
            file: String::from(""),
            row: 0
        }
    }
}

struct Piece {
    white: bool, 
    role: Role
}