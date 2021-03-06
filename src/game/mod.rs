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
use std::sync::atomic::AtomicBool;
use std::{thread, time};
use device_query::{DeviceQuery, DeviceState, Keycode};

const WIDTH: usize = 80;
const HEIGHT: usize = 64;

#[derive(Copy, Clone, PartialEq, Debug)]
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
        let ds = DeviceState::new();
        let key_list = vec!(Keycode::X, Keycode::Key1, Keycode::Key2, Keycode::Key3, 
            Keycode::Q, Keycode::W, Keycode::E, Keycode::A, Keycode::S, Keycode::D, 
            Keycode::Z, Keycode::C, Keycode::Key4, Keycode::R, Keycode::F, Keycode::V);
        while let Some(e) = events.next({
            // scoping so lock gets released
            &mut self.window
        }) {
            if let Some(args) = e.render_args() {
                self.render(&args);
            }
            let key_presses: Vec<Keycode> = ds.get_keys();
            // let mut keys = self.state.lock().unwrap().keys;
            for key in key_presses.iter() {
                match key {
                    Keycode::X => {self.state.lock().unwrap().keys[0] = KeyState::Pressed}
                    Keycode::Key1 => {self.state.lock().unwrap().keys[1] = KeyState::Pressed}
                    Keycode::Key2 => {self.state.lock().unwrap().keys[2] = KeyState::Pressed}
                    Keycode::Key3 => {self.state.lock().unwrap().keys[3] = KeyState::Pressed}
                    Keycode::Q => {self.state.lock().unwrap().keys[4] = KeyState::Pressed}
                    Keycode::W => {self.state.lock().unwrap().keys[5] = KeyState::Pressed}
                    Keycode::E => {self.state.lock().unwrap().keys[6] = KeyState::Pressed}
                    Keycode::A => {self.state.lock().unwrap().keys[7] = KeyState::Pressed}
                    Keycode::S => {self.state.lock().unwrap().keys[8] = KeyState::Pressed}
                    Keycode::D => {self.state.lock().unwrap().keys[9] = KeyState::Pressed}
                    Keycode::Z => {self.state.lock().unwrap().keys[10] = KeyState::Pressed}
                    Keycode::C => {self.state.lock().unwrap().keys[11] = KeyState::Pressed}
                    Keycode::Key4 => {self.state.lock().unwrap().keys[12] = KeyState::Pressed}
                    Keycode::R => {self.state.lock().unwrap().keys[13] = KeyState::Pressed}
                    Keycode::F => {self.state.lock().unwrap().keys[14] = KeyState::Pressed}
                    Keycode::V => {self.state.lock().unwrap().keys[15] = KeyState::Pressed}
                    _ => {}
                }
            }
            for key in key_list.iter() {
                if !key_presses.contains(&key) {
                    match key {
                        Keycode::X => {self.state.lock().unwrap().keys[0] = KeyState::Released}
                        Keycode::Key1 => {self.state.lock().unwrap().keys[1] = KeyState::Released}
                        Keycode::Key2 => {self.state.lock().unwrap().keys[2] = KeyState::Released}
                        Keycode::Key3 => {self.state.lock().unwrap().keys[3] = KeyState::Released}
                        Keycode::Q => {self.state.lock().unwrap().keys[4] = KeyState::Released}
                        Keycode::W => {self.state.lock().unwrap().keys[5] = KeyState::Released}
                        Keycode::E => {self.state.lock().unwrap().keys[6] = KeyState::Released}
                        Keycode::A => {self.state.lock().unwrap().keys[7] = KeyState::Released}
                        Keycode::S => {self.state.lock().unwrap().keys[8] = KeyState::Released}
                        Keycode::D => {self.state.lock().unwrap().keys[9] = KeyState::Released}
                        Keycode::Z => {self.state.lock().unwrap().keys[10] = KeyState::Released}
                        Keycode::C => {self.state.lock().unwrap().keys[11] = KeyState::Released}
                        Keycode::Key4 => {self.state.lock().unwrap().keys[12] = KeyState::Released}
                        Keycode::R => {self.state.lock().unwrap().keys[13] = KeyState::Released}
                        Keycode::F => {self.state.lock().unwrap().keys[14] = KeyState::Released}
                        Keycode::V => {self.state.lock().unwrap().keys[15] = KeyState::Released}
                        _ => {}
                    }
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
