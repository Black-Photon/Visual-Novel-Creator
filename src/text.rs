use freetype::{Face, Library};
use std::ffi::OsStr;

pub(crate) fn create_font<P>(library: Library, name: P, resolution: u32) -> Face
    where P: AsRef<OsStr>
{
    let face = library.new_face(name, 0).unwrap();
    face.set_char_size(40 * 64, 0, resolution, resolution).unwrap();
    return face;
}

pub(crate) fn load_char(face: Face, c: char) -> (Vec<u8>, f32, f32, f32) {
    face.load_char(c as usize, freetype::face::LoadFlag::RENDER).unwrap();
    let glyph = face.glyph();
    let bitmap = glyph.bitmap();

    let mut figure: Vec<u8> = Vec::new();
    let w = bitmap.width() as usize;
    let x_max = w;
    let y_max = bitmap.rows() as usize;

    for j in 0 .. y_max {
        for i in 0 .. x_max {
            for _ in 0 .. 4 {
                figure.push(bitmap.buffer()[(y_max - j - 1) * w + i]);
            }
        }
    }

    let width = bitmap.width() as f32;
    let height = bitmap.rows() as f32;
    let offset = (bitmap.rows() - glyph.bitmap_top()) as f32;

    (figure, width, height, offset)
}