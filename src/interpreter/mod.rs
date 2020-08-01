use crate::game::{GameState, KeyState};
use std::{thread, time};
use std::sync::{Mutex, Arc};
use std::num::Wrapping;

// Instructions associated with their decode scheme
#[derive(Debug)]
enum Instruction {
    ClearDisplay,
    ReturnFromSubroutine,
    JumpToLoc(u16), // Addr
    CallSubroutine(u16), // Addr
    SkipEq(u8, u8), // Vx, byte
    SkipNeq(u8, u8), // Vx, byte
    SkipRegsEq(u8, u8), // Vx, Vy
    SetReg(u8, u8), // Vx, byte
    AddReg(u8, u8), // Vx, byte
    SetRegFromReg(u8, u8), // Vx, Vy
    BitwiseOr(u8, u8), // Vx, Vy
    BitwiseAnd(u8, u8), // Vx, Vy
    BitwiseXor(u8, u8), // Vx, Vy
    AddRegWithCarry(u8, u8), // Vx, Vy
    SubReg(u8, u8), // Vx, Vy
    ShiftRight(u8, u8), // Vx, Vy (Vy not needed)
    SubRegBackwards(u8, u8), // Vx, Vy
    ShiftLeft(u8, u8), // Vx, Vy (Vy not needed)
    SkipRegsNeq(u8, u8), // Vx, Vy
    SetI(u16), // Addr
    JumpToLocRel(u16), // Offset
    Random(u8, u8), // Vx, kk
    DrawSprite(u8, u8, u8), // Vx, Vy, nibble
    SkipIfPressed(u8), // Vx
    SkipIfNotPressed(u8), // Vx
    SetRegToDelayTimer(u8), // Vx
    BlockOnKeypress(u8), // Vx
    SetDelayTimer(u8), // Vx
    SetSoundTimer(u8), // Vx
    AddI(u8), // Vx
    LoadSprite(u8), // Vx
    ToDecimal(u8), // Vx
    CopyRegsIntoMemory(u8), // Vx
    CopyRegsFromMemory(u8), // Vx
    InvalidInstruction(u16),
}

fn decode(byte_code: u16) -> Instruction {
    let first_nibble = ((byte_code & 0xF000) >> 12) as u8;
    let last_nibble = (byte_code & 0x000F) as u8;
    let addr = byte_code & 0x0FFF;
    let x = ((byte_code & 0x0F00) >> 8) as u8;
    let y = ((byte_code & 0x00F0) >> 4) as u8;
    let kk = (byte_code & 0x00FF) as u8;

    use Instruction::*;
    match first_nibble {
        0x00 => match byte_code {
            0x00E0 => return ClearDisplay,
            0x00EE => return ReturnFromSubroutine,
            _ => return InvalidInstruction(byte_code),
        },
        0x01 => return JumpToLoc(addr),
        0x02 => return CallSubroutine(addr),
        0x03 => return SkipEq(x, kk),
        0x04 => return SkipNeq(x, kk),
        0x05 => return SkipRegsEq(x, y),
        0x06 => return SetReg(x, kk),
        0x07 => return AddReg(x, kk),
        0x08 => match last_nibble {
            0x00 => return SetRegFromReg(x, y),
            0x01 => return BitwiseOr(x, y),
            0x02 => return BitwiseAnd(x, y),
            0x03 => return BitwiseXor(x, y),
            0x04 => return AddRegWithCarry(x, y),
            0x05 => return SubReg(x, y),
            0x06 => return ShiftRight(x, y),
            0x07 => return SubRegBackwards(x, y),
            0x0E => return ShiftLeft(x, y),
            _ => return InvalidInstruction(byte_code),
        },
        0x09 => SkipRegsNeq(x, y),
        0x0A => SetI(addr),
        0x0B => JumpToLocRel(addr),
        0x0C => Random(x, kk),
        0x0D => DrawSprite(x, y, last_nibble),
        0x0E => match kk {
            0x9E => return SkipIfPressed(x),
            0xA1 => return SkipIfNotPressed(x),
            _ => return InvalidInstruction(byte_code),
        },
        0x0F => match kk {
            0x07 => return SetRegToDelayTimer(x),
            0x0A => return BlockOnKeypress(x),
            0x15 => return SetDelayTimer(x),
            0x18 => return SetSoundTimer(x),
            0x1E => return AddI(x),
            0x29 => return LoadSprite(x),
            0x33 => return ToDecimal(x),
            0x55 => return CopyRegsIntoMemory(x),
            0x65 => return CopyRegsFromMemory(x),
            _ => return InvalidInstruction(byte_code),
        },
        _ => return InvalidInstruction(byte_code),
    }
}

