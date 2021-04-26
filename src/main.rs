use std::f32::consts::FRAC_PI_2;
use std::time::Instant;

use graphics::{Colors, draw_params, glium, GVec3, Light, load_glsl, load_png_texture, Material, Vertex};
use graphics::glium::glutin::dpi::{PhysicalPosition, PhysicalSize, Size};
use graphics::glium::glutin::event::{Event, StartCause};
use graphics::glium::glutin::event_loop::ControlFlow;
use graphics::glium::glutin::GlProfile;
use graphics::glium::glutin::window::WindowBuilder;
use graphics::glium::Surface;
use graphics::glium::uniform;
use math::{CameraSystem, Perspective, RawMat4, TransformBuilder};
use math::glm::{cross, look_at, Mat4, normalize, vec3};
use rust_opengl::{set_fullscreen, TICK_DRAW_ID, TICK_FRAME_ID, TICK_RENDER_ID, TickSystem};
use rust_opengl::geometry::cube::{cube_indexes, cube_vertexes};
use ui::{Binding, Gesture, Input};

pub type LoopType = glium::glutin::event_loop::EventLoop<()>;
pub type VertexBuffer = glium::VertexBuffer<Vertex>;
pub type IndexBuffer = glium::IndexBuffer<u16>;

const CAMERA_SPEED: f32 = 0.25;
const PITCH_MAX: f32 = 1.55334f32;
const WIDTH: f32 = 1024f32;
const HEIGHT: f32 = 768f32;
const FOV_MIN: f32 = 0.0174533f32;
const FOV_MAX: f32 = 0.785398f32;

// compared € [to_compare - epsilon; to_compare + epsilon]
#[inline]
fn float_eq(value: f32, compared: f32, epsilon: f32) -> bool {
    (value - compared).abs() < epsilon
}

#[inline]
fn to_radians(degree: f32) -> f32 {
    degree.to_radians()
}

