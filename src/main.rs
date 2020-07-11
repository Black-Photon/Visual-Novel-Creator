mod text;
mod renderer;

#[macro_use]
extern crate glium;
extern crate image;
extern crate freetype;

use glium::Surface;
use crate::renderer::{create_texture, Artifact};
use glium::texture::Texture2d;
use image::ImageFormat;

use freetype::Library;
use crate::text::{create_font, load_char};

fn main() {
    let (display, event_loop) = renderer::prepare_window();
    let render_program = renderer::prepare(&display);

    let shape = renderer::Shape {
        bl_anchor: [0.0, 0.0],
        tr_anchor: [1.0, 1.0],
        bl_pos: [0.0, 0.0],
        tr_pos: [0.0, 0.0]
    };
    let texture = create_texture(&display, "D:/Programming/Projects/VNC/images/steinsgate_okabe_monitor.jpg", ImageFormat::Jpeg);
    let cog = Artifact {
        shape: shape,
        image: texture,
        depth: 0.0
    };

    let bottom = renderer::Shape {
        bl_anchor: [0.0, 0.0],
        tr_anchor: [1.0, 0.0],
        bl_pos: [0.0, 0.0],
        tr_pos: [0.0, 300.0]
    };
    let texture_2 = create_texture(&display, "D:/Programming/Projects/VNC/images/textbox.png", ImageFormat::Png);
    let alt = Artifact {
        shape: bottom,
        image: texture_2,
        depth: 0.5
    };


    let lib = Library::init().unwrap();
    let face = create_font(lib, "C:/Windows/fonts/Arial.ttf", 1024);
    let (figure, width, height, offset) = load_char(face, 'A');

    let char_shape = renderer::Shape {
        bl_anchor: [0.5, 0.5],
        tr_anchor: [0.5, 0.5],
        bl_pos: [0.0, -offset],
        tr_pos: [width, height]
    };
    let image_dimensions = (width as u32, height as u32);
    let char_image = glium::texture::RawImage2d::from_raw_rgba(figure, image_dimensions);
    let char_tex = Texture2d::new(&display, char_image).unwrap();
    let char_art = Artifact {
        shape: char_shape,
        image: char_tex,
        depth: 0.8
    };

    let artifacts = vec![cog, alt, char_art];

    renderer::start_draw(&frame, event_loop, display, render_program, artifacts);
}

fn frame(display: &glium::Display, render_program: &renderer::Renderer, artifacts: &Vec<Artifact>, _delta_t: f32, _abs_t: f32) {
    let mut target = display.draw();
    target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
    for artifact in artifacts {
        target = renderer::draw_shape(render_program, target, artifact);
    }

    target.finish().unwrap();
}