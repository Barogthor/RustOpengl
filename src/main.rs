use std::f32::consts::{FRAC_PI_2, PI};
use std::time::Instant;

use debug_ui::EguiGlium;
use graphics::{Colors, DirectionalLight, Draw, draw_params, glium, GVec3, load_glsl, load_jpeg_texture, load_png_texture, load_tif_texture, Material, PointLight, SpotLight, Vertex};
use graphics::glium::glutin::dpi::{PhysicalPosition, PhysicalSize, Size};
use graphics::glium::glutin::event::{Event, StartCause};
use graphics::glium::glutin::event_loop::ControlFlow;
use graphics::glium::glutin::GlProfile;
use graphics::glium::glutin::window::WindowBuilder;
use graphics::glium::Surface;
use graphics::glium::uniform;
use graphics::glium::uniforms::AsUniformValue;
use graphics::model::load_model_gltf;
use graphics::uniform::{StructToUniform, UniformStorage};
use math::{CameraSystem, Perspective, RawMat4, TransformBuilder};
use math::glm::{cross, look_at, Mat4, normalize, vec3};
use rust_opengl::{show_window, State};
use rust_opengl::geometry::cube::{cube_indexes, cube_vertexes_2d};
use rust_opengl::set_fullscreen;
use rust_opengl::tick::{TICK_DRAW_ID, TICK_FRAME_ID, TICK_RENDER_EGUI_ID, TICK_RENDER_ID, TickSystem};
use ui::{Binding, Gesture, Input};

pub type LoopType = glium::glutin::event_loop::EventLoop<()>;
pub type VertexBuffer = glium::VertexBuffer<Vertex>;
pub type IndexBuffer = glium::IndexBuffer<u16>;

