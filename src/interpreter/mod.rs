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
    program: Vec<u16>,
    program_counter: usize,
    stack: Vec<u16>,
    registers: [u8; 16],
    i_reg: u16,
    dt_reg: u8,
    st_reg: u8,
    running: bool,
}

impl Interpreter {
    pub fn new(program: Vec<u16>) -> Self {
        Interpreter{ 
            program, 
            program_counter: 0, 
            stack: vec![],
            registers: [0x00; 16],
            i_reg: 0x0000,
            dt_reg: 0x00,
            st_reg: 0x00,
            running: false}
    }

    pub fn interpret(&mut self) {
        self.running = true;
        while self.running && self.program_counter < self.program.len() {
            let byte_code = self.program[self.program_counter];
            let instruction = decode(byte_code);
            use Instruction::*;
            println!("{:?}", instruction);
            match instruction {
                ReturnFromSubroutine => {
                    if let Some(addr) = self.stack.pop() {
                        self.program_counter = addr as usize;
                    } else {
                        self.running = false;
                        eprintln!("Attempted to pop from an empty stack!");
                    }
                },
                JumpToLoc(addr) => self.program_counter = addr as usize,
                CallSubroutine(addr) => {
                    self.stack.push(addr);
                    self.program_counter = addr as usize;
                    println!("{:X}", byte_code);
                },
                SkipEq(reg_idx, byte) => {
                    if self.registers[reg_idx as usize] == byte {
                        self.program_counter += 2;
                    } else {
                        self.program_counter += 1;
                    }
                },
                SkipNeq(reg_idx, byte) => {
                    if self.registers[reg_idx as usize] != byte {
                        self.program_counter += 2;
                    } else {
                        self.program_counter += 1;
                    }
                },
                SkipRegsEq(reg_idx, reg_idy) => {
                    if self.registers[reg_idx as usize] == self.registers[reg_idy as usize] {
                        self.program_counter += 2;
                    } else {
                        self.program_counter += 1;
                    }
                },
                SetReg(reg_idx, byte) => {
                    self.registers[reg_idx as usize] = byte;
                    self.program_counter += 1;
                }
                AddReg(reg_idx, byte) => {
                    self.registers[reg_idx as usize] = self.registers[reg_idx as usize] + byte;
                    self.program_counter += 1;
                }
                SetRegFromReg(reg_idx, reg_idy) => {
                    self.registers[reg_idx as usize] = self.registers[reg_idy as usize];
                    self.program_counter += 1;
                }

                InvalidInstruction(byte_code) => {
                    println!("Error: {:X} unrecognized", byte_code);
                    self.program_counter += 1;
                }
                _ => {
                    println!("{:?} not implemented yet", instruction);
                    self.program_counter += 1;
                },
            }
        }
    }
}