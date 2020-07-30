extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use std::sync::Mutex;
use std::sync::Arc;

const WIDTH: usize = 80;
const HEIGHT: usize = 64;

pub struct DisplayState {
    state: [[bool; HEIGHT]; WIDTH],
}

pub struct Display {
    gl: GlGraphics,
    window: Window,
    state: Arc<Mutex<DisplayState>>,
}

impl DisplayState {
    pub fn new() -> Self {
        DisplayState {
            state: [[false; HEIGHT as usize]; WIDTH as usize],
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, pixel_state: bool) {
        self.state[x % WIDTH][y % HEIGHT] = pixel_state;
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> bool {
        self.state[x % WIDTH][y % HEIGHT]
    }
}

impl Display {
    pub fn new(title: String, state: Arc<Mutex<DisplayState>>) -> Self {
        // Change this to OpenGL::V2_1 if not working.
        let opengl = OpenGL::V3_2;

        let window = WindowSettings::new(title, [800, 640])
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();

        return Display {
            state: state,
            gl: GlGraphics::new(opengl),
            window: window,
        }
    }

    pub fn start(&mut self) {
        let mut events = Events::new(EventSettings::new());
        while let Some(e) = events.next({
            // scoping so lock gets released
            &mut self.window
        }) {
            if let Some(args) = e.render_args() {
                self.render(&args);
            }
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        for i in 0..WIDTH {
            let x = args.window_size[0] / WIDTH as f64 * i as f64;
            for j in 0..HEIGHT {
                let y = args.window_size[1] / HEIGHT as f64 * j as f64;

                let square = rectangle::square(0.0, 0.0, 20.0);
                
                {
                    let pixel_state = self.state.lock().unwrap().get_pixel(i, j);
                    self.gl.draw(args.viewport(), |c, gl| {
                        let color: [f32; 4];
                        if pixel_state {
                            color = WHITE;
                        } else {
                            color = BLACK;
                        }

                        let transform = c
                            .transform
                            .trans(x, y);
                        
                        rectangle(color, square, transform, gl);
                    });
                } 
            }
        }
    }
}
