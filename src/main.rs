extern crate piston;
extern crate piston_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate freetype;

use piston_window::PistonWindow;
use piston::window::WindowSettings;
use piston::event::*;
use opengl_graphics::{ GlGraphics, OpenGL };

mod app;
mod field;
mod font;
mod settings;

fn main() {
    let settings = settings::Settings::new();

    let opengl = OpenGL::_3_2;
    let window: PistonWindow =
        WindowSettings::new("Sudoku",
            [(settings.wind_size.x as u32), (settings.wind_size.y as u32)])
        .exit_on_esc(true)
        .opengl(opengl)
        .into();
    let ref mut gl = GlGraphics::new(opengl);
    let mut face = font::make_face("Verdana.ttf", settings.font_size);

    let mut app = app::App::new(settings);

    for e in window.events() {
        if let Some(args) = e.render_args() {
            app.on_render(&args, gl, &mut face);
        }

        if let Some(button) = e.press_args() {
            app.on_button_press(&button);
        }

        if let Some(args) = e.mouse_cursor_args() {
            app.on_mouse_move(&args);
        }
    }
}
