use std::collections::HashMap;
pub use super::piece::Signature;

pub struct Offsets {
    offs_coll: HashMap<u8, Vec<i8>>
}

impl Offsets {
    fn init() -> Self {
        let mut map = HashMap::new();
        map.insert(0b001, vec![8, 16, 7, 9]);
        map.insert(0b010, vec![15, 17, 10, 6, -6, -10, -15, -17]);
        map.insert(0b011, vec![7, -7, 9, -9]);
        map.insert(0b100, vec![8, -8, 1, -1]);
        map.insert(0b101, vec![7, -7, 9, -9, 8, -8, 1, -1]);
        map.insert(0b111, vec![1, -1, 8, -8, 7, -7, 9, -9]);
        Offsets {
            offs_coll: map
        }
    }

    fn get(&self, id: u8) -> &Vec<i8> {
        self.offs_coll
        .get(&id)
        .expect("Offsets.get() ex: no matching offset for id = {:b}")
    }

    fn get_elegant(&self, _piece: Signature) -> &Vec<i8> {
        match _piece {
            Signature::Pawn(_) => self.get(0b001),
            Signature::Knight(_) => self.get(0b010),
            Signature::Bishop(_) => self.get(0b011),
            Signature::Rook(_) => self.get(0b100),
            Signature::Queen(_) => self.get(0b101),
            Signature::King(_) => self.get(0b111),
            _     => panic!("Piece.get_figure() ex: No such figure")
        }
    }
}
