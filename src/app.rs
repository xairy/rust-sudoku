use piston::input::*;
use graphics;
use opengl_graphics::GlGraphics;
use opengl_graphics::glyph_cache::GlyphCache;

use field;
use settings;

struct Vec2f {
    x: f64,
    y: f64
}

pub struct App {
    settings: settings::Settings,
    mouse_coords: Vec2f,
    field: field::Field,
    selected_cell: Option<field::Coords>,
    conflicting_cell: Option<field::Coords>
}

impl App {
    pub fn new(settings: settings::Settings) -> App {
        App {
            settings: settings,
            mouse_coords: Vec2f{ x: 0.0, y: 0.0 },
            field: field::Field::new(),
            selected_cell: None,
            conflicting_cell: None
        }
    }

    pub fn on_render(&mut self, args: &RenderArgs,
                     gl: &mut GlGraphics, cache: &mut GlyphCache) {
        gl.draw(args.viewport(), |c, g| {
            use graphics::*;
            clear([1.0; 4], g);

            let pointed_cell = field::Coords{
                x: (self.mouse_coords.x / self.settings.cell_size.x as f64)
                    .floor() as u8,
                y: (self.mouse_coords.y / self.settings.cell_size.y as f64)
                    .floor() as u8 };
            rectangle([0.95, 0.95, 0.95, 1.0],
                      [(pointed_cell.x as f64) * self.settings.cell_size.x,
                       (pointed_cell.y as f64) * self.settings.cell_size.y,
                       self.settings.cell_size.x, self.settings.cell_size.y],
                      c.transform, g);

            for y in 0..9 {
                for x in 0..9 {
                    let cell = self.field.get_cell(x, y);
                    if cell.fixed {
                        rectangle([0.9, 0.9, 0.9, 1.0],
                            [(x as f64) * self.settings.cell_size.x,
                             (y as f64) * self.settings.cell_size.y,
                             self.settings.cell_size.x,
                             self.settings.cell_size.y],
                            c.transform, g);
                    }
                }
            }

            if let Some(ref cell) = self.selected_cell {
                if let Some(digit) = self.field.get_cell(cell.x, cell.y).digit {
                    for y in 0..9 {
                        for x in 0..9 {
                            if let Some(other_digit) =
                                    self.field.get_cell(x, y).digit {
                                if other_digit == digit {
                                    rectangle([0.8, 0.8, 0.9, 1.0],
                                        [(x as f64) * self.settings.cell_size.x,
                                         (y as f64) * self.settings.cell_size.y,
                                         self.settings.cell_size.x,
                                         self.settings.cell_size.y],
                                        c.transform, g);
                                }
                            }
                        }
                    }
                }
            }

            if let Some(ref cell) = self.conflicting_cell {
                rectangle([0.9, 0.8, 0.8, 1.0],
                          [(cell.x as f64) * self.settings.cell_size.x,
                           (cell.y as f64) * self.settings.cell_size.y,
                           self.settings.cell_size.x, self.settings.cell_size.y],
                          c.transform, g);
            }

            if let Some(ref cell) = self.selected_cell {
                rectangle([0.8, 0.9, 0.8, 1.0],
                          [(cell.x as f64) * self.settings.cell_size.x,
                           (cell.y as f64) * self.settings.cell_size.y,
                           self.settings.cell_size.x, self.settings.cell_size.y],
                          c.transform, g);
            }

            for y in 0..9 {
                for x in 0..9 {
                    if let Some(ref digit) = self.field.cells[y][x].digit {
                        let transform = c.transform.trans(
                            (x as f64) * self.settings.cell_size.x +
                                self.settings.text_offset.x,
                            (y as f64) * self.settings.cell_size.y +
                                self.settings.text_offset.y);
                        let text = graphics::Text::new(self.settings.font_size);
                        text.draw(&digit.to_string(), cache,
                                  &c.draw_state, transform, g);
                    }
                }
            }

            for n in 1..9 {
                let mut thick = 2.0;
                if n % 3 == 0 {
                    thick = 8.0;
                }
                rectangle([0.0, 0.0, 0.0, 1.0],
                          [(n as f64) * self.settings.cell_size.x - thick / 2.0,
                           0.0, thick / 2.0, self.settings.wind_size.y],
                           c.transform, g);
                rectangle([0.0, 0.0, 0.0, 1.0],
                          [0.0, (n as f64) * self.settings.cell_size.y -
                                thick / 2.0,
                           self.settings.wind_size.x, thick / 2.0],
                           c.transform, g);
            }
        });
    }

