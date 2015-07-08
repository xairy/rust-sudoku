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

fn main() {
    let wind_size_x = 900.0;
    let wind_size_y = 900.0;

    let cell_size_x = wind_size_x / 9.0;
    let cell_size_y = wind_size_y / 9.0;

    let font_size = 64;

    let text_offset_x = 20.0;
    let text_offset_y = 75.0;

    let opengl = OpenGL::_3_2;
    let window: PistonWindow =
        WindowSettings::new("Sudoku",
                            [(wind_size_x as u32), (wind_size_y as u32)])
        .exit_on_esc(true)
        .opengl(opengl)
        .into();
    let ref mut gl = GlGraphics::new(opengl);

    let mut face = make_face("Verdana.ttf", font_size);

    let mut mouse_x = 0.0;
    let mut mouse_y = 0.0;

    let mut field = [[0u8; 9]; 9];
    field[0][0] = 1;
    field[3][0] = 2;
    field[3][5] = 9;

    let mut selected_cell_x = -1;
    let mut selected_cell_y = -1;

    for e in window.events() {
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                use graphics::*;
                clear([1.0; 4], g);

                let pointed_cell_x = (mouse_x / cell_size_x as f64).floor();
                let pointed_cell_y = (mouse_y / cell_size_y as f64).floor();
                rectangle([0.95, 0.95, 0.95, 1.0],
                          [pointed_cell_x * cell_size_x,
                           pointed_cell_y * cell_size_y,
                           cell_size_x, cell_size_y],
                          c.transform, g);

                if selected_cell_x != -1 && selected_cell_y != -1 {
                    rectangle([0.8, 0.8, 0.8, 1.0],
                              [(selected_cell_x as f64) * cell_size_x,
                               (selected_cell_y as f64) * cell_size_y,
                               cell_size_x, cell_size_y],
                              c.transform, g);
                }

                for row in 0..9 {
                    for col in 0..9 {
                        if field[row][col] != 0 {
                            let transform = c.transform.trans(
                                (col as f64) * cell_size_x + text_offset_x,
                                (row as f64) * cell_size_y + text_offset_y);
                            render_text(&mut face, g, transform,
                                        &field[row][col].to_string());
                        }
                    }
                }

                for n in 1..9 {
                    let mut thick = 2.0;
                    if n % 3 == 0 {
                        thick = 6.0;
                    }
                    rectangle([0.0, 0.0, 0.0, 1.0],
                              [(n as f64) * cell_size_x - thick / 2.0,
                               0.0, thick / 2.0, wind_size_y],
                               c.transform, g);
                    rectangle([0.0, 0.0, 0.0, 1.0],
                              [0.0, (n as f64) * cell_size_y - thick / 2.0,
                               wind_size_x, thick / 2.0],
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
                            selected_cell_x = (mouse_x / cell_size_x) as i32;
                            selected_cell_y = (mouse_y / cell_size_y) as i32;
                        },
                        _ => println!("Pressed mouse {:?}", mouse_button)
                    }
                },
                piston::input::Button::Keyboard(key) => {
                    match key {
                        piston::input::Key::D1 => {
                            if selected_cell_x != -1 && selected_cell_y != -1 {
                                field[selected_cell_y as usize]
                                     [selected_cell_x as usize] = 1;
                            }
                        },
                        _ => println!("Pressed {:?}", button)
                    }
                }
            }
        }

        if let Some(args) = e.mouse_cursor_args() {
            mouse_x = args[0];
            mouse_y = args[1];
            println!("Mouse: {} {}", mouse_x, mouse_y);
        }
    }
}
