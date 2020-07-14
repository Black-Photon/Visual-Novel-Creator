use freetype::{Face, Library};
use std::ffi::OsStr;
use crate::renderer::{Position, Renderer, Artifact};
use glium::{Frame, Display, Surface};
use glium::texture::Texture2d;

pub(crate) fn create_font<P>(library: Library, name: P, resolution: u32) -> Face
    where P: AsRef<OsStr>
{
    let face = library.new_face(name, 0).unwrap();
    face.set_char_size(40 * 64, 0, resolution, resolution).unwrap();
    return face;
}

pub(crate) fn load_char(display: &Display, face: &Face, c: char) -> Character {
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

    let mut width = bitmap.width();
    let height = bitmap.rows();
    let offset = (bitmap.rows() - glyph.bitmap_top()) as f32;
    if c == ' ' {
        width = 50;
    }

    let char_image = glium::texture::RawImage2d::from_raw_rgba(figure, (width as u32, height as u32));
    let char_tex = Texture2d::new(display, char_image).unwrap();

    Character {
        image: char_tex,
        width: width as f32,
        height: height as f32,
        offset
    }
}

pub(crate) fn create_string(string: &str, char_set: &'static Vec<Character>, position: Position, depth: f32, font_size: f32) -> TextArtifact {
    let mut string_vec: Vec<&'static Character> = Vec::new();
    for c in string.chars() {
        let character: &'static Character = char_set.get(c as usize).unwrap();
        string_vec.push(character);
    }
    TextArtifact {
        position,
        string: string_vec,
        depth,
        font_size
    }
}

pub(crate) struct Character {
    pub(crate) image: Texture2d,
    pub(crate) width: f32,
    pub(crate) height: f32,
    pub(crate) offset: f32
}

pub(crate) struct TextArtifact {
    pub(crate) position: Position,
    pub(crate) string: Vec<&'static Character>,
    pub(crate) depth: f32,
    pub(crate) font_size: f32
}

impl Artifact for TextArtifact {
    fn draw(&self, renderer: &Renderer, mut target: Frame) -> Frame {
        let pos = self.position;
        let mut origin = pos.position;

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            blend: glium::draw_parameters::Blend::alpha_blending(),
            ..Default::default()
        };

        for c in self.string.to_vec() {
            let width = c.width * self.font_size * 0.01;
            let height = c.height * self.font_size * 0.01;
            let offset = c.offset * self.font_size * 0.01;

            let bl_pos = [*origin.get(0).unwrap(), origin.get(1).unwrap() - offset];
            let tr_pos = [origin.get(0).unwrap() + width, origin.get(1).unwrap() + height - offset];
            unsafe {
                c.image.generate_mipmaps(); // This binds the texture
            };
            let uniforms = uniform! {
                aspect_ratio: [1920.0, 1080.0f32],
                bl_anchor: pos.anchor,
                tr_anchor: pos.anchor,
                bl_pos: bl_pos,
                tr_pos: tr_pos,
                depth: self.depth,
                image: &c.image
            };

            target.draw(&renderer.vertex_buffer, &renderer.indices, &renderer.program, &uniforms, &params).unwrap();
            origin = [origin.get(0).unwrap() + width + 5.0, *origin.get(1).unwrap()];
        }
        return target
    }
}