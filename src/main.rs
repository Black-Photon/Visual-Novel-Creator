#[macro_use]
extern crate glium;
use glium::{glutin, Surface};
use glium::backend::glutin::glutin::ContextCurrentState;
use glium::index::IndicesSource;

use std::fs::read_to_string;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

fn main() {
    // Setup event loop and widow params
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let (wb, cb) = setup_window(wb, cb);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    let (vertex_buffer, indices, program) = prepare(&display);

    let start = std::time::Instant::now();
    let mut current = start;

    event_loop.run(move |ev, _, control_flow| {
        let now = std::time::Instant::now();
        let delta_t = now.duration_since(current);
        let abs_t = now.duration_since(start);
        current = now;

        main_loop(&display, &vertex_buffer, IndicesSource::from(&indices), &program, delta_t.as_secs_f32(), abs_t.as_secs_f32());

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);


        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            _ => (),
        }
    });
}

fn setup_window
    <T: ContextCurrentState>
    (mut wb: glutin::window::WindowBuilder, cb: glutin::ContextBuilder<T>) ->
        (glutin::window::WindowBuilder, glutin::ContextBuilder<T>) {
    wb = wb.with_title("My awesome visual novel!");
    wb = wb.with_maximized(true);
    return (wb, cb)
}

fn prepare(display: &glium::Display) -> (glium::VertexBuffer<Vertex>, glium::IndexBuffer<u16>, glium::Program) {
    implement_vertex!(Vertex, position);

    let shape = vec![
        Vertex { position: [1.0, 1.0] },
        Vertex { position: [1.0, -1.0] },
        Vertex { position: [-1.0, -1.0] },
        Vertex { position: [-1.0, 1.0] },
    ];
    let indices = vec![
        0, 1, 2,
        0, 2, 3
    ];

    let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();
    let indices = glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList,
                                          &indices).unwrap();

    let vertex_shader_src = read_to_string("shaders/vertex.vert ").unwrap();
    let fragment_shader_src = read_to_string("shaders/fragment.frag").unwrap();

    let program = glium::Program::from_source(display, vertex_shader_src.as_ref(), fragment_shader_src.as_ref(), None).unwrap();

    return (vertex_buffer, indices, program);
}

fn main_loop(display: &glium::Display, vertex_buffer: &glium::VertexBuffer<Vertex>, indices: IndicesSource, program: &glium::Program, delta_t: f32, abs_t: f32) {
    let mut target = display.draw();
    target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

    let uniforms = uniform!{ };

    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        .. Default::default()
    };

    target.draw(vertex_buffer, indices, program, &uniforms, &params).unwrap();
    target.finish().unwrap();
}