pub struct Interpreter {
    mem: Memory,
    game: Arc<Mutex<GameState>>,
    running: bool,
}

impl Interpreter {
    pub fn new(program: Vec<u16>, game: Arc<Mutex<GameState>>) -> Self {
        Self { 
            mem: Memory::new(program),
            game: game,
            running: false
        }
    }

    pub fn print_program(&mut self) {
        for _ in 0..150 {
            let byte_code = self.mem.fetch_instruction();
            let instruction = decode(byte_code);
            let address = self.mem.get_pc();
            use Instruction::*;
            println!("{:?}: {:?}", address, instruction);
            self.mem.inc_pc();
        }
    }

    pub fn interpret(&mut self) {
        self.running = true;
        while self.running {
            let byte_code = self.mem.fetch_instruction();
            let address = self.mem.get_pc();
            let instruction = decode(byte_code);
            use Instruction::*;
            println!("{:?}: {:?}", address, instruction);
            // self.screen.print();
            match instruction {
                ReturnFromSubroutine => {
                    if let Some(addr) = self.mem.pop_stack() {
                        self.mem.set_pc(addr);
                    } else {
                        self.running = false;
                        eprintln!("Attempted to pop from an empty stack!");
                    }
                },
                JumpToLoc(addr) => self.mem.set_pc(addr),
                CallSubroutine(addr) => {
                    self.mem.push_stack(self.mem.get_pc() as u16 + 2);
                    self.mem.set_pc(addr);
                    println!("{:X}", byte_code);
                },
                SkipEq(reg_idx, byte) => {
                    if self.mem.get_reg(reg_idx) == byte {
                        self.mem.double_inc_pc();
                    } else {
                        self.mem.inc_pc();
                    }
                },
                SkipNeq(reg_idx, byte) => {
                    if self.mem.get_reg(reg_idx) != byte {
                        self.mem.double_inc_pc();
                    } else {
                        self.mem.inc_pc();
                    }
                },
                SkipRegsEq(reg_idx, reg_idy) => {
                    if self.mem.get_reg(reg_idx) == self.mem.get_reg(reg_idy) {
                        self.mem.double_inc_pc();
                    } else {
                        self.mem.inc_pc();
                    }
                },
                SetReg(reg_idx, byte) => {
                    self.mem.set_reg(reg_idx, byte);
                    self.mem.inc_pc();
                },
                AddReg(reg_idx, byte) => {
                    let sum = (Wrapping(self.mem.get_reg(reg_idx)) + Wrapping(byte)).0;
                    self.mem.set_reg(reg_idx, sum);
                    self.mem.inc_pc();
                },
                SetRegFromReg(reg_idx, reg_idy) => {
                    self.mem.set_reg(reg_idx, self.mem.get_reg(reg_idy));
                    self.mem.inc_pc();
                },
                BitwiseOr(reg_idx, reg_idy) => {
                    let or = self.mem.get_reg(reg_idx) | self.mem.get_reg(reg_idy);
                    self.mem.set_reg(reg_idx, or);
                    self.mem.inc_pc();
                },
                BitwiseAnd(reg_idx, reg_idy) => {
                    let and = self.mem.get_reg(reg_idx) & self.mem.get_reg(reg_idy);
                    self.mem.set_reg(reg_idx, and);
                    self.mem.inc_pc();
                },
                BitwiseXor(reg_idx, reg_idy) => {
                    let xor = self.mem.get_reg(reg_idx) ^ self.mem.get_reg(reg_idy);
                    self.mem.set_reg(reg_idx, xor);
                    self.mem.inc_pc();
                },
                AddRegWithCarry(reg_idx, reg_idy) => {
                    let sum = (Wrapping(self.mem.get_reg(reg_idx)) + Wrapping(self.mem.get_reg(reg_idy))).0;
                    let carry = self.mem.get_reg(reg_idx) as u16 + self.mem.get_reg(reg_idy) as u16 > 255;
                    if carry {
                        self.mem.set_reg(0x0F, 0x01);
                    } else {
                        self.mem.set_reg(0x0F, 0x01);
                    }
                    self.mem.set_reg(reg_idx, sum);
                    self.mem.inc_pc();
                },
                SubReg(reg_idx, reg_idy) => {
                    let diff = (Wrapping(self.mem.get_reg(reg_idx)) - Wrapping(self.mem.get_reg(reg_idy))).0;
                    if self.mem.get_reg(reg_idx) > self.mem.get_reg(reg_idy) {
                        self.mem.set_reg(0x0F, 0x01);
                    } else {
                        self.mem.set_reg(0x0F, 0x01);
                    }
                    self.mem.set_reg(reg_idx, diff);
                    self.mem.inc_pc();
                },
                ShiftRight(reg_idx, _) => {
                    let lsb = self.mem.get_reg(reg_idx) & 0x01;
                    self.mem.set_reg(0x0F, lsb);
                    self.mem.set_reg(reg_idx, self.mem.get_reg(reg_idx) / 2);
                    self.mem.inc_pc();
                },
                SubRegBackwards(reg_idx, reg_idy) => {
                    let diff = (Wrapping(self.mem.get_reg(reg_idy)) - Wrapping(self.mem.get_reg(reg_idx))).0;
                    if self.mem.get_reg(reg_idy) > self.mem.get_reg(reg_idx) {
                        self.mem.set_reg(0x0F, 0x01);
                    } else {
                        self.mem.set_reg(0x0F, 0x01);
                    }
                    self.mem.set_reg(reg_idx, diff);
                    self.mem.inc_pc();
                },
                ShiftLeft(reg_idx, _) => {
                    let msb = (self.mem.get_reg(reg_idx) & 0x80) >> 7;
                    self.mem.set_reg(0x0F, msb);
                    self.mem.set_reg(reg_idx, self.mem.get_reg(reg_idx) * 2);
                },
                SkipRegsNeq(reg_idx, reg_idy) => {
                    if self.mem.get_reg(reg_idx) != self.mem.get_reg(reg_idy) {
                        self.mem.double_inc_pc();
                    } else {
                        self.mem.inc_pc();
                    }
                },
                SetI(addr) => {
                    self.mem.set_ireg(addr);
                    self.mem.inc_pc();
                },
                JumpToLocRel(offset) => {
                    self.mem.set_pc(offset + self.mem.get_reg(0x00) as u16);
                },
                Random(reg_idx, byte) => {
                    use rand::Rng;
                    let mut rng = rand::thread_rng();
                    let random: u8 = rng.gen();
                    self.mem.set_reg(reg_idx, random & byte);
                    self.mem.inc_pc();
                },
                DrawSprite(reg_idx, reg_idy, n) => {
                    let x = self.mem.get_reg(reg_idx);
                    let y = self.mem.get_reg(reg_idy);
                    let mut bytes = Vec::new();
                    let i = self.mem.get_ireg();
                    for offset in 0..n {
                        let addr = i + offset as u16;
                        let byte = self.mem.get(addr as usize);
                        bytes.push(byte);
                    }
                    self.display_byte_sprite(x as usize, y as usize, bytes);
                    self.mem.inc_pc();
                },
                SkipIfPressed(reg_idx) => {
                    let key_id = self.mem.get_reg(reg_idx);
                    if self.game.lock().unwrap().get_key_state(key_id) == KeyState::Pressed {
                        self.mem.double_inc_pc();
                    } else {
                        self.mem.inc_pc();
                    }
                },
                SkipIfNotPressed(reg_idx) => {
                    let key_id = self.mem.get_reg(reg_idx);
                    if self.game.lock().unwrap().get_key_state(key_id) == KeyState::Released {
                        self.mem.double_inc_pc();
                    } else {
                        self.mem.inc_pc();
                    }
                },
                SetRegToDelayTimer(reg_idx) => {
                    self.mem.set_reg(reg_idx, self.mem.get_dt_reg());
                    self.mem.inc_pc();
                },
                BlockOnKeypress(reg_idx) => {
                    let mut blocked = true;
                    while blocked {
                        let state = self.game.lock().unwrap();
                        for key_id in 0..16 {
                            if state.get_key_state(key_id) == KeyState::Pressed {
                                blocked = false;
                                self.mem.set_reg(reg_idx, key_id);
                            }
                        }
                        thread::sleep(time::Duration::from_millis(1));
                    }
                    self.mem.inc_pc();
                },
                SetDelayTimer(reg_idx) => {
                    self.mem.set_dt_reg(self.mem.get_reg(reg_idx));
                    self.mem.inc_pc();
                },
                SetSoundTimer(reg_idx) => {
                    self.mem.set_st_reg(self.mem.get_reg(reg_idx));
                    self.mem.inc_pc();
                },
                AddI(reg_idx) => {
                    let sum = self.mem.get_reg(reg_idx) as u16 + self.mem.get_ireg();
                    self.mem.set_ireg(sum);
                    self.mem.inc_pc();
                },
                // LoadSprite(reg_idx) => {
                   
                // },
                ToDecimal(reg_idx) => {
                    let num = self.mem.get_reg(reg_idx);
                    let hundreds: u16 = (num / 100).into();
                    let tens: u16 = ((num % 100) / 10).into();
                    let ones: u16 = (num % 10).into();
                    let bcd = hundreds << 12 | tens << 8 | ones << 4;
                    self.mem.set_ireg(bcd); 
                    self.mem.inc_pc();
                }
                // CopyRegsIntoMemory(u8), // Vx
                // CopyRegsFromMemory(u8), // Vx

                InvalidInstruction(byte_code) => {
                    println!("Error: {:X} unrecognized", byte_code);
                    return
                }
                _ => {
                    println!("{:?} not implemented yet", instruction);
                    self.mem.inc_pc();
                },
            }
            thread::sleep(time::Duration::from_millis(1));
        }
    }

