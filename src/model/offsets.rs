use std::collections::HashMap;
pub use super::piece::Signature;

pub struct OriginalSqrs {
    orig_pos: HashMap<u8, u8>
}

impl OriginalSqrs {
    pub fn init() -> Self {
        let mut map = HashMap::new();
        map.insert(0, 0b10100);
        map.insert(1, 0b10010);
        map.insert(2, 0b10011);
        map.insert(3, 0b10101);
        map.insert(4, 0b10111);
        map.insert(5, 0b10011);
        map.insert(6, 0b10010);
        map.insert(7, 0b10100);
        map.insert(8, 0b10001);
        map.insert(9, 0b10001);
        map.insert(10, 0b10001);
        map.insert(11, 0b10001);
        map.insert(12, 0b10001);
        map.insert(13, 0b10001);
        map.insert(14, 0b10001);
        map.insert(15, 0b10001);
        map.insert(49, 0b11001);
        map.insert(48, 0b11001);
        map.insert(50, 0b11001);
        map.insert(51, 0b11001);
        map.insert(52, 0b11001);
        map.insert(53, 0b11001);
        map.insert(54, 0b11001);
        map.insert(55, 0b11001);
        map.insert(56, 0b11100);
        map.insert(57, 0b11010);
        map.insert(58, 0b11011);
        map.insert(59, 0b11101);
        map.insert(60, 0b11111);
        map.insert(61, 0b11011);
        map.insert(62, 0b11010);
        map.insert(63, 0b11100);

        OriginalSqrs {
            orig_pos: map,
        }
    }

    pub fn get_by_sqr(&self, sqr: u8) -> u8 {
        *self.orig_pos
        .get(&sqr)
        .unwrap_or(&255)
    }
    
}

pub struct Offsets {
    offs_coll: HashMap<u8, Vec<(i8, i8, i8)>>
}

impl Offsets {
    pub fn init() -> Self {
        let mut map = HashMap::new();
        map.insert(0b001, vec![(0, 2, 16), (0, 1, 8), (-1, 1, 7), (1, 1, 9)]);
        map.insert(0b010, vec![(-1, 2, 15), (1, 2, 17), (2, 1, 10), (-2, 1, 6), (2, -1, -6), (-2, -1, -10), (1, -2, -15), (-1, -2, -17)]);
        map.insert(0b011, vec![(-1, 1, 7),(1, -1, -7), (1, 1, 9), (-1, -1, -9)]);
        map.insert(0b100, vec![(0, 1, 8), (0, -1, -8), (1, 0, 1), (-1, 0, -1)]);
        map.insert(0b101, vec![(-1, 1, 7),(1, -1, -7), (1, 1, 9), (-1, -1, -9), (0, 1, 8), (0, -1, -8), (1, 0, 1), (-1, 0, -1)]);
        map.insert(0b111, vec![(-1, 1, 7),(1, -1, -7), (1, 1, 9), (-1, -1, -9), (0, 1, 8), (0, -1, -8), (1, 0, 1), (-1, 0, -1)]);
        Offsets {
            offs_coll: map
        }
    }

    pub fn get(&self, id: u8) -> &Vec<(i8, i8, i8)> {
        self.offs_coll
        .get(&id)
        .expect("Offsets.get() ex: no matching offset")
    }

    pub fn get_elegant(&self, _piece: Signature) -> &Vec<(i8, i8, i8)> {
        match _piece {
            Signature::Pawn(_) => self.get(0b001),
            Signature::Knight(_) => self.get(0b010),
            Signature::Bishop(_) => self.get(0b011),
            Signature::Rook(_) => self.get(0b100),
            Signature::Queen(_) => self.get(0b101),
            Signature::King(_) => self.get(0b111),
        }
    }
}