fn main() {
    let z_axis = vec3(0.0, 0.0, 1.0f32);
    let y_axis = vec3(0.0, 1.0, 0.0f32);
    let x_axis = vec3(1.0, 0.0, 0.0f32);
    let custom_axis = vec3(1.0, 0.3, 0.5f32);
    let draw_params = draw_params();
    let mut tick_system = TickSystem::new();
    tick_system.register_listener(TICK_FRAME_ID);
    tick_system.register_listener(TICK_DRAW_ID);
    tick_system.register_listener(TICK_RENDER_ID);
    let event_loop: LoopType = LoopType::new();
    let wb = WindowBuilder::new()
        .with_title("3D Playground")
        .with_inner_size(Size::Physical(PhysicalSize::new(WIDTH as u32, HEIGHT as u32)));
    let cb = glium::glutin::ContextBuilder::new().with_gl_profile(GlProfile::Core);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    let mut input = Input::create();
    let binding = Binding::create();
    let mut fullscreen = false;
    let background_color = Colors::BLACK;
    let bricks_tex = load_png_texture("resources/textures/bricks.png", &display).unwrap();
    let rubiks_tex = load_png_texture("resources/textures/rubiks cube.png", &display).unwrap();
    let gold = Material::new(GVec3::new(0.24725, 0.1995, 0.0745), GVec3::new(0.75164, 0.60648, 0.22648), GVec3::new(0.628281, 0.555802, 0.366065), 0.4);
    let ruby = Material::new(GVec3::new(0.1745, 0.01175, 0.01175), GVec3::new(0.61424, 0.04136, 0.04136), GVec3::new(0.727811, 0.626959, 0.626959), 0.6);
    let square = [
        Vertex::new(0.0, 0.0, 0.0, [0.0, 0.0, 1.0], [1.0, 1.0]),
        Vertex::new(1.0, 0.0, 0.0, [0.0, 0.0, 1.0], [1.0, 0.0]),
        Vertex::new(0.0, 1.0, 0.0, [0.0, 0.0, 1.0], [0.0, 0.0]),
        Vertex::new(1.0, 1.0, 0.0, [0.0, 0.0, 1.0], [0.0, 1.0])
    ];

    let square_vertexes = VertexBuffer::new(&display, &square).unwrap();
    let square_indexes = IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &[0, 1, 3, 3, 2, 0]).unwrap();
    let mut gold_buffer = glium::uniforms::UniformBuffer::new(&display, gold).unwrap();
    let ruby_buffer = glium::uniforms::UniformBuffer::new(&display, ruby).unwrap();
    let floor_model = TransformBuilder::new().scale(10., 10., 10.).build();
    let sample_vertex_src = load_glsl("resources/shaders/material_lighting.vs.glsl");
    let sample_fragment_src = load_glsl("resources/shaders/material_lighting.fs.glsl");
    let lighting_vertex_src = load_glsl("resources/shaders/lighting.vs.glsl");
    let lighting_fragment_src = load_glsl("resources/shaders/lighting.fs.glsl");
    let lighting_program =
        glium::Program::from_source(&display, &lighting_vertex_src, &lighting_fragment_src, None)
            .unwrap();
    let sample_program =
        glium::Program::from_source(&display, &sample_vertex_src, &sample_fragment_src, None)
            .unwrap();
    let cube_vertexes = VertexBuffer::new(&display, &cube_vertexes()).unwrap();
    let cube_indexes = IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &cube_indexes()).unwrap();
    let cube_models = [
        TransformBuilder::new().translate(0.0, 0.0, 1.0).build(),
        TransformBuilder::new().translate(2.0, 5.0, -15.0).rotate(to_radians(82.0), &y_axis).build(),
        TransformBuilder::new().translate(-1.5, -2.2, -2.5).rotate(to_radians(55.0), &x_axis).build(),
        TransformBuilder::new().translate(-3.8, -2.0, -12.3).rotate(to_radians(112.), &z_axis).build(),
        TransformBuilder::new().translate(2.4, -0.4, -3.5).rotate(to_radians(71.), &y_axis).build(),
        TransformBuilder::new().translate(-1.7, 3.0, -7.5).rotate(to_radians(170.), &custom_axis).build(),
        TransformBuilder::new().translate(1.3, -2.0, -2.5).build(),
        TransformBuilder::new().translate(1.5, 2.0, -2.5).rotate(to_radians(131.), &x_axis).build(),
        TransformBuilder::new().translate(1.5, 0.2, -1.5).rotate(to_radians(310.), &z_axis).build(),
        TransformBuilder::new().translate(-1.3, 1.0, -1.5).build(),
    ];
    let mut uniform_color = Colors::MAGENTA;
    let mut camera = CameraSystem::default();
    let (mut w, mut h) = (display.get_framebuffer_dimensions().0, display.get_framebuffer_dimensions().1);
    let mut perspective = Perspective::default();
    let vp = perspective.get() * &camera.view();
    let mut pre_vp: RawMat4 = vp.into();
    let light_color = GVec3::new(0.33, 0.42, 0.18f32);
    let light_color = GVec3::new(1.0, 1.0, 1.0f32);
    let object_color = GVec3::new(1.0, 0.5, 0.31f32);
    let mut light_position = GVec3::new(1.2, 2.0, 2.0f32);
    let mut light_bulb = TransformBuilder::new().translate(light_position.data.x, light_position.data.y, light_position.data.z).scale(0.2, 0.2, 0.2).build();
    let mut light = Light::new(
        GVec3::new(1.2, 2.0, 2.0f32),
        GVec3::new(0.2, 0.2, 0.2),
        GVec3::new(0.5, 0.5, 0.5),
        GVec3::new(1.0, 1.0, 1.0));
    // let mut light_bulb = TransformBuilder::new().translate(light.position.0, light.position.1, light.position.2).scale(0.2, 0.2, 0.2).build();
    let (mut yaw, mut pitch) = (FRAC_PI_2 * 2., 0.0);

    event_loop.run(move |event, _, control_flow| match event {
        Event::NewEvents(cause) => match cause {
            StartCause::ResumeTimeReached { .. } => {}
            StartCause::WaitCancelled { .. } => {}
            StartCause::Poll => {}
            StartCause::Init => {
                tick_system.start_tick(TICK_FRAME_ID);
            }
        },
        Event::MainEventsCleared => {
            if input.poll_gesture(&binding.toggle_mouse) {
                let window_context = display.gl_window();
                let window = window_context.window();
                // let win_pos = window.set_cursor_visible(false);
                let mouse = &input.poll_analog2d(&binding.look);

                w = display.get_framebuffer_dimensions().0;
                h = display.get_framebuffer_dimensions().1;
                window.set_cursor_position(PhysicalPosition::new(w / 2, h / 2)).unwrap();
                yaw += mouse.x;
                pitch += mouse.y;
                if pitch > PITCH_MAX {
                    pitch = PITCH_MAX;
                }
                if pitch < -PITCH_MAX {
                    pitch = -PITCH_MAX;
                }
                let direction = vec3(
                    yaw.cos() * pitch.cos(),
                    pitch.sin(),
                    yaw.sin() * pitch.cos(),
                );
                camera.front = direction.normalize();
            }
            let step = input.poll_analog2d(&binding.scroll);
            if !float_eq(step.y, 0.0, 1e-3) {
                perspective.fov -= step.y;
                if perspective.fov < FOV_MIN {
                    perspective.fov = FOV_MIN;
                } else if perspective.fov > FOV_MAX {
                    perspective.fov = FOV_MAX;
                }
            }

            if input.poll_gesture(&binding.swap_color) {
                uniform_color = Colors::random();
            }
            let step = input.poll_analog2d(&binding.movement);
            if step.y != 0. {
                camera.pos += camera.front * step.y * CAMERA_SPEED;
            }
            if step.x != 0. {
                camera.pos += normalize(&cross(&camera.front, &camera.up)) * step.x * CAMERA_SPEED;
            }
            // rotate_camera_around_scene(&mut camera, &before_run);
            if let Some(duration) = tick_system.duration_since_frame_start() {
                // rotate_light_around_scene(&mut light_position, duration as f32);
                // light_bulb.move_to(light_position.data.x, light_position.data.y, light_position.data.z);
                rotate_light_around_scene(&mut light.position, duration as f32);
                light_bulb.move_to(light.position.data.x, light.position.data.y, light.position.data.z);
            }
            pre_vp = (perspective.get() * camera.view()).into();
            display.gl_window().window().request_redraw();
        }
        Event::RedrawRequested(_) => {
            tick_system.start_tick(TICK_RENDER_ID);
            let mut frame = display.draw();
            frame.clear_color_and_depth(background_color.into(), 1.);
            let color: [f32; 3] = uniform_color.into();

            let model = light_bulb.get_raw();
            let uniforms = uniform! {
                vp: pre_vp,
                model: model
            };
            frame.draw(&cube_vertexes, &cube_indexes, &lighting_program, &uniforms, &draw_params).unwrap();
            let view_pos: [f32; 3] = camera.pos.into();
            let view: RawMat4 = camera.view().into();
            let model = cube_models[0].get_raw();
            let uniforms = uniform! {
                lightColor: light_color,
                objectColor: object_color,
                lightPos: light_position,
                vp: pre_vp,
                view: view,
                model: model,
                viewPos: view_pos,
            };
            let uniforms = uniforms.add("material.ambient", gold.ambient);
            let uniforms = uniforms.add("material.diffuse", gold.diffuse);
            let uniforms = uniforms.add("material.specular", gold.specular);
            let uniforms = uniforms.add("material.shininess", gold.shininess);
            let uniforms = uniforms.add("light.position", light.position);
            let uniforms = uniforms.add("light.ambient", light.ambient);
            let uniforms = uniforms.add("light.diffuse", light.diffuse);
            let uniforms = uniforms.add("light.specular", light.specular);
            frame.draw(&cube_vertexes, &cube_indexes, &sample_program, &uniforms, &draw_params).unwrap();

            let object_color: [f32; 3] = Colors::RED.into();
            let model = floor_model.get_raw();
            let view_pos: [f32; 3] = camera.pos.into();
            let view: RawMat4 = camera.view().into();
            let uniforms = uniform! {
                lightColor: light_color,
                objectColor: object_color,
                lightPos: light_position,
                vp: pre_vp,
                view: view,
                model: model,
                viewPos: view_pos,
                // material: &ruby_buffer,
                // light: &light_buffer
            };
            let uniforms = uniforms.add("material.ambient", ruby.ambient);
            let uniforms = uniforms.add("material.diffuse", ruby.diffuse);
            let uniforms = uniforms.add("material.specular", ruby.specular);
            let uniforms = uniforms.add("material.shininess", ruby.shininess);
            let uniforms = uniforms.add("light.position", light.position);
            let uniforms = uniforms.add("light.ambient", light.ambient);
            let uniforms = uniforms.add("light.diffuse", light.diffuse);
            let uniforms = uniforms.add("light.specular", light.specular);
            frame.draw(&square_vertexes, &square_indexes, &sample_program, &uniforms, &draw_params).unwrap();

            frame.finish().unwrap();
            tick_system.end_tick(TICK_RENDER_ID);
            // tick_system.debug_tick(TICK_RENDER_ID);
        }
        Event::RedrawEventsCleared => {
            if input.poll_gesture(&binding.exit) || input.poll_gesture(&Gesture::QuitTrigger) {
                *control_flow = ControlFlow::Exit;
            }
            if input.poll_gesture(&binding.fullscreen) {
                set_fullscreen(&display, &mut fullscreen);
            }
            input.tick_reset();
            tick_system.end_tick(TICK_FRAME_ID);
            // tick_system.debug_tick(TICK_FRAME_ID);
            tick_system.update_time();
            if tick_system.should_reset() {
                tick_system.debug_tick_iteration();
                tick_system.reset();
            }
            // println!();
            tick_system.start_tick(TICK_FRAME_ID);
        }
        _ => input.update(&event),
    });
}

fn _rotate_camera_around_scene(camera: &mut Mat4, run_start: &Instant) {
    let radius = 5.0;
    let delta = Instant::now().duration_since(run_start.clone()).as_secs_f32();
    let cam_x = delta.sin() * radius;
    let cam_z = delta.cos() * radius;
    *camera = look_at(&vec3(cam_x, 2.0, cam_z),
                      &vec3(0.0, 0.0, 0.0),
                      &vec3(0.0, 1.0, 0.0f32));
}

fn rotate_light_around_scene(light_pos: &mut GVec3, delta: f32) {
    *light_pos.data = *math::glm::rotate_vec3(&mut light_pos.data, delta, &vec3(0.0, 0.0, 1.0));
}

fn _rotate_light_around_scene_raw(light_pos: &mut (f32, f32, f32), delta: f32) {
    let tmp = vec3(light_pos.0, light_pos.1, light_pos.2);
    let tmp = math::glm::rotate_vec3(&tmp, delta, &vec3(0.0, 0.0, 1.0));
    *light_pos = (tmp.x, tmp.y, tmp.z);
}

