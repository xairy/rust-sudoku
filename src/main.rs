extern crate piston_window;
extern crate rand;

use piston_window::*;
use std::path::Path;

mod app;
mod field;
mod settings;

fn main() {
    let settings = settings::Settings::new();

    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new(
        "Sudoku",
        [(settings.wind_size.x as u32), (settings.wind_size.y as u32)],
    )
    .exit_on_esc(true)
    .resizable(false)
    .graphics_api(opengl)
    .build()
    .unwrap();

    let font_path = Path::new("assets/Verdana.ttf");
    let mut cache = window.load_font(font_path).unwrap();

    let mut app = app::App::new(settings);

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, device| {
            app.on_render_new(c, g, &mut cache);

            cache.factory.encoder.flush(device);
        });

        if let Some(button) = e.press_args() {
            app.on_button_press(&button);
        }

        if let Some(args) = e.mouse_cursor_args() {
            app.on_mouse_move(&args);
        }
    }
}
