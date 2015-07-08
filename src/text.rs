// Taken from:
// https://github.com/PistonDevelopers/piston-examples/blob/master/freetype

extern crate freetype;
extern crate find_folder;

use opengl_graphics::{ GlGraphics, Texture };
use graphics::math::Matrix2d;

pub fn make_face(font_filename: &str, font_size: u32) -> freetype::Face {
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let freetype = freetype::Library::init().unwrap();
    let font = assets.join(font_filename);
    let face = freetype.new_face(&font, 0).unwrap();
    face.set_pixel_sizes(0, font_size).unwrap();
    face
}

pub fn render_text(face: &mut freetype::Face, gl: &mut GlGraphics,
                   t: Matrix2d, text: &str) {
    use graphics::*;
    let mut x = 10;
    let mut y = 0;
    for ch in text.chars() {
        face.load_char(ch as usize, freetype::face::RENDER).unwrap();
        let g = face.glyph();

        let bitmap = g.bitmap();
        let texture = Texture::from_memory_alpha(bitmap.buffer(),
                                                 bitmap.width() as u32,
                                                 bitmap.rows() as u32).unwrap();
        Image::new_colored(color::BLACK).draw(
            &texture,
            default_draw_state(),
            t.trans((x + g.bitmap_left()) as f64,
                    (y - g.bitmap_top()) as f64),
            gl
        );

        x += (g.advance().x >> 6) as i32;
        y += (g.advance().y >> 6) as i32;
    }
}
