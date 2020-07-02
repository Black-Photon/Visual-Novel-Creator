mod renderer;

#[macro_use]
extern crate glium;
extern crate image;

use glium::Surface;
use crate::renderer::{create_texture, Shape};
use glium::texture::{Texture2d, TextureAny};
use image::ImageFormat;

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

    let artifacts = vec![cog, alt];

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

pub(crate) struct Artifact {
    shape: Shape,
    image: Texture2d,
    depth: f32
}