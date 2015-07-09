pub struct Coords {
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

    pub fn get_cell(&mut self, x: u8, y: u8) -> &mut Cell {
        &mut self.cells[y as usize][x as usize]
    }

    pub fn find_conflicts(&mut self, coords: &Coords,
                          digit: u8) -> Option<Coords> {
        for x in 0..8 {
            if x != coords.x {
                if let Some(cell_digit) = self.get_cell(x, coords.y).digit {
                    if cell_digit == digit {
                        return Some(Coords{ x: x, y: coords.y});
                    }
                }
            }
        }

        for y in 0..8 {
            if y != coords.y {
                if let Some(cell_digit) = self.get_cell(coords.x, y).digit {
                    if cell_digit == digit {
                        return Some(Coords{ x: coords.x, y: y});
                    }
                }
            }
        }

        let section = Coords{ x: coords.x / 3, y: coords.y / 3};
        for x in section.x * 3 .. (section.x + 1) * 3 {
            for y in section.y * 3 .. (section.y + 1) * 3 {
                if x != coords.x && y != coords.y {
                    if let Some(cell_digit) = self.get_cell(x, y).digit {
                        if cell_digit == digit {
                            return Some(Coords{ x: x, y: y});
                        }
                    }
                }
            }
        }

        println!("{} {}", section.x, section.y);
        
        None
    }
}
