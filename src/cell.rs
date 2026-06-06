#[derive(Clone, Copy)]
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

    pub fn beats(self, other: Cell) -> bool {
        matches!(
            (self, other),
            (Cell::Red, Cell::Green) | (Cell::Green, Cell::Blue) | (Cell::Blue, Cell::Red),
        )
    }
}