    pub fn on_button_press(&mut self, button: &Button) {
        match button {
            &Button::Keyboard(key) => {
                self.on_key_down(&key);
            },
            &Button::Mouse(button) => {
                self.on_mouse_click(&button);
            }
            &Button::Joystick(_) => {}
        }
    }

    fn on_key_down(&mut self, pressed_key: &Key) {
        let key_digit_mapping = [
            (Key::D1, 1), (Key::D2, 2), (Key::D3, 3), 
            (Key::D4, 4), (Key::D5, 5), (Key::D6, 6), 
            (Key::D7, 7), (Key::D8, 8), (Key::D9, 9) ];
        for &(key, digit) in key_digit_mapping.iter() {
            if pressed_key == &key {
                if let Some(ref cell) = self.selected_cell {
                    if !self.field.get_cell(cell.x, cell.y).fixed {
                        match self.field.find_conflict(cell, digit) {
                            Some(coords) => {
                                self.conflicting_cell = Some(coords);
                            },
                            None => {
                                self.field.get_cell(cell.x, cell.y).digit =
                                    Some(digit);
                                self.conflicting_cell = None;
                            }
                        }
                    }
                }
            }
        }
        if pressed_key == &Key::Backspace {
            if let Some(ref cell) = self.selected_cell {
                if !self.field.get_cell(cell.x, cell.y).fixed {
                    self.field.get_cell(cell.x, cell.y).digit = None;
                    self.conflicting_cell = None;
                }
            }
        }
        if pressed_key == &Key::S {
            self.field.fill_solution();
            self.conflicting_cell = None;
            self.selected_cell = None;
        }
        if pressed_key == &Key::R {
            self.field.fill_random();
            self.conflicting_cell = None;
            self.selected_cell = None;
        }
        if pressed_key == &Key::Up {
            match self.selected_cell {
                Some(ref mut cell) => if cell.y > 0 { cell.y -= 1; },
                None => self.selected_cell = Some(field::Coords{ x: 0, y: 0})
            }
        }
        if pressed_key == &Key::Down {
            match self.selected_cell {
                Some(ref mut cell) => if cell.y < 8 { cell.y += 1; },
                None => self.selected_cell = Some(field::Coords{ x: 0, y: 0})
            }
        }
        if pressed_key == &Key::Left {
            match self.selected_cell {
                Some(ref mut cell) => if cell.x > 0 { cell.x -= 1; },
                None => self.selected_cell = Some(field::Coords{ x: 0, y: 0})
            }
        }
        if pressed_key == &Key::Right {
            match self.selected_cell {
                Some(ref mut cell) => if cell.x < 8 { cell.x += 1; },
                None => self.selected_cell = Some(field::Coords{ x: 0, y: 0})
            }
        }
    }

    fn on_mouse_click(&mut self, button: &MouseButton) {
        if let &MouseButton::Left = button {
            self.selected_cell = Some(field::Coords{
                x: (self.mouse_coords.x / self.settings.cell_size.x) as u8,
                y: (self.mouse_coords.y / self.settings.cell_size.y) as u8 });
        }
    }

    pub fn on_mouse_move(&mut self, args: &[f64; 2]) {
        self.mouse_coords.x = args[0];
        self.mouse_coords.y = args[1];
    }
}
