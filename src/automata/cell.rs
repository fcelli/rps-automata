#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Red,
    Green,
    Blue,
}

impl Cell {
    pub fn rgba(self) -> [u8; 4] {
        match self {
            Cell::Red => [255, 0, 0, 255],
            Cell::Green => [0, 255, 0, 255],
            Cell::Blue => [0, 0, 255, 255],
        }
    }

    pub fn predator(self) -> Cell {
        match self {
            Cell::Red => Cell::Blue,
            Cell::Green => Cell::Red,
            Cell::Blue => Cell::Green,
        }
    }
}
