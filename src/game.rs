use std::collections::HashMap;

struct MovesByPiece {
    pieces: HashMap<uint32, Vec<String>>,
}

impl MovesByPiece {
    fn add_move(&self, the_move: &str) {
        self.pieces.insert(1, String::from(the_move));
    }
}