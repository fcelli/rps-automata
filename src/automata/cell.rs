#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rgba_returns_correct_color() {
        let cases = [
            (Cell::Red, [255, 0, 0, 255]),
            (Cell::Green, [0, 255, 0, 255]),
            (Cell::Blue, [0, 0, 255, 255]),
        ];

        for (cell, color) in cases {
            assert_eq!(cell.rgba(), color)
        }
    }

    #[test]
    fn predator_relationships_are_correct() {
        let cases = [
            (Cell::Red, Cell::Blue),
            (Cell::Blue, Cell::Green),
            (Cell::Green, Cell::Red),
        ];

        for (cell, predator) in cases {
            assert_eq!(cell.predator(), predator);
        }
    }
}
