use glium::{Display, glutin, Surface};
use winit::event::{Event, StartCause};
use winit::event_loop::ControlFlow;
use winit::window::Fullscreen;

use helper::{Colors, load_glsl};
use rust_opengl::binding::Binding;
use rust_opengl::input::{Gesture, Input};
use rust_opengl::Vertex;

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
    let mut background_color = Colors::BLACK;
    let triangle = [
        Vertex::new(-0.5, -0.5, 0.0),
        Vertex::new(0.5, -0.5, 0.0),
        Vertex::new(0.0, 0.5, 0.0)
    ];
    let triangle_vertex_src = load_glsl("resources/shaders/sample.vs.glsl");
    let triangle_fragment_src = load_glsl("resources/shaders/sample.fs.glsl");
    let triangle_program =
        glium::Program::from_source(&display, &triangle_vertex_src, &triangle_fragment_src, None)
            .unwrap();
    let triangle_vertexes = VertexBuffer::new(&display, &triangle).unwrap();
    let triangle_indexes = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    event_loop.run(move |event, _, control_flow| match event {
        Event::NewEvents(cause) => match cause {
            StartCause::ResumeTimeReached { .. } => {}
            StartCause::WaitCancelled { .. } => {}
            StartCause::Poll => {}
            StartCause::Init => {}
        },
        Event::MainEventsCleared => {
            if input.poll_gesture(&binding.swap_color) {
                background_color = Colors::random();
            }
            display.gl_window().window().request_redraw();
        }
        Event::RedrawRequested(_) => {
            let mut frame = display.draw();
            frame.clear_color_and_depth(background_color.into(), 1.);
            frame.draw(&triangle_vertexes, &triangle_indexes, &triangle_program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
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