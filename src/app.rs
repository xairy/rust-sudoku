extern crate graphics;
extern crate piston;
extern crate opengl_graphics;
extern crate freetype;

use piston::event::*;
use piston::input::*;
use opengl_graphics::GlGraphics;

use field;
use font;
use settings;

struct Vec2f {
    x: f64,
    y: f64
}

pub struct App {
    settings: settings::Settings,
    field: field::Field,
    selected_cell: Option<field::CellCoords>,
    mouse_coords: Vec2f
}

impl App {
    pub fn new(settings: settings::Settings) -> App {
        App {
            settings: settings,
            field: field::Field::new(),
            selected_cell: None,
            mouse_coords: Vec2f{ x: 0.0, y: 0.0 }
        }
    }

    pub fn on_render(&mut self, args: &RenderArgs,
                     gl: &mut GlGraphics, face: &mut freetype::Face) {
        gl.draw(args.viewport(), |c, g| {
            use graphics::*;
            clear([1.0; 4], g);

            let pointed_cell = field::CellCoords{
                x: (self.mouse_coords.x / self.settings.cell_size.x as f64)
                    .floor() as u8,
                y: (self.mouse_coords.y / self.settings.cell_size.y as f64)
                    .floor() as u8 };
            rectangle([0.95, 0.95, 0.95, 1.0],
                      [(pointed_cell.x as f64) * self.settings.cell_size.x,
                       (pointed_cell.y as f64) * self.settings.cell_size.y,
                       self.settings.cell_size.x, self.settings.cell_size.y],
                      c.transform, g);

            if let Some(ref cell) = self.selected_cell {
                rectangle([0.8, 0.8, 0.8, 1.0],
                          [(cell.x as f64) * self.settings.cell_size.x,
                           (cell.y as f64) * self.settings.cell_size.y,
                           self.settings.cell_size.x, self.settings.cell_size.y],
                          c.transform, g);
            }

            for row in 0..9 {
                for col in 0..9 {
                    if let Some(ref digit) = self.field.cells[row][col].digit {
                        let transform = c.transform.trans(
                            (col as f64) * self.settings.cell_size.x +
                                self.settings.text_offset.x,
                            (row as f64) * self.settings.cell_size.y +
                                self.settings.text_offset.y);
                        font::render_text(face, g, transform,
                                    &digit.to_string());
                    }
                }
            }

            for n in 1..9 {
                let mut thick = 2.0;
                if n % 3 == 0 {
                    thick = 6.0;
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
        }
    }

    fn on_key_down(&mut self, key: &Key) {
        match key {
            &Key::D1 => {
                if let Some(ref cell) = self.selected_cell {
                    self.field.cells[cell.y as usize]
                                    [cell.x as usize].digit = Some(1);
                }
            },
            _ => println!("Pressed {:?}", key)
        }
    }

    fn on_mouse_click(&mut self, button: &MouseButton) {
        match button {
            &MouseButton::Left => {
                println!("Pressed mouse left");
                self.selected_cell = Some(field::CellCoords{
                    x: (self.mouse_coords.x / self.settings.cell_size.x)
                        as u8,
                    y: (self.mouse_coords.y / self.settings.cell_size.y)
                        as u8 });
            },
            _ => println!("Pressed mouse {:?}", button)
        }
    }

    pub fn on_mouse_move(&mut self, args: &[f64; 2]) {
        self.mouse_coords.x = args[0];
        self.mouse_coords.y = args[1];
        println!("Mouse: {} {}", self.mouse_coords.x, self.mouse_coords.y);
    }
}