    fn display_byte_sprite(&mut self, x: usize, y: usize, bytes: Vec<u8>) -> bool {
        for (i, byte) in bytes.iter().enumerate() {
            for j in 0..8 {
                let bit = (*byte & (0x80 >> j)) >> (7-j);
                let mut pixel_state: bool = false;
                match bit {
                    0x00 => pixel_state = false,
                    0x01 => pixel_state = true,
                    _ => eprintln!("bit is in invalid state"),
                }
                self.game.lock().unwrap().set_pixel(x+j, y+i, pixel_state);
            }
        }
        // TODO return true if occluded
        return false;
    }
}

struct Memory {
    ram: [u8; 0xFFF],
    program_addr: usize,
    program_counter: usize,
    stack: Vec<u16>,
    registers: [u8; 16],
    i_reg: u16,
    dt_reg: Arc<Mutex<u8>>,
    st_reg: Arc<Mutex<u8>>,
}

impl Memory {
    fn new(program: Vec<u16>) -> Self {
        let mut mem = Memory {
            ram: [0x00; 0xFFF],
            program_addr: 0x200,
            program_counter: 0x200,
            stack: Vec::new(),
            registers: [0x00; 16],
            i_reg: 0x0000,
            dt_reg: Arc::new(Mutex::new(0x00)),
            st_reg: Arc::new(Mutex::new(0x00)),
        };
        mem.load_program(program);
        return mem;
    }

