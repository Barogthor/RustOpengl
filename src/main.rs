#[macro_use]
extern crate glium;

use glium::{Display, glutin, Surface};
use winit::event::{Event, StartCause};
use winit::event_loop::ControlFlow;
use winit::window::Fullscreen;

use helper::{Colors, get_camera, get_perspective, load_glsl};
use rust_opengl::{load_png_texture, Vertex};
use rust_opengl::binding::Binding;
use rust_opengl::input::{Gesture, Input};

pub type LoopType = glium::glutin::event_loop::EventLoop<()>;
pub type VertexBuffer = glium::VertexBuffer<Vertex>;
pub type IndexBuffer = glium::IndexBuffer<u16>;

fn main() {
    let event_loop: LoopType = LoopType::new();
    let wb = glutin::window::WindowBuilder::new().with_title("3D Playground");
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    let mut input = Input::create();
    let binding = Binding::create();
    let mut fullscreen = false;
    let background_color = Colors::BLACK;
    let bricks_tex = load_png_texture("resources/textures/bricks.png", &display).unwrap();
    let square = [
        Vertex::new(0.5, 0.5, 0.0, [1.0, 1.0]),
        Vertex::new(0.5, -0.5, 0.0, [1.0, 0.0]),
        Vertex::new(-0.5, -0.5, 0.0, [0.0, 0.0]),
        Vertex::new(-0.5, 0.5, 0.0, [0.0, 1.0])
    ];
    let indexes = [0, 1, 2, 2, 3, 0];
    let sample_vertex_src = load_glsl("resources/shaders/sample_tex.vs.glsl");
    let sample_fragment_src = load_glsl("resources/shaders/sample_tex.fs.glsl");
    let sample_program =
        glium::Program::from_source(&display, &sample_vertex_src, &sample_fragment_src, None)
            .unwrap();
    let triangle_vertexes = VertexBuffer::new(&display, &square).unwrap();
    let triangle_indexes = IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &indexes).unwrap();
    let mut uniform_color = Colors::MAGENTA;
    let vp = get_perspective(display.get_framebuffer_dimensions().0, display.get_framebuffer_dimensions().1) * get_camera();
    let pre_vp: [[f32; 4]; 4] = vp.into();

    event_loop.run(move |event, _, control_flow| match event {
        Event::NewEvents(cause) => match cause {
            StartCause::ResumeTimeReached { .. } => {}
            StartCause::WaitCancelled { .. } => {}
            StartCause::Poll => {}
            StartCause::Init => {}
        },
        Event::MainEventsCleared => {
            if input.poll_gesture(&binding.swap_color) {
                uniform_color = Colors::random();
            }
            display.gl_window().window().request_redraw();
        }
        Event::RedrawRequested(_) => {
            let mut frame = display.draw();
            frame.clear_color_and_depth(background_color.into(), 1.);
            let color: [f32; 3] = uniform_color.into();
            let uniforms = uniform! {
                uColor: color,
                tex: &bricks_tex,
                vp: pre_vp
            };
            frame.draw(&triangle_vertexes, &triangle_indexes, &sample_program, &uniforms, &Default::default()).unwrap();
            frame.finish().unwrap()
        }
        Event::RedrawEventsCleared => {
            if input.poll_gesture(&binding.exit) || input.poll_gesture(&Gesture::QuitTrigger) {
                *control_flow = ControlFlow::Exit;
            }
            if input.poll_gesture(&binding.fullscreen) {
                set_fullscreen(&display, &mut fullscreen);
            }
            input.increment_index();
        }
        _ => input.update(&display, &event),
    });
}

fn set_fullscreen(display: &Display, fullscreen: &mut bool) {
    if *fullscreen {
        display.gl_window().window().set_fullscreen(None);
        *fullscreen = false;
    } else {
        let monitor_handle = display
            .gl_window()
            .window()
            .available_monitors()
            .next()
            .unwrap();
        let fs = Fullscreen::Borderless(Some(monitor_handle));
        display.gl_window().window().set_fullscreen(Some(fs));

        *fullscreen = true;
    }
}