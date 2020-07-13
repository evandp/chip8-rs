extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

const WIDTH: usize = 80;
const HEIGHT: usize = 64;
pub struct Display {
    gl: GlGraphics,
    state: [[bool; HEIGHT]; WIDTH],
}

// main code to come back to later

// let mut display = Display::New("This is a test".to_string());

// for i in 0..HEIGHT {
//     display.set_pixel(i, i, true);
// }

// let mut events = Events::new(EventSettings::new());
// while let Some(e) = events.next(&mut window) {
//     if let Some(args) = e.render_args() {
//         display.render(&args);
//     }
// }

impl Display {

    fn New(title: String) -> Self {
        // Change this to OpenGL::V2_1 if not working.
        let opengl = OpenGL::V3_2;

        // Create an Glutin window.
        let mut window: Window = WindowSettings::new(title, [800, 640])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

        // Create a new game and run it.
        let mut display = Display {
            gl: GlGraphics::new(opengl),
            state: [[false; HEIGHT as usize]; WIDTH as usize],
        };

        return display
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

                let pixel_state = self.state[i][j];
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

    fn set_pixel(&mut self, x: usize, y: usize, pixel_state: bool) -> bool {
        if x < 0 || x > WIDTH || y < 0 || y > HEIGHT {
            return false;
        }
        self.state[x][y] = pixel_state;
        return true;
    }
}
