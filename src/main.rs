mod renderer;

#[macro_use]
extern crate glium;

fn main() {
    let (display, event_loop) = renderer::prepare_window();
    let render_program = renderer::prepare(&display);
    renderer::start_draw(&frame, event_loop, display, render_program);
}

fn frame(display: &glium::Display, render_program: &renderer::Renderer, _delta_t: f32, _abs_t: f32) {
    let shape = renderer::Shape {
        position: [0.0, 0.6],
        size: [0.2, 0.1]
    };
    renderer::draw_shape(display, render_program, shape);
}