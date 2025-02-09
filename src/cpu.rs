use rand::Rng;

const MEMORY_SIZE: usize = 4096;
const STACK_SIZE: usize = 16;
const PROGRAM_START: usize = 0x200;
const REGISTERS_NUMBER: usize = 16;
pub const FONT_SET_START_ADDRESS: usize = 0x50;
pub const FONT_SET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

#[derive(Debug)]
struct CPU {
    pc: u16,
    i: u16,
    sp: u8,
    stack: [u16; STACK_SIZE],
    regs: [u8; REGISTERS_NUMBER],
    ram: [u8; MEMORY_SIZE],
    rng: rand::rngs::ThreadRng,
}

impl CPU {
    fn new() -> Self {
        let mut cpu = CPU {
            pc,
            i,
            sp,
            stack,
            regs,
            ram,
            rng: rand::thread_rng(),
        };

        cpu.load_font_set();
        cpu
    }

    pub fn cycle(&mut self) {
        let opcode: u16 = self.fetch();

        self.pc += 2;

        self.decode(opcode);
    }

    pub fn load_program(&mut self, buffer: &[u8]) {
        for i in 0..buffer.len() {
            self.ram[PROGRAM_START + i] = buffer[i];
        }
    }

    fn load_font_set(&mut self) {
        self.ram[FONT_SET_START_ADDRESS..(FONT_SET_START_ADDRESS + FONT_SET.len())].copy_from_slice(&FONT_SET);
    }

    fn fetch(&mut self) -> u16 {
        let opcode: u16 = (self.ram[self.pc] << 8) | (self.ram[self.pc + 1]);
        opcode
    }

    fn decode(&mut self, opcode: u16) {
        let vx = opcode & 0x0F00 >> 8;
        let vy = opcode & 0x00F0 >> 4;

        match (opcode & 0xF000 >> 12) {
            0x0 => {
                match (opcode & 0x00FF) {
                    0xE0 => self.op_00e0(),
                    0xEE => self.op_00ee(),
                    _ => println!("Unimplemented opcode: {:04X}", opcode),
                }
            },
            0x1 => self.op_1nnn(opcode),
            0x2 => self.op_2nnn(opcode),
            0x3 => self.op_3xkk(vx, opcode),
            0x4 => self.op_4xkk(vx, opcode),
            0x5 => self.op_5xy0(vx, vy),
            0x6 => self.op_6xkk(vx, opcode),
            0x7 => self.op_7xkk(vx, opcode),
            0x8 => {
                match (opcode & 0x000F) {
                    0x0 => self.op_8xy0(vx, vy),
                    0x1 => self.op_8xy1(vx, vy),
                    0x2 => self.op_8xy2(vx, vy),
                    0x3 => self.op_8xy3(vx, vy),
                    0x4 => self.op_8xy4(vx, vy),
                    0x5 => self.op_8xy5(vx, vy),
                    0x6 => self.op_8xy6(vx),
                    0x7 => self.op_8xy7(vx, vy),
                    0xE => self.op_8xye(vx, vy),
                    _ => println!("Unimplemented opcode: {:04X}", opcode),
                }
            },
            0x9 => self.op_9xy0(vx, vy),
            0xA => self.op_annnn(opcode),
            0xB => self.op_bnnnn(opcode),
            0xC => self.op_cxkk(vx, opcode),
            _ => println!("Unimplemented opcode: {:04X}", opcode),
        }
    }

    fn op_00e0(&mut self) {}

    fn op_00ee(&mut self) {
        self.sp -= 1;
        self.pc = self.stack[self.sp];
    }

    fn op_1nnn(&mut self, opcode: u16) {
        let address = opcode & 0x0FFF;
        self.pc = address;
    }

    fn op_2nnn(&mut self, opcode: u16) {
        let address = opcode & 0x0FFF;
        self.stack[self.sp] = self.pc;
        self.sp += 1;
        self.pc = address;
    }

    fn op_3xkk(&mut self, vx: u16, opcode: u16) {
        let byte = opcode & 0x00FF;

        if self.regs[vx] == byte {
            self.sp += 2;
        }
    }

    fn op_4xkk(&mut self, vx: u16, opcode: u16) {
        let byte = opcode & 0x00FF;

        if self.regs[vx] != byte {
            self.sp += 2;
        }
    }

    fn op_5xy0(&mut self, vx: u16, vy: u16) {
        if self.regs[vx] == self.regs[vy] {
            self.sp += 2;
        }
    }

    fn op_6xkk(&mut self, vx: u16, opcode: u16) {
        let byte = opcode & 0x00FF;

        self.regs[vx] = byte;
    }

    fn op_7xkk(&mut self, vx: u16, opcode: u16) {
        let byte = opcode & 0x00FF;

        self.regs[vx] += byte;
    }

    fn op_8xy0(&mut self, vx: u16, vy: u16) {
        self.regs[vx] = self.regs[vy];
    }

    fn op_8xy1(&mut self, vx: u16, vy: u16) {
        self.regs[vx] |= self.regs[vy];
    }

    fn op_8xy2(&mut self, vx: u16, vy: u16) {
        self.regs[vx] &= self.regs[vy];
    }

    fn op_8xy3(&mut self, vx: u16, vy: u16) {
        self.regs[vx] ^= self.regs[vy];
    }

    fn op_8xy4(&mut self, vx: u16, vy: u16) {
        let sum = self.regs[vx] + self.regs[vy];

        if sum > 255 {
            self.regs[0xF] = 1;
        } else {
            self.regs[0xF] = 0;
        }

        self.regs[vx] = sum & 0xFF;
    }

    fn op_8xy5(&mut self, vx: u16, vy: u16) {
        if self.regs[vx] > self.regs[vy] {
            self.regs[0xF] = 1;
        } else {
            self.regs[0xF] = 0;
        }

        self.regs[vx] -= self.regs[vy];
    }

    fn op_8xy6(&mut self, vx: u16) {
        self.regs[0xF] = self.regs[vx] & 1;
        self.regs[vx] >>= 1;
    }

    fn op_8xy7(&mut self, vx: u16, vy: u16) {
        if (self.regs[vx] < self.regs[vy]) {
            self.regs[0xF] = 1;
        } else {
            self.regs[0xF] = 0;
        }

        self.regs[vx] = self.regs[vy] - self.regs[vx];
    }

    fn op_8xye(&mut self, vx: u16, vy: u16) {
        self.regs[0xF] = (self.regs[vx] & 0x80) >> 7;
        self.regs[vx] <<= 1;
    }

    fn op_9xy0(&mut self, vx: u16, vy: u16) {
        if self.regs[vx] != self.regs[vy] {
            self.sp += 2;
        }
    }

    fn op_annnn(&mut self, opcode: u16) {
        let address = opcode & 0x0FFF;
        self.i = address;
    }

    fn op_bnnnn(&mut self, opcode: u16) {
        let address = opcode & 0x0FFF;
        self.pc = self.regs[0] as u16 + address;
    }

    fn op_cxkk(&mut self, vx: u16, opcode: u16) {
        let byte = opcode & 0x00FF;
        let rand_byte = self.rng.gen::<u16>();

        self.regs[vx] = rand_byte & byte;
    }
}