#[macro_use]
extern crate glium;

use std::time::Instant;

use glium::{glutin, Surface};
use winit::event::{Event, StartCause};
use winit::event_loop::ControlFlow;

use helper::{CameraSystem, Colors, get_perspective, load_glsl, RawMat4, Transform};
use helper::glm::{cross, look_at, Mat4, normalize, vec3};
use rust_opengl::{draw_params, load_png_texture, set_fullscreen, Vertex};
use rust_opengl::binding::Binding;
use rust_opengl::geometry::cube::{cube_indexes, cube_vertexes};
use rust_opengl::input::{Gesture, Input};

pub type LoopType = glium::glutin::event_loop::EventLoop<()>;
pub type VertexBuffer = glium::VertexBuffer<Vertex>;
pub type IndexBuffer = glium::IndexBuffer<u16>;

const CAMERA_SPEED: f32 = 0.25;

fn main() {
    let draw_params = draw_params();
    let event_loop: LoopType = LoopType::new();
    let wb = glutin::window::WindowBuilder::new().with_title("3D Playground");
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    let mut input = Input::create();
    let binding = Binding::create();
    let mut fullscreen = false;
    let background_color = Colors::BLACK;
    let bricks_tex = load_png_texture("resources/textures/bricks.png", &display).unwrap();
    let rubiks_tex = load_png_texture("resources/textures/rubiks cube.png", &display).unwrap();
    let square = [
        Vertex::new(0.5, 0.5, 0.0, [1.0, 1.0]),
        Vertex::new(0.5, -0.5, 0.0, [1.0, 0.0]),
        Vertex::new(-0.5, -0.5, 0.0, [0.0, 0.0]),
        Vertex::new(-0.5, 0.5, 0.0, [0.0, 1.0])
    ];
    let sample_vertex_src = load_glsl("resources/shaders/sample_tex.vs.glsl");
    let sample_fragment_src = load_glsl("resources/shaders/sample_tex.fs.glsl");
    let sample_program =
        glium::Program::from_source(&display, &sample_vertex_src, &sample_fragment_src, None)
            .unwrap();
    let triangle_vertexes = VertexBuffer::new(&display, &square).unwrap();
    let triangle_indexes = IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &[0, 1, 2, 2, 3, 0]).unwrap();
    let triangle_transform = Transform::new();
    let cube_vertexes = VertexBuffer::new(&display, &cube_vertexes()).unwrap();
    let cube_indexes = IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &cube_indexes()).unwrap();
    let mut cube_transform = Transform::new();
    cube_transform.translate(-2., -1., -1.);

    let mut uniform_color = Colors::MAGENTA;
    let mut camera_system = CameraSystem::default();
    let mut camera: Mat4 = (&camera_system).into();
    let perspective = get_perspective(display.get_framebuffer_dimensions().0, display.get_framebuffer_dimensions().1);
    let vp = &perspective * &camera;
    let mut pre_vp: RawMat4 = vp.into();

    let before_run = Instant::now();
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
            let step = input.poll_analog2d(&binding.movement);
            if step.y != 0. {
                camera_system.pos += camera_system.front * step.y * CAMERA_SPEED;
            }
            if step.x != 0. {
                camera_system.pos += normalize(&cross(&camera_system.front, &camera_system.up)) * step.x * CAMERA_SPEED;
            }
            camera = (&camera_system).into();
            // rotate_camera_around_scene(&mut camera, &before_run);
            pre_vp = (&perspective * &camera).into();
            display.gl_window().window().request_redraw();
        }
        Event::RedrawRequested(_) => {
            let mut frame = display.draw();
            frame.clear_color_and_depth(background_color.into(), 1.);
            let color: [f32; 3] = uniform_color.into();
            let t: RawMat4 = (&triangle_transform).into();
            let uniforms = uniform! {
                uColor: color,
                tex: &bricks_tex,
                vp: pre_vp,
                transform: t
            };
            frame.draw(&triangle_vertexes, &triangle_indexes, &sample_program, &uniforms, &draw_params).unwrap();

            let t: RawMat4 = (&cube_transform).into();
            let uniforms = uniform! {
                uColor: color,
                tex: &rubiks_tex,
                vp: pre_vp,
                transform: t
            };
            frame.draw(&cube_vertexes, &cube_indexes, &sample_program, &uniforms, &draw_params).unwrap();
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

fn rotate_camera_around_scene(camera: &mut Mat4, run_start: &Instant) {
    let radius = 5.0;
    let delta = Instant::now().duration_since(run_start.clone()).as_secs_f32();
    let cam_x = delta.sin() * radius;
    let cam_z = delta.cos() * radius;
    *camera = look_at(&vec3(cam_x, 2.0, cam_z),
                      &vec3(0.0, 0.0, 0.0),
                      &vec3(0.0, 1.0, 0.0f32));
}