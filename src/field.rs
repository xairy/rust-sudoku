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
        let mut field = Field {
            cells: [[Cell{ digit: None, fixed: false }; 9]; 9]
        };
        field.fill_example();
        field
    }

    pub fn fill_example(&mut self) {
        let example = [
            [7, 2, 9, 1, 8, 4, 0, 5, 0],
            [0, 0, 4, 3, 0, 0, 0, 1, 7],
            [0, 0, 0, 0, 0, 0, 9, 0, 2],
            [0, 0, 0, 9, 5, 0, 0, 0, 0],
            [3, 8, 0, 4, 0, 1, 0, 7, 9],
            [0, 0, 0, 0, 2, 8, 0, 0, 0],
            [8, 0, 6, 0, 0, 0, 0, 0, 0],
            [5, 9, 0, 0, 0, 7, 6, 0, 0],
            [0, 7, 0, 6, 4, 5, 1, 9, 8]
        ];
        for y in 0..9 {
            for x in 0..9 {
                if example[y][x] != 0 {
                    self.cells[y][x].digit = Some(example[y][x]);
                    self.cells[y][x].fixed = true;
                }
            }
        }
    }

    pub fn get_cell(&mut self, x: u8, y: u8) -> &mut Cell {
        &mut self.cells[y as usize][x as usize]
    }

    pub fn find_conflicts(&mut self, coords: &Coords,
                          digit: u8) -> Option<Coords> {
        for x in 0..9 {
            if x != coords.x {
                if let Some(cell_digit) = self.get_cell(x, coords.y).digit {
                    if cell_digit == digit {
                        return Some(Coords{ x: x, y: coords.y});
                    }
                }
            }
        }

        for y in 0..9 {
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
                if x != coords.x || y != coords.y {
                    if let Some(cell_digit) = self.get_cell(x, y).digit {
                        if cell_digit == digit {
                            return Some(Coords{ x: x, y: y});
                        }
                    }
                }
            }
        }

        None
    }

    pub fn fill_solution(&mut self) -> bool {
        let mut empty_cell: Option<Coords> = None;
        'outer: for y in 0..9 {
            'inner: for x in 0..9 {
                if let None = self.get_cell(x, y).digit {
                    empty_cell = Some(Coords{ x: x.clone(), y: y.clone() });
                    break 'outer;
                }
            }
        }

        if let None = empty_cell {
            return true;
        }
        let coords = empty_cell.unwrap();

        for digit in 1..10 {
            if let None = self.find_conflicts(&coords, digit) { 
                self.get_cell(coords.x, coords.y).digit = Some(digit);
                if self.fill_solution() {
                    return true;
                }
                self.get_cell(coords.x, coords.y).digit = None;
            }
        }

        false
    }
}
