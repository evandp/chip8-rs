extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use crate::game::piston::{PressEvent, ReleaseEvent, Button, Key};
use std::sync::Mutex;
use std::sync::Arc;
use std::{thread, time};

const WIDTH: usize = 80;
const HEIGHT: usize = 64;

#[derive(Copy, Clone, PartialEq)]
pub enum KeyState {
    Pressed,
    Released,
}

pub struct GameState {
    display: [[bool; HEIGHT]; WIDTH],
    keys: [KeyState;16],
}

pub struct Game {
    gl: GlGraphics,
    window: Window,
    state: Arc<Mutex<GameState>>,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            display: [[false; HEIGHT as usize]; WIDTH as usize],
            keys: [KeyState::Released; 16],
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, pixel_state: bool) {
        self.display[x % WIDTH][y % HEIGHT] = pixel_state;
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> bool {
        self.display[x % WIDTH][y % HEIGHT]
    }

    pub fn clear_display(&mut self) {
        self.display = [[false; HEIGHT as usize]; WIDTH as usize];
    }

    pub fn get_key_state(&self, key_id: u8) -> KeyState {
        self.keys[(key_id) as usize]
    }
}

impl Game {
    pub fn new(title: String, state: Arc<Mutex<GameState>>) -> Self {
        // Change this to OpenGL::V2_1 if not working.
        let opengl = OpenGL::V3_2;

        let window = WindowSettings::new(title, [800, 640])
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();

        return Game {
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
            if let Some(args) = e.press_args() {
                let mut keys = self.state.lock().unwrap().keys;
                match args {
                    Button::Keyboard(Key::X) => {keys[0] = KeyState::Pressed}
                    Button::Keyboard(Key::D1) => {keys[1] = KeyState::Pressed}
                    Button::Keyboard(Key::D2) => {keys[2] = KeyState::Pressed}
                    Button::Keyboard(Key::D3) => {keys[3] = KeyState::Pressed}
                    Button::Keyboard(Key::Q) => {keys[4] = KeyState::Pressed}
                    Button::Keyboard(Key::W) => {keys[5] = KeyState::Pressed}
                    Button::Keyboard(Key::E) => {keys[6] = KeyState::Pressed}
                    Button::Keyboard(Key::A) => {keys[7] = KeyState::Pressed}
                    Button::Keyboard(Key::S) => {keys[8] = KeyState::Pressed}
                    Button::Keyboard(Key::D) => {keys[9] = KeyState::Pressed}
                    Button::Keyboard(Key::Z) => {keys[10] = KeyState::Pressed}
                    Button::Keyboard(Key::C) => {keys[11] = KeyState::Pressed}
                    Button::Keyboard(Key::D4) => {keys[12] = KeyState::Pressed}
                    Button::Keyboard(Key::R) => {keys[13] = KeyState::Pressed}
                    Button::Keyboard(Key::F) => {keys[14] = KeyState::Pressed}
                    Button::Keyboard(Key::V) => {keys[15] = KeyState::Pressed}
                    _ => {}
                }
            }
            if let Some(args) = e.release_args() {
                let mut keys = self.state.lock().unwrap().keys;
                match args {
                    Button::Keyboard(Key::X) => {keys[0] = KeyState::Released}
                    Button::Keyboard(Key::D1) => {keys[1] = KeyState::Released}
                    Button::Keyboard(Key::D2) => {keys[2] = KeyState::Released}
                    Button::Keyboard(Key::D3) => {keys[3] = KeyState::Released}
                    Button::Keyboard(Key::Q) => {keys[4] = KeyState::Released}
                    Button::Keyboard(Key::W) => {keys[5] = KeyState::Released}
                    Button::Keyboard(Key::E) => {keys[6] = KeyState::Released}
                    Button::Keyboard(Key::A) => {keys[7] = KeyState::Released}
                    Button::Keyboard(Key::S) => {keys[8] = KeyState::Released}
                    Button::Keyboard(Key::D) => {keys[9] = KeyState::Released}
                    Button::Keyboard(Key::Z) => {keys[10] = KeyState::Released}
                    Button::Keyboard(Key::C) => {keys[11] = KeyState::Released}
                    Button::Keyboard(Key::D4) => {keys[12] = KeyState::Released}
                    Button::Keyboard(Key::R) => {keys[13] = KeyState::Released}
                    Button::Keyboard(Key::F) => {keys[14] = KeyState::Released}
                    Button::Keyboard(Key::V) => {keys[15] = KeyState::Released}
                    _ => {}
                }
            }
            thread::sleep(time::Duration::from_millis(1));
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
