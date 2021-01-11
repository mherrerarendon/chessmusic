use std::collections::HashMap;
use super::cell::Cell;

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
    if char_to_role.contains_key(role_char) {
        return char_to_role[role_char];
    }
    else {
        return Role::Pawn;
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum MoveType {
    Simple,
    Take,
    CastleKing,
    CastleQueen
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PieceName {
    Apawn, Bpawn, Cpawn, Dpawn, Epawn, Fpawn, Gpawn, Hpawn,
    Qrook, Qknight, Qbishop, Queen, King, Kbishop, Kknight, Krook
}
