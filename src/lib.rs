use std::fs::File;
use std::io::BufReader;

use graphics::glium::Display;
use graphics::glium::glutin::window::Fullscreen;

pub mod geometry;

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