    fn load_program(&mut self, program: Vec<u16>) {
        let mut addr = self.program_addr;
        for instruction in program {
            let first_byte = ((instruction & 0xFF00) >> 8) as u8;
            let second_byte = (instruction & 0x00FF) as u8;
            self.set(addr, first_byte);
            self.set(addr+1, second_byte);
            addr += 2;
        }
    }

    fn fetch_instruction(&self) -> u16 {
        let first_byte = self.ram[self.program_counter];
        let second_byte = self.ram[self.program_counter + 1];
        return ((first_byte as u16) << 8) | (second_byte as u16);
    }

    fn inc_pc(&mut self) {
        self.program_counter += 2;
    }

    fn double_inc_pc(&mut self) {
        self.program_counter += 4;
    }

    fn get_reg(&self, reg: u8) -> u8 {
        return self.registers[reg as usize];
    }

    fn set_reg(&mut self, reg: u8, value: u8) {
        self.registers[reg as usize] = value
    }

    fn get_ireg(&self) -> u16 {
        return self.i_reg;
    }

    fn set_ireg(&mut self, value: u16) {
        self.i_reg = value;
    }

    fn get_dt_reg(&self) -> u8 {
        return self.dt_reg.lock().unwrap().clone();
    }

    fn set_dt_reg(&mut self, value: u8) {
        let dt_arc_clone = self.dt_reg.clone();
        thread::spawn(move || {
            thread::sleep(time::Duration::from_secs_f64(1.0 / 60.0));
            let dt = *dt_arc_clone.lock().unwrap();
            if dt == 0 {
                return;
            }
            *dt_arc_clone.lock().unwrap() -= 1;
        });
    }

    fn get_st_reg(&self) -> u8 {
        return self.st_reg.lock().unwrap().clone();
    }

    fn set_st_reg(&mut self, value: u8) {
        let st_arc_clone = self.st_reg.clone();
        thread::spawn(move || {
            thread::sleep(time::Duration::from_secs_f64(1.0 / 60.0));
            let st = *st_arc_clone.lock().unwrap();
            if st == 0 {
                return;
            }
            *st_arc_clone.lock().unwrap() -= 1;
        });
    }

    fn get_pc(&self) -> usize {
        return self.program_counter;
    }

    fn set_pc(&mut self, pc: u16) {
        self.program_counter = pc as usize;
    }


    fn push_stack(&mut self, data: u16) {
        self.stack.push(data);
    }

    fn pop_stack(&mut self) -> Option<u16> {
        return self.stack.pop()
    }

    fn get(&self, addr: usize) -> u8 {
        return self.ram[addr];
    }

    fn set(&mut self, addr: usize, value: u8) {
        self.ram[addr] = value;
    }
}
