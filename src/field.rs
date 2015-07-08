pub struct CellCoords {
    pub x: u8,
    pub y: u8
}

#[derive(Copy, Clone)]
pub struct Cell {
    pub digit: Option<u8>,
    pub fixed: bool
}

pub struct Field {
    pub cells: [[Cell; 9]; 9]
}

impl Field {
    pub fn new() -> Field {
        Field {
            cells: [[Cell{ digit: None, fixed: false }; 9]; 9]
        }
    }
}
