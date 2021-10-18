use debug_ui::{EguiGlium, Grid, Layout, SidePanel, TopBottomPanel, Ui, Widget};
use debug_ui::color::Hsva;
use debug_ui::Window as DWindow;
use graphics::glium::Display;
use graphics::glium::glutin::window::Fullscreen;

pub mod geometry;
pub mod tick;

pub fn set_fullscreen(display: &Display, fullscreen: &mut bool) {
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

pub struct State {
    pub open_debug: bool,
    pub background_color: [f32; 4],
    pub light_bulb_color: [[f32; 4]; 4],
    pub frame_time: u128,
    pub quit: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            open_debug: false,
            background_color: [0.0, 0.0, 0.0, 1.0],
            light_bulb_color: [
                [1.0, 1.0, 1.0, 1.0],
                [1.0, 1.0, 1.0, 1.0],
                [1.0, 1.0, 1.0, 1.0],
                [1.0, 1.0, 1.0, 1.0]
            ],
            frame_time: 0,
            quit: false,
        }
    }
}

fn label<'a>(title: &'a str) -> impl Widget + 'a {
    let label = format!("{}:", title);
    move |ui: &mut Ui| {
        ui.label(label)
    }
}

fn show_widgets(ui: &mut Ui, state: &mut State) {
    ui.add(label("Background"));
    ui.color_edit_button_rgba_premultiplied(&mut state.background_color);
    ui.end_row();
    for i in 0..state.light_bulb_color.len() {
        ui.add(label(&format!("Bulb {}", i)));

        ui.color_edit_button_rgba_premultiplied(&mut state.light_bulb_color[i]);
        ui.end_row();
    }
}

pub fn show_window(egui: &mut EguiGlium, state: &mut State) {
    TopBottomPanel::top("my_top_bar").show(egui.ctx(), |ui| {
        ui.with_layout(Layout::left_to_right(), |ui| {
            ui.add(label("Hello world"));
            if ui.button("New window").clicked() {
                state.open_debug = true;
            }
        });
    });
    SidePanel::left("my_side_panel").min_width(150.).show(egui.ctx(), |ui| {
        ui.heading("Hello World!");
        if ui.button("Quit").clicked() {
            state.quit = true;
        }

        Grid::new("my_grid")
            .num_columns(2)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                show_widgets(ui, state);
            });
    });
    if state.open_debug {
        DWindow::new("Debug Window").min_width(150.).open(&mut state.open_debug).show(egui.ctx(), |ui| {
            ui.add(label("Debug label"));
        });
    }
}