const CAMERA_SPEED: f32 = 10.;
const PITCH_MAX: f32 = 1.55334f32;
const WIDTH: f32 = 1366f32;
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
    let backpack_model = load_model_gltf("resources/models/survival_guitar_backpack_low_poly/scene.gltf", &display);
    let mut egui = EguiGlium::new(&display);
    let mut input = Input::create();
    let binding = Binding::create();
    let mut fullscreen = false;
    let mut toggle_torchlight = true;
    let background_color = Colors::BLACK;
    let bricks_tex = load_png_texture("resources/textures/bricks.png", &display).unwrap();
    let rock_soil_albedo = load_tif_texture("resources/textures/TexturesCom_Rock_Soil_512_albedo.tif", &display).unwrap();
    let rock_soil_rough = load_tif_texture("resources/textures/TexturesCom_Rock_Soil_512_roughness.tif", &display).unwrap();
    // let backpack_albedo = load_jpeg_texture("resources/models/survival_guitar_backpack_low_poly/textures/Scene_-_Root_baseColor.jpeg", &display).unwrap();
    let backpack_albedo = load_jpeg_texture("resources/textures/1001_albedo.jpg", &display).unwrap();
    // let backpack_rough = load_png_texture("resources/models/survival_guitar_backpack_low_poly/textures/Scene_-_Root_metallicRoughness.png", &display).unwrap();
    let backpack_rough = load_jpeg_texture("resources/textures/1001_roughness.jpg", &display).unwrap();
    // let backpack_normal = load_png_texture("resources/models/survival_guitar_backpack_low_poly/textures/Scene_-_Root_normal.png", &display).unwrap();
    let backpack_normal = load_png_texture("resources/textures/1001_normal.png", &display).unwrap();
    let rubiks_tex = load_png_texture("resources/textures/rubiks cube.png", &display).unwrap();
    let container_diffuse = load_png_texture("resources/textures/container2.png", &display).unwrap();
    let container_specular = load_png_texture("resources/textures/container2_specular.png", &display).unwrap();
    let crate_mat = Material::new(container_diffuse, container_specular, 0.6);
    let rock_soil_mat = Material::new(rock_soil_albedo, rock_soil_rough, 1.);
    let backpack_mat = Material::new(backpack_albedo, backpack_rough, 1.);
    // let ruby = Material::new(GVec3::new(0.1745, 0.01175, 0.01175), GVec3::new(0.61424, 0.04136, 0.04136), GVec3::new(0.727811, 0.626959, 0.626959), 0.6);
    let square = [
        Vertex::new(0.0, 0.0, 0.0, [0.0, 0.0, 1.0], [1.0, 0.0]),
        Vertex::new(1.0, 0.0, 0.0, [0.0, 0.0, 1.0], [1.0, 1.0]),
        Vertex::new(0.0, 1.0, 0.0, [0.0, 0.0, 1.0], [0.0, 0.0]),
        Vertex::new(1.0, 1.0, 0.0, [0.0, 0.0, 1.0], [0.0, 1.0])
    ];
    let square_vertexes = VertexBuffer::new(&display, &square).unwrap();
    let square_indexes = IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &[0, 1, 3, 3, 2, 0]).unwrap();

    let floor_model = TransformBuilder::new()
        .scale(100., 100., 100.)
        .rotate(-PI / 2., &x_axis)
        .translate(-0.2, -0.4, -0.1)
        .build();
    let sample_vertex_src = load_glsl("resources/shaders/material_lightcaster.vs.glsl");
    let sample_fragment_src = load_glsl("resources/shaders/material_lightcaster_all.fs.glsl");
    let model_vertex_src = load_glsl("resources/shaders/model.vs.glsl");
    let model_fragment_src = load_glsl("resources/shaders/model.fs.glsl");
    let lighting_vertex_src = load_glsl("resources/shaders/lighting.vs.glsl");
    let lighting_fragment_src = load_glsl("resources/shaders/lighting.fs.glsl");
    let lighting_program =
        glium::Program::from_source(&display, &lighting_vertex_src, &lighting_fragment_src, None)
            .unwrap();
    let sample_program =
        glium::Program::from_source(&display, &sample_vertex_src, &sample_fragment_src, None)
            .unwrap();
    let model_program =
        glium::Program::from_source(&display, &model_vertex_src, &model_fragment_src, None)
            .unwrap();
    let cube_vertexes = VertexBuffer::new(&display, &cube_vertexes_2d()).unwrap();
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
    let backpack_transform = TransformBuilder::new().translate(-4.7, 1.0, -8.5).scale(0.01, 0.01, 0.01).build();
    let mut uniform_color = Colors::MAGENTA;
    let mut camera = CameraSystem::default();
    let (mut w, mut h) = (display.get_framebuffer_dimensions().0, display.get_framebuffer_dimensions().1);
    let mut perspective = Perspective::default();
    let vp = perspective.get() * &camera.view();
    let mut pre_vp: RawMat4 = vp.into();
    let mut dir_light = DirectionalLight::new(
        GVec3::new(1.2, 2.0, 2.0),
        GVec3::new(0.05, 0.05, 0.1),
        GVec3::new(0.2, 0.2, 0.7),
        GVec3::new(0.7, 0.7, 0.7));
    let mut light_points = [
        PointLight::new(
            GVec3::new(1.2, 2.0, 2.0f32),
            GVec3::new(0.1, 0.1, 0.1),
            GVec3::new(0.5, 0.5, 0.5),
            GVec3::new(1.0, 1.0, 1.0),
            1.0, 0.09, 0.0032),
        PointLight::new(
            GVec3::new(2.3, -3.3, -4.0),
            GVec3::new(0.1, 0.1, 0.1),
            GVec3::new(0.5, 0.5, 0.5),
            GVec3::new(1.0, 1.0, 1.0),
            1.0, 0.09, 0.0032),
        PointLight::new(
            GVec3::new(-4.0, 2.0, -12.0),
            GVec3::new(0.1, 0.1, 0.1),
            GVec3::new(0.5, 0.5, 0.5),
            GVec3::new(1.0, 1.0, 1.0),
            1.0, 0.09, 0.0032),
        PointLight::new(
            GVec3::new(0.0, 0.0, -3.0),
            GVec3::new(0.1, 0.1, 0.1),
            GVec3::new(0.5, 0.5, 0.5),
            GVec3::new(1.0, 1.0, 1.0),
            1.0, 0.09, 0.0032),
    ];
    let mut light_bulbs = [
        TransformBuilder::new().translate(light_points[0].position.data.x, light_points[0].position.data.y, light_points[0].position.data.z).scale(0.2, 0.2, 0.2).build(),
        TransformBuilder::new().translate(light_points[1].position.data.x, light_points[1].position.data.y, light_points[1].position.data.z).scale(0.2, 0.2, 0.2).build(),
        TransformBuilder::new().translate(light_points[2].position.data.x, light_points[2].position.data.y, light_points[2].position.data.z).scale(0.2, 0.2, 0.2).build(),
        TransformBuilder::new().translate(light_points[3].position.data.x, light_points[3].position.data.y, light_points[3].position.data.z).scale(0.2, 0.2, 0.2).build(),
    ];
    let mut light_spot = SpotLight::new(
        GVec3::new(4.0, 4.0, 2.0),
        {
            let mut dir = GVec3::new(-4.0, -4.0, -2.0);
            dir.data = dir.data.normalize();
            dir
        },
        GVec3::new(0.05, 0.05, 0.05),
        GVec3::new(0.5, 0.5, 0.5),
        GVec3::new(1.0, 1.0, 1.0),
        1.0, 0.045, 0.0075,
        to_radians(12.5).cos(), to_radians(17.5).cos(),
    );
    // let mut light_bulb = TransformBuilder::new().translate(light.position.0, light.position.1, light.position.2).scale(0.2, 0.2, 0.2).build();
    let (mut yaw, mut pitch) = (FRAC_PI_2 * 2., 0.0);
    let mut state = State::default();

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
            update_light_color(&mut light_points, &mut state);
            display.gl_window().window().request_redraw();
        }
        Event::RedrawRequested(_) => {
            tick_system.start_tick(TICK_RENDER_ID);

            egui.begin_frame(&display);

            show_window(&mut egui, &mut state);

            let (needs_repaint, shapes) = egui.end_frame(&display);


            let mut frame = display.draw();
            let bgc = {
                let c = &state.background_color;
                (c[0], c[1], c[2], c[3])
            };
            frame.clear_color_and_depth(bgc, 1.);


            for i in 0..light_bulbs.len() {
                let model = light_bulbs[i].get_raw();
                let mut my_storage = UniformStorage::default();
                my_storage.add("vp", pre_vp.as_uniform_value());
                my_storage.add("model", model.as_uniform_value());
                my_storage.add("color", state.light_bulb_color[i].as_uniform_value());
                frame.draw(&cube_vertexes, &cube_indexes, &lighting_program, &my_storage, &draw_params).unwrap();
            }

            let view_pos: [f32; 3] = camera.pos.into();
            let view: RawMat4 = camera.view().into();
            {
                let model = floor_model.get_raw();
                let mut my_storage = UniformStorage::default();
                my_storage.add("vp", pre_vp.as_uniform_value());
                my_storage.add("view", view.as_uniform_value());
                my_storage.add("model", model.as_uniform_value());
                my_storage.add("viewPos", view_pos.as_uniform_value());
                my_storage.add("toggleTorchLight", toggle_torchlight.as_uniform_value());
                rock_soil_mat.as_uniform("material", &mut my_storage);
                dir_light.as_uniform("dirLight", &mut my_storage);
                light_spot.as_uniform("spotLight", &mut my_storage);
                for (i, lp) in light_points.iter().enumerate() {
                    lp.as_uniform(&format!("pointLights[{}]", i), &mut my_storage);
                }
                frame.draw(&square_vertexes, &square_indexes, &sample_program, &my_storage, &draw_params).unwrap();
            }

            for x in cube_models.iter() {
                let model = x.get_raw();
                let mut my_storage = UniformStorage::default();
                my_storage.add("vp", pre_vp.as_uniform_value());
                my_storage.add("view", view.as_uniform_value());
                my_storage.add("model", model.as_uniform_value());
                my_storage.add("viewPos", view_pos.as_uniform_value());
                my_storage.add("toggleTorchLight", toggle_torchlight.as_uniform_value());
                crate_mat.as_uniform("material", &mut my_storage);
                dir_light.as_uniform("dirLight", &mut my_storage);
                light_spot.as_uniform("spotLight", &mut my_storage);
                for (i, lp) in light_points.iter().enumerate() {
                    lp.as_uniform(&format!("pointLights[{}]", i), &mut my_storage);
                }
                frame.draw(&cube_vertexes, &cube_indexes, &sample_program, &my_storage, &draw_params).unwrap();
            }
            {
                let model = backpack_transform.get_raw();
                let mut my_storage = UniformStorage::default();
                my_storage.add("vp", pre_vp.as_uniform_value());
                my_storage.add("view", view.as_uniform_value());
                my_storage.add("model", model.as_uniform_value());
                my_storage.add("viewPos", view_pos.as_uniform_value());
                my_storage.add("toggleTorchLight", toggle_torchlight.as_uniform_value());
                dir_light.as_uniform("dirLight", &mut my_storage);
                light_spot.as_uniform("spotLight", &mut my_storage);
                backpack_mat.as_uniform("material", &mut my_storage);
                for (i, lp) in light_points.iter().enumerate() {
                    lp.as_uniform(&format!("pointLights[{}]", i), &mut my_storage);
                }
                backpack_model.draw(&mut frame, &model_program, &my_storage, &draw_params);
            }


            tick_system.start_tick(TICK_RENDER_EGUI_ID);
            egui.paint(&display, &mut frame, shapes);
            tick_system.end_tick(TICK_RENDER_EGUI_ID);

            frame.finish().unwrap();
            tick_system.end_tick(TICK_RENDER_ID);
            // tick_system.debug_tick(TICK_RENDER_ID);
        }
        Event::RedrawEventsCleared => {
            if input.poll_gesture(&binding.exit) || input.poll_gesture(&Gesture::QuitTrigger) || state.quit {
                *control_flow = ControlFlow::Exit;
            }
            if input.poll_gesture(&binding.fullscreen) {
                set_fullscreen(&display, &mut fullscreen);
            }
            if input.poll_gesture(&binding.toggle_torch_light) {
                toggle_torchlight = !toggle_torchlight;
            }
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
                light_spot.direction.data = direction.normalize();
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

            // rotate_camera_around_scene(&mut camera, &before_run);

            pre_vp = (perspective.get() * camera.view()).into();

            if let Some(duration) = tick_system.duration_since_frame_start() {
                let step = input.poll_analog2d(&binding.movement);
                if step.y != 0. {
                    camera.pos += camera.front * step.y * CAMERA_SPEED * (duration as f32);
                    light_spot.position.data = camera.pos.clone();
                }
                if step.x != 0. {
                    camera.pos += normalize(&cross(&camera.front, &camera.up)) * step.x * CAMERA_SPEED * (duration as f32);
                    light_spot.position.data = camera.pos.clone();
                }
                // rotate_light_around_scene(&mut light_position, duration as f32);
                // light_bulb.move_to(light_position.data.x, light_position.data.y, light_position.data.z);
                rotate_light_around_scene(&mut light_points[0].position, duration as f32);
                light_bulbs[0].move_to(light_points[0].position.data.x, light_points[0].position.data.y, light_points[0].position.data.z);
            }
            input.tick_reset();
            tick_system.end_tick(TICK_FRAME_ID);
            // tick_system.debug_tick(TICK_FRAME_ID);
            tick_system.update_time();
            if tick_system.should_reset() {
                tick_system.debug_tick_iteration();
                tick_system.reset();
            }
            tick_system.start_tick(TICK_FRAME_ID);
        },
        _ => {
            match &event {
                Event::WindowEvent { event, .. } => {
                    if egui.is_quit_event(&event) {
                        *control_flow = glium::glutin::event_loop::ControlFlow::Exit;
                    }
                    egui.on_event(event)
                },
                _ => {}
            }
            input.update(&event);
        },
    });
}

fn update_light_color(lights: &mut [PointLight; 4], state: &mut State) {
    for i in 0..lights.len() {
        let l = &mut lights[i];
        let color = &state.light_bulb_color[i];
        l.ambient = GVec3::new(color[0], color[1], color[2]);
        l.ambient.data *= 0.1;
        l.diffuse = GVec3::new(color[0], color[1], color[2]);
        l.specular = GVec3::new(color[0], color[1], color[2]);
    }
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
    *light_pos.data = *math::glm::rotate_vec3(&mut light_pos.data, PI / 14. * delta, &vec3(0.0, 0.0, 1.0));
}

fn _rotate_light_around_scene_raw(light_pos: &mut (f32, f32, f32), delta: f32) {
    let tmp = vec3(light_pos.0, light_pos.1, light_pos.2);
    let tmp = math::glm::rotate_vec3(&tmp, delta, &vec3(0.0, 0.0, 1.0));
    *light_pos = (tmp.x, tmp.y, tmp.z);
}

