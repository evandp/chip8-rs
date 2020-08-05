mod interpreter;
mod game;

use interpreter::Interpreter;
use game::*;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::sync::Mutex;
use std::sync::Arc;
use std::thread;

fn main() -> io::Result<()> {
    let mut f = File::open("roms/INVADERS")?;
    let mut read_buffer = Vec::new();
    f.read_to_end(&mut read_buffer)?;
    let mut first_byte = true;
    let mut half_word = 0x0000;
    let mut instructions = Vec::new();
    for byte in read_buffer {
        if first_byte {
            half_word |= (byte as u16) << 8;
            first_byte = false;
        } else {
            half_word |= byte as u16;
            instructions.push(half_word);
            half_word = 0x0000;
            first_byte = true;
        }
    }
    let display_state = Arc::new(Mutex::new(GameState::new()));
    let clone = display_state.clone();
    let mut interpreter = Interpreter::new(instructions, clone);
    thread::spawn(|| {
        let mut display = Game::new("Test title".to_string(), display_state);
        display.start();
    });
    // interpreter.print_program();
    interpreter.interpret();

    Ok(())
}
