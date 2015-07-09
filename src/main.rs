extern crate piston;
extern crate piston_window;
extern crate graphics;
extern crate opengl_graphics;

use std::path::Path;

use piston_window::PistonWindow;
use piston::window::WindowSettings;
use piston::event::*;
use opengl_graphics::{ GlGraphics, OpenGL };
use opengl_graphics::glyph_cache::GlyphCache;

mod app;
mod field;
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

    let font_path = Path::new("assets/Verdana.ttf");
    let ref mut cache = GlyphCache::new(font_path).unwrap();

    let mut app = app::App::new(settings);

    for e in window.events() {
        if let Some(args) = e.render_args() {
            app.on_render(&args, gl, cache);
        }

        if let Some(button) = e.press_args() {
            app.on_button_press(&button);
        }

        if let Some(args) = e.mouse_cursor_args() {
            app.on_mouse_move(&args);
        }
    }
}
