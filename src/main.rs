mod text;
mod renderer;

#[macro_use]
extern crate glium;
extern crate image;
extern crate freetype;

use glium::Surface;
use crate::renderer::{create_texture, SpriteArtifact, Artifact, Position, EmptyArtifact};
use image::ImageFormat;

use freetype::Library;
use crate::text::{create_font, load_char, create_string, Character};

fn main() {
    let (display, event_loop) = renderer::prepare_window();
    let render_program = renderer::prepare(&display);
    let lib = Library::init().unwrap();
    let font = create_font(lib, "C:/Windows/fonts/Arial.ttf", 1024);
    static mut CHARS: Vec<Character> = Vec::new();
    unsafe {
        for c in 0..128 as u8 {
            CHARS.push(load_char(&display, &font, c as char));
        }
    }

    let background_shape = renderer::Shape {
        bl_anchor: [0.0, 0.0],
        tr_anchor: [1.0, 1.0],
        bl_pos: [0.0, 0.0],
        tr_pos: [0.0, 0.0]
    };
    let background_tex = create_texture(&display, "D:/Programming/Projects/VNC/images/steinsgate_okabe_monitor.jpg", ImageFormat::Jpeg);
    let background = SpriteArtifact {
        shape: background_shape,
        image: background_tex,
        depth: 0.0,
        name: "background".to_string(),
        children: vec![]
    };

    let textbox_shape = renderer::Shape {
        bl_anchor: [0.0, 0.0],
        tr_anchor: [1.0, 0.0],
        bl_pos: [0.0, 0.0],
        tr_pos: [0.0, 300.0]
    };
    let textbox_tex = create_texture(&display, "D:/Programming/Projects/VNC/images/textbox.png", ImageFormat::Png);
    let textbox = SpriteArtifact {
        shape: textbox_shape,
        image: textbox_tex,
        depth: 0.5,
        name: "textbox".to_string(),
        children: vec![]
    };

    let mut root = EmptyArtifact {
        name: "root".to_string(),
        children: vec![Box::new(background), Box::new(textbox)]
    };

    unsafe {
        let text = create_string("No! You can't go now! That would surely spell the end for all of us!", &CHARS, Position::new([0.0, 0.0], [40.0, 200.0]), 0.8, 6.0, "text", vec![]);
        root.add_child(Box::new(text));
    }
    let artifacts: Vec<Box<dyn Artifact>> = vec![Box::new(root)];
    renderer::start_draw(&frame, event_loop, display, render_program, artifacts);
}

fn frame(display: &glium::Display, render_program: &renderer::Renderer, artifacts: &Vec<Box<dyn Artifact>>, _delta_t: f32, _abs_t: f32) {
    let mut target = display.draw();
    target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
    for artifact in artifacts {
        target = artifact.draw(render_program, target);
    }

    target.finish().unwrap();
}