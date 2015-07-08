extern crate piston;
extern crate piston_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate freetype;

use piston_window::PistonWindow;
use piston::window::WindowSettings;
use piston::event::*;
use opengl_graphics::{ GlGraphics, OpenGL };

mod text;
use text::*;

struct Vec2f {
    x: f64,
    y: f64
}

struct CellCoords {
    x: u8,
    y: u8
}

#[derive(Copy, Clone)]
struct Cell {
    digit: Option<u8>,
    fixed: bool
}

struct Field {
    cells: [[Cell; 9]; 9]
}

impl Field {
    fn new() -> Field {
        Field {
            cells: [[Cell{ digit: None, fixed: false }; 9]; 9]
        }
    }
}

fn main() {
    let wind_size = Vec2f{ x: 900.0, y: 900.0 };
    let cell_size = Vec2f{ x: wind_size.x / 9.0, y: wind_size.y / 9.0 };
    let font_size = 64;
    let text_offset = Vec2f{ x: 20.0, y: 75.0 };

    let opengl = OpenGL::_3_2;
    let window: PistonWindow =
        WindowSettings::new("Sudoku",
                            [(wind_size.x as u32), (wind_size.y as u32)])
        .exit_on_esc(true)
        .opengl(opengl)
        .into();
    let ref mut gl = GlGraphics::new(opengl);
    let mut face = make_face("Verdana.ttf", font_size);

    let mut mouse_coords = Vec2f{ x: 0.0, y: 0.0 };
    let mut selected_cell: Option<CellCoords> = None;

    let mut field = Field::new();
    field.cells[5][1].digit = Some(5);

    for e in window.events() {
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                use graphics::*;
                clear([1.0; 4], g);

                let pointed_cell = CellCoords{
                    x: (mouse_coords.x / cell_size.x as f64).floor() as u8,
                    y: (mouse_coords.y / cell_size.y as f64).floor() as u8 };
                rectangle([0.95, 0.95, 0.95, 1.0],
                          [(pointed_cell.x as f64) * cell_size.x,
                           (pointed_cell.y as f64) * cell_size.y,
                           cell_size.x, cell_size.y],
                          c.transform, g);

                if let Some(ref cell) = selected_cell {
                    rectangle([0.8, 0.8, 0.8, 1.0],
                              [(cell.x as f64) * cell_size.x,
                               (cell.y as f64) * cell_size.y,
                               cell_size.x, cell_size.y],
                              c.transform, g);
                }

                for row in 0..9 {
                    for col in 0..9 {
                        if let Some(ref digit) = field.cells[row][col].digit {
                            let transform = c.transform.trans(
                                (col as f64) * cell_size.x + text_offset.x,
                                (row as f64) * cell_size.y + text_offset.y);
                            render_text(&mut face, g, transform,
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
                              [(n as f64) * cell_size.x - thick / 2.0,
                               0.0, thick / 2.0, wind_size.y],
                               c.transform, g);
                    rectangle([0.0, 0.0, 0.0, 1.0],
                              [0.0, (n as f64) * cell_size.y - thick / 2.0,
                               wind_size.x, thick / 2.0],
                               c.transform, g);
                }
            });
        }

        if let Some(button) = e.press_args() {
            match button {
                piston::input::Button::Mouse(mouse_button) => {
                    match mouse_button {
                        piston::input::MouseButton::Left => {
                            println!("Pressed Mouse::Left");
                            selected_cell = Some(CellCoords{
                                x: (mouse_coords.x / cell_size.x) as u8,
                                y: (mouse_coords.y / cell_size.y) as u8 });
                        },
                        _ => println!("Pressed mouse {:?}", mouse_button)
                    }
                },
                piston::input::Button::Keyboard(key) => {
                    match key {
                        piston::input::Key::D1 => {
                            if let Some(ref cell) = selected_cell {
                                field.cells[cell.y as usize]
                                           [cell.x as usize].digit = Some(1);
                            }
                        },
                        _ => println!("Pressed {:?}", button)
                    }
                }
            }
        }

        if let Some(args) = e.mouse_cursor_args() {
            mouse_coords.x = args[0];
            mouse_coords.y = args[1];
            println!("Mouse: {} {}", mouse_coords.x, mouse_coords.y);
        }
    }
}
