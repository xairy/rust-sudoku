use rand;
use rand::Rng;

pub struct Coords {
    pub x: u8,
    pub y: u8
}

#[derive(Copy, Clone)]
pub struct Cell {
    pub digit: Option<u8>,
    pub fixed: bool
}

#[derive(Copy, Clone)]
pub struct Field {
    pub cells: [[Cell; 9]; 9]
}

impl Field {
    pub fn new() -> Field {
        let mut field = Field {
            cells: [[Cell{ digit: None, fixed: false }; 9]; 9]
        };
        field.fill_random();
        field
    }

    pub fn get_cell(&mut self, x: u8, y: u8) -> &mut Cell {
        &mut self.cells[y as usize][x as usize]
    }

    pub fn find_conflict(&mut self, coords: &Coords,
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

    pub fn clear(&mut self) {
        for y in 0..9 {
            for x in 0..9 {
                self.cells[x][y] = Cell{ digit: None, fixed: false };
            }
        }
    }

    pub fn fill_random(&mut self) {
        self.clear();

        let x = rand::thread_rng().gen_range(0u8, 9u8);
        let y = rand::thread_rng().gen_range(0u8, 9u8);
        let digit = rand::thread_rng().gen_range(1u8, 10u8);
        self.get_cell(x, y).digit = Some(digit);

        let solution = self.find_solution().unwrap();
        self.cells = solution.cells;

        loop {
            let mut x;
            let mut y;
            let digit;

            loop {
                x = rand::thread_rng().gen_range(0u8, 9u8);
                y = rand::thread_rng().gen_range(0u8, 9u8);
                if self.get_cell(x, y).digit.is_none() {
                    continue;
                }
                digit = self.get_cell(x, y).digit.unwrap();
                self.get_cell(x, y).digit = None;
                break;
            }

            let solutions = self.find_solutions(2);
            if solutions.len() == 1 {
                continue;
            }
            self.get_cell(x, y).digit = Some(digit);

            break;
        }

        // FIXME(xairy): generates perfect sudoku, but slow.
        /*
        let mut cells = Vec::new();
        for y in 0..9 {
            for x in 0..9 {
                cells.push((x, y));
            }
        }
        rand::thread_rng().shuffle(&mut cells);

        for &(x, y) in cells.iter() {
            let digit = self.get_cell(x, y).digit.unwrap();
            self.get_cell(x, y).digit = None;
            let solutions = self.find_solutions(2);
            if solutions.len() > 1 {
                self.get_cell(x, y).digit = Some(digit);
            }
        }
        */


        for y in 0..9 {
            for x in 0..9 {
                if self.get_cell(x, y).digit.is_some() {
                    self.get_cell(x, y).fixed = true;
                }
            }
        }
    }

    pub fn fill_solution(&mut self) {
        if let Some(s) = self.find_solution() {
            self.cells = s.cells;
        }
    }

    pub fn find_solution(&mut self) -> Option<Field> {
        let solutions = self.find_solutions(1);
        if solutions.len() > 0 {
            return Some(solutions[0]);
        }
        None
    }

    fn find_solutions(&mut self, stop_at: u32) -> Vec<Field> {
        let mut solutions = Vec::new();
        let mut field = self.clone();
        field.find_solutions_impl(&mut solutions, stop_at);
        solutions
    }

    fn find_solutions_impl(&mut self, solutions: &mut Vec<Field>,
                           stop_at: u32) -> bool {
        let mut empty_cell: Option<Coords> = None;
        'outer: for y in 0..9 {
            'inner: for x in 0..9 {
                if self.get_cell(x, y).digit.is_none() {
                    empty_cell = Some(Coords{ x: x, y: y });
                    break 'outer;
                }
            }
        }

        if empty_cell.is_none() {
            solutions.push(self.clone());
            return solutions.len() >= (stop_at as usize);
        }
        let coords = empty_cell.unwrap();

        let mut digits: Vec<u8> = (1..10).collect();
        rand::thread_rng().shuffle(&mut digits);

        for &digit in digits.iter() {
            if self.find_conflict(&coords, digit).is_none() { 
                self.get_cell(coords.x, coords.y).digit = Some(digit);
                if self.find_solutions_impl(solutions, stop_at) {
                    return true;
                }
                self.get_cell(coords.x, coords.y).digit = None;
            }
        }

        false
    }
}
