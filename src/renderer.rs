use glium::{glutin, Surface, Display, IndexBuffer, Frame};
use glium::backend::glutin::glutin::ContextCurrentState;
use glutin::event_loop::EventLoop;
use glium::texture::Texture2d;

use std::fs::{read_to_string, File};
use std::io::Read;
use image::ImageFormat;

#[derive(Copy, Clone)]
pub(crate) struct Vertex {
    position: [f32; 2],
}

/// Prepares a window that can be drawn on
///
/// # Return
/// * Display - An object representing the created window
/// * EventLoop<()> - An event loop associated with the window
pub(crate) fn prepare_window() -> (Display, EventLoop<()>) {
    // Setup event loop and widow params
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let (wb, cb) = setup_window(wb, cb);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    return (display, event_loop);
}

fn setup_window
    <T: ContextCurrentState>
    (mut wb: glutin::window::WindowBuilder, cb: glutin::ContextBuilder<T>) ->
        (glutin::window::WindowBuilder, glutin::ContextBuilder<T>) {
    wb = wb.with_title("My awesome visual novel!");
    wb = wb.with_maximized(true);
    return (wb, cb)
}

pub(crate) fn start_draw<F: 'static>(main_loop: F, event_loop: EventLoop<()>, display: glium::Display, renderer: Renderer, artifacts: Vec<Artifact>) -> !
where
    F: Fn(&glium::Display, &Renderer, &Vec<Artifact>, f32, f32) -> ()
{
    let start = std::time::Instant::now();
    let mut current = start;

    event_loop.run(move |ev, _, control_flow| {
        let now = std::time::Instant::now();
        let delta_t = now.duration_since(current);
        let abs_t = now.duration_since(start);
        current = now;

        main_loop(&display, &renderer, &artifacts, delta_t.as_secs_f32(), abs_t.as_secs_f32());

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

pub(crate) fn prepare(display: &glium::Display) -> Renderer {
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
    let indices: IndexBuffer<u32> = glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList,
                                          &indices).unwrap();

    let vertex_shader_src = read_to_string("shaders/vertex.vert ").unwrap();
    let fragment_shader_src = read_to_string("shaders/fragment.frag").unwrap();

    let program = glium::Program::from_source(display, vertex_shader_src.as_ref(), fragment_shader_src.as_ref(), None).unwrap();

    return Renderer{ vertex_buffer, indices, program };
}

pub(crate) fn draw_shape(renderer: &Renderer, mut target: Frame, artifact: &Artifact) -> Frame {
    let shape = artifact.shape;
    unsafe {
        artifact.image.generate_mipmaps(); // This binds the texture
    };
    let uniforms = uniform!{
        aspect_ratio: [1920.0, 1080.0f32],
        bl_anchor: shape.bl_anchor,
        tr_anchor: shape.tr_anchor,
        bl_pos: shape.bl_pos,
        tr_pos: shape.tr_pos,
        depth: artifact.depth,
        image: &artifact.image
    };

    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        blend: glium::draw_parameters::Blend::alpha_blending(),
        .. Default::default()
    };

    target.draw(&renderer.vertex_buffer, &renderer.indices, &renderer.program, &uniforms, &params).unwrap();
    return target
}

pub(crate) fn create_texture(display: &Display, location: &str, format: ImageFormat) -> Texture2d {
    use std::io::Cursor;
    let mut file = File::open(location).unwrap();
    let mut buffer = Vec::with_capacity(0);
    file.read_to_end(&mut buffer).unwrap();

    let image = image::load(Cursor::new(buffer),
                            format).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    return Texture2d::new(display, image).unwrap();
}

pub(crate) struct Renderer {
    vertex_buffer: glium::VertexBuffer<Vertex>,
    indices: IndexBuffer<u32>,
    program: glium::Program
}

#[derive(Copy, Clone)]
pub(crate) struct Shape {
    pub(crate) bl_anchor: [f32;2],
    pub(crate) tr_anchor: [f32;2],
    pub(crate) bl_pos: [f32;2],
    pub(crate) tr_pos: [f32;2]
}

pub(crate) struct Artifact {
    pub(crate) shape: Shape,
    pub(crate) image: Texture2d,
    pub(crate) depth: f32
}