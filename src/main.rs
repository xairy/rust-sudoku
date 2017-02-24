extern crate piston;
extern crate piston_window;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate rand;

use piston::window::WindowSettings;
use piston::input::*;
use piston::event_loop::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{ GlGraphics, OpenGL };
use opengl_graphics::glyph_cache::GlyphCache;
use std::path::Path;

mod app;
mod field;
mod settings;

fn main() {
    let settings = settings::Settings::new();

    let opengl = OpenGL::V3_2;
    let mut window: GlutinWindow =
        WindowSettings::new("Sudoku",
            [(settings.wind_size.x as u32), (settings.wind_size.y as u32)])
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();
    let ref mut gl = GlGraphics::new(opengl);

    let font_path = Path::new("assets/Verdana.ttf");
    let ref mut cache = GlyphCache::new(font_path).unwrap();

    let mut app = app::App::new(settings);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
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
