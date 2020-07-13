mod interpreter;

use interpreter::Interpreter;
use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let mut f = File::open("roms/MAZE")?;
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
    let mut interpreter = Interpreter::new(instructions);
    interpreter.interpret();
    Ok(())
}
