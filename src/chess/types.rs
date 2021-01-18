use std::collections::HashMap;
use std::fmt;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Role {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King
}

impl fmt::Debug for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let role = match *self {
            Role::Pawn => "Pawn",
            Role::Bishop => "Bishop",
            Role::Knight => "Knight",
            Role::Rook => "Rook",
            Role::Queen => "Queen",
            Role::King => "King"
        };
        f.debug_struct("Role")
         .field("name", &role)
         .finish()
    }
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
    None,
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
