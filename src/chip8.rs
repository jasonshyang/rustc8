/*
Chip-8 specifications:
- 4k memory
- 16 8-bit registers
- 16-bit index register
- 16-bit program counter
- 16 levels of stack
- 8-bit stack pointer
- 8-bit delay timer
- 8-bit sound timer
- 64x32 pixel monochrome display
- 16-key hexadecimal keyboard
*/
const MEMORY_SIZE: usize = 4096;
const REGISTERS_SIZE: usize = 16;
const STACK_SIZE: usize = 16;
pub const DISPLAY_HEIGHT: usize = 32;
pub const DISPLAY_WIDTH: usize = 64;
const DISPLAY_SIZE: usize = DISPLAY_HEIGHT * DISPLAY_WIDTH;
const KEYBOARD_SIZE: usize = 16;

/*
Chip-8 draws graphics on screen through the use of sprites.
A sprite is a group of bytes which are a binary representation of the desired picture.
Chip-8 sprites may be up to 15 bytes, for a possible sprite size of 8x15.
Programs may also refer to a group of sprites representing the hexadecimal digits 0 through F. These sprites are 5 bytes long, or 8x5 pixels.
The data should be stored in the interpreter area of Chip-8 memory (0x000 to 0x1FF).
Each u8 value represents a row of 8 pixels.
For example, the hexadecimal digit 0 is represented by the following sprite:
0xF0  11110000
0x90  10010000
0x90  10010000
0x90  10010000
0xF0  11110000
*/
const CHAR_SPRITES: [u8; 80] = [
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
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

/*
Memory Map:
+---------------+= 0xFFF (4095) End of Chip-8 RAM
|               |
|               |
|               |
|               |
|               |
| 0x200 to 0xFFF|
|     Chip-8    |
| Program / Data|
|     Space     |
|               |
|               |
|               |
+- - - - - - - -+= 0x600 (1536) Start of ETI 660 Chip-8 programs
|               |
|               |
|               |
+---------------+= 0x200 (512) Start of most Chip-8 programs
| 0x000 to 0x1FF|
| Reserved for  |
|  interpreter  |
+---------------+= 0x000 (0) Start of Chip-8 RAM
*/
const MEMORY_START: usize = 0x200;

pub struct Chip8 {
    // Index Register
    // Used to store memory addresses
    pub i: u16,
    // Program Counter
    // Points to the current instruction
    // Incremented by 2 after each instruction as each instruction is 2 bytes long
    pub pc: u16,
    // Memory
    // Stores the program, data (e.g. sprites), and stack
    pub memory: [u8; MEMORY_SIZE],
    // Registers
    // Used for temporary data storage
    // V0 to VE are general purpose registers
    // VF is used as a flag
    pub v: [u8; REGISTERS_SIZE],
    // Stack
    // Store return addresses when subroutines are called
    // When a subroutine is called (CALL addr), the program counter is pushed onto the stack, and the program counter is set to addr
    // When a subroutine returns (RET), the program counter is popped from the stack, and the program counter is set to the popped value
    pub stack: [u16; STACK_SIZE],
    // Stack Pointer
    // Points to the top of the stack
    // Incremented when a value is pushed onto the stack
    // Decremented when a value is popped from the stack
    pub sp: u16,
    // Delay Timer
    // Used for timing events
    // Decremented at a rate of 60Hz
    // When dt is greater than 0, it is decremented by 1 every cycle (1/60 second)
    // When dt is set to 0, it remains at 0
    pub dt: u8,
    // Sound Timer
    // Similar to the delay timer
    // Used for sound effects
    // When st is greater than 0, a beeping sound is made
    pub st: u8,
    // Keyboard Array
    // Represents the state of the Chip-8 hexadecimal keyboard (16 keys 0x0 to 0xF)
    // Instructions that interact with the keyboard will check this array
    pub keyboard: [bool; KEYBOARD_SIZE],
    // Display Array
    // Represents the state of the Chip-8 64x32 pixel display
    // Instructions like DRW will update this array to draw sprites on the display
    pub display: [bool; DISPLAY_SIZE],
    pub is_drawing: bool,
}

impl Chip8 {
    pub fn new() -> Self {
        let mut chip8 = Chip8 {
            i: 0,
            pc: MEMORY_START as u16,
            memory: [0; MEMORY_SIZE],
            v: [0; REGISTERS_SIZE],
            stack: [0; STACK_SIZE],
            sp: 0,
            dt: 0,
            st: 0,
            keyboard: [false; KEYBOARD_SIZE],
            display: [false; DISPLAY_SIZE],
            is_drawing: false,
        };

        // Load the character sprites into memory
        for i in 0..CHAR_SPRITES.len() {
            chip8.memory[i] = CHAR_SPRITES[i];
        }

        chip8
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        // Load the ROM into memory
        for i in 0..rom.len() {
            self.memory[MEMORY_START + i] = rom[i];
        }
    }

    pub fn get_display_data(&self) -> Vec<bool> {
        self.display.to_vec()
    }

    pub fn set_key(&mut self, key: u8) {
        self.keyboard[key as usize] = true;
    }

    pub fn reset_all_keys(&mut self) {
        for i in 0..KEYBOARD_SIZE {
            self.keyboard[i] = false;
        }
    }

    pub fn run_cycle(&mut self) {
        // Fetch the opcode
        let opcode1 = (self.memory[self.pc as usize] as u16) << 8;
        let opcode2 = self.memory[self.pc as usize + 1] as u16;
        let opcode = opcode1 | opcode2;

        // Increment the program counter
        self.pc += 2;

        // Process the opcode
        self.process_opcode(opcode);

        // Update the timers
        self.update_timers();
    }

    fn process_opcode(&mut self, opcode: u16) {
        // Variables to store the values of the opcode
        // x - A 4-bit value, the lower 4 bits of the high byte of the instruction
        // y - A 4-bit value, the upper 4 bits of the low byte of the instruction
        // kk or byte - An 8-bit value, the lowest 8 bits of the instruction
        // nnn or addr - A 12-bit value, the lowest 12 bits of the instruction
        // n / nibble - A 4-bit value, the lowest 4 bits of the instruction
        // Explanation of the bitwise operations:
        // >> 8 performs right shift by 8 bits
        // & 0xF performs bitwise AND with 0x000F, which effectively masks the lower 4 bits
        // for example, if opcode is 0xABCD, then x = ((opcode >> 8) & 0x000F) = ((0xABCD >> 8) & 0x000F) = (0xAB & 0x000F) = 0xB
        let x = ((opcode >> 8) & 0x000F) as usize;
        let y = ((opcode >> 4) & 0x000F) as usize;
        let kk = (opcode & 0x00FF) as u8;
        let nnn = (opcode & 0x0FFF) as u16;
        let nibble = (opcode & 0x000F) as u8;

        // Mask to extract the most significant nibble to determine the type of instruction
        match opcode & 0xF000 {
            0x0000 => {
                match nibble {
                    0x0000 => {
                        // 00E0 - CLS
                        // Clear the display
                        self.display = [false; DISPLAY_SIZE];
                        self.is_drawing = true;
                    }
                    0x000E => {
                        // 00EE - RET
                        // Return from a subroutine
                        self.sp -= 1;
                        self.pc = self.stack[self.sp as usize];
                    }
                    _ => {
                        // 0nnn - SYS addr
                        // Jump to a machine code routine at nnn
                        // This instruction is only used on the old computers on which Chip-8 was originally implemented.
                        self.pc = nnn;
                    }
                }
            }
            0x1000 => {
                // 1nnn - JP addr
                // Jump to location nnn
                self.pc = nnn;
            }
            0x2000 => {
                // 2nnn - CALL addr
                // Call subroutine at nnn
                self.stack[self.sp as usize] = self.pc; // Store the current pc on the stack so that RET can return to it later
                self.sp += 1; // Increment the stack pointer
                self.pc = nnn; // Set the pc to the address of the subroutine so that it is executed next
            }
            0x3000 => {
                // 3xkk - SE Vx, byte
                // Skip next instruction if Vx = kk
                if self.v[x] == kk {
                    self.pc += 2;
                }
            }
            0x4000 => {
                // 4xkk - SNE Vx, byte
                // Skip next instruction if Vx != kk
                if self.v[x] != kk {
                    self.pc += 2;
                }
            }
            0x5000 => {
                // 5xy0 - SE Vx, Vy
                // Skip next instruction if Vx = Vy
                if self.v[x] == self.v[y] {
                    self.pc += 2;
                }
            }
            0x6000 => {
                // 6xkk - LD Vx, byte
                // Set Vx = kk
                self.v[x] = kk;
            }
            0x7000 => {
                // 7xkk - ADD Vx, byte
                // Set Vx = Vx + kk
                self.v[x] = self.v[x].wrapping_add(kk); // Wrapping add to align with expected behavior of Chip-8
            }
            0x8000 => {
                match nibble {
                    0x0000 => {
                        // 8xy0 - LD Vx, Vy
                        // Set Vx = Vy
                        self.v[x] = self.v[y];
                    }
                    0x0001 => {
                        // 8xy1 - OR Vx, Vy
                        // Set Vx = Vx OR Vy
                        self.v[x] |= self.v[y];
                    }
                    0x0002 => {
                        // 8xy2 - AND Vx, Vy
                        // Set Vx = Vx AND Vy
                        self.v[x] &= self.v[y];
                    }
                    0x0003 => {
                        // 8xy3 - XOR Vx, Vy
                        // Set Vx = Vx XOR Vy
                        self.v[x] ^= self.v[y];
                    }
                    0x0004 => {
                        // 8xy4 - ADD Vx, Vy
                        // Set Vx = Vx + Vy, set VF = carry
                        // The values of Vx and Vy are added together.
                        // If the result is greater than 8 bits (i.e., > 0x00FF), VF is set to 1, otherwise 0.
                        // Only the lowest 8 bits of the result are kept, and stored in Vx.
                        if self.v[y] > (0x00FF - self.v[x]) {
                            self.v[0x000F] = 1;
                        } else {
                            self.v[0x000F] = 0;
                        }
                        self.v[x] = self.v[x].wrapping_add(self.v[y]);
                    }
                    0x0005 => {
                        // 8xy5 - SUB Vx, Vy
                        // Set Vx = Vx - Vy, set VF = NOT borrow
                        // If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx.
                        if self.v[x] > self.v[y] {
                            self.v[0x000F] = 1;
                        } else {
                            self.v[0x000F] = 0;
                        }
                        self.v[x] = self.v[x].wrapping_sub(self.v[y]);
                    }
                    0x0006 => {
                        // 8xy6 - SHR Vx {, Vy}
                        // Set Vx = Vx SHR 1
                        // If LSB of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
                        self.v[0x000F] = self.v[x] & 0x1; // v[x] & 0x1 gets LSB
                        self.v[x] >>= 1; // Divide by 2 is equivalent to right shift by 1 as each bit represents a power of 2
                    }
                    0x0007 => {
                        // 8xy7 - SUBN Vx, Vy
                        // Set Vx = Vy - Vx, set VF = NOT borrow
                        // Similar to 8xy5
                        if self.v[y] > self.v[x] {
                            self.v[0x000F] = 1;
                        } else {
                            self.v[0x000F] = 0;
                        }
                        self.v[x] = self.v[y].wrapping_sub(self.v[x]);
                    }
                    0x000E => {
                        // 8xyE - SHL Vx {, Vy}
                        // Set Vx = Vx SHL 1
                        // If MSB of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is multiplied by 2.
                        self.v[0x000F] = (self.v[x] & 0x80) >> 7; // v[x] & 0x80 gets MSB, right shift by 7 to move to LSB
                        self.v[x] <<= 1; // Multiply by 2 is equivalent to left shift by 1 as each bit represents a power of 2
                    }
                    _ => {
                        // Invalid opcode
                        // Panic!
                        panic!("Invalid opcode: {:#X}", opcode);
                    }
                }
            }
            0x9000 => {
                // 9xy0 - SNE Vx, Vy
                // Skip next instruction if Vx != Vy
                if self.v[x] != self.v[y] {
                    self.pc += 2;
                }
            }
            0xA000 => {
                // Annn - LD I, addr
                // Set I = nnn
                self.i = nnn;
            }
            0xB000 => {
                // Bnnn - JP V0, addr
                // Jump to location nnn + V0
                self.pc = nnn + self.v[0] as u16;
            }
            0xC000 => {
                // Cxkk - RND Vx, byte
                // Set Vx = random byte AND kk
                let random: u8 = rand::random();
                self.v[x] = random & kk;
            }
            0xD000 => {
                // Dxyn - DRW Vx, Vy, nibble
                // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision
                // Reads n bytes from memory, starting at the address stored in I.
                // These bytes are then displayed as sprites on screen at coordinates (Vx, Vy).
                // Sprites are XORed onto the existing screen.
                // If this causes any pixels to be erased, VF is set to 1, otherwise it is set to 0.
                // If the sprite is positioned so part of it is outside the coordinates of the display, it wraps around to the opposite side of the screen.
                let size = nibble as usize;
                let x = self.v[x] as usize;
                let y = self.v[y] as usize;

                self.v[0x000F] = 0; // Reset collision flag

                for line in 0..size {
                    // Loop through each line of the sprite to draw in display
                    let buffer = self.memory[self.i as usize + line]; // Read each byte of the sprite from memory, representing a line of 8 pixels
                    for pixel in 0..8 {
                        // Loop through each pixel in the line
                        if (buffer & (0x80 >> pixel)) != 0 {
                            // Check if the pixel is set
                            // Calculate the index of the pixel in the display array
                            // x is the starting x coord, pixel is the current pixel in the line, (x + pixel) % DISPLAY_WIDTH wraps around the display
                            // y is the starting y coord, line is the current line, (y + line) % DISPLAY_HEIGHT wraps around the display
                            let i = (x + pixel) % DISPLAY_WIDTH
                                + ((y + line) % DISPLAY_HEIGHT) * DISPLAY_WIDTH;
                            if self.display[i] {
                                self.v[0x000F] = 1; // Collision detected
                            }
                            self.display[i] ^= true; // XOR the pixel value
                        }
                    }
                }
                self.is_drawing = true;
            }
            0xE000 => {
                match kk {
                    0x009E => {
                        // Ex9E - SKP Vx
                        // Skip next instruction if key with the value of Vx is pressed
                        if self.keyboard[self.v[x] as usize] {
                            self.pc += 2;
                        }
                    }
                    0x00A1 => {
                        // ExA1 - SKNP Vx
                        // Skip next instruction if key with the value of Vx is not pressed
                        if !self.keyboard[self.v[x] as usize] {
                            self.pc += 2;
                        } else {
                            self.reset_all_keys();
                        }
                    }
                    _ => {
                        // Invalid opcode
                        // Panic!
                        panic!("Invalid opcode: {:#X}", opcode);
                    }
                }
            }
            0xF000 => {
                match kk {
                    0x0007 => {
                        // Fx07 - LD Vx, DT
                        // Set Vx = delay timer value
                        self.v[x] = self.dt;
                    }
                    0x000A => {
                        // Fx0A - LD Vx, K
                        // Wait for a key press, store the value of the key in Vx
                        // This is a blocking operation, this is implemented by moving the pc back by 2 if no key is pressed
                        let mut is_blocking = true;

                        for i in 0..KEYBOARD_SIZE {
                            if self.keyboard[i] {
                                self.v[x] = i as u8;
                                is_blocking = false;
                                break;
                            }
                        }

                        if is_blocking {
                            self.pc -= 2;
                        } else {
                            self.reset_all_keys();
                        }
                    }
                    0x0015 => {
                        // Fx15 - LD DT, Vx
                        // Set delay timer = Vx
                        self.dt = self.v[x];
                    }
                    0x0018 => {
                        // Fx18 - LD ST, Vx
                        // Set sound timer = Vx
                        self.st = self.v[x];
                    }
                    0x001E => {
                        // Fx1E - ADD I, Vx
                        // Set I = I + Vx
                        self.i += self.v[x] as u16;
                    }
                    0x0029 => {
                        // Fx29 - LD F, Vx
                        // Set I = location of sprite for digit Vx
                        self.i = self.v[x] as u16 * 5; // * 5 because each sprite is 5 bytes long
                    }
                    0x0033 => {
                        // Fx33 - LD B, Vx
                        // Store Binary-Coded Decimal (BCD) representation of Vx in memory locations I, I+1, and I+2
                        self.memory[self.i as usize] = self.v[x] / 100; // Hundreds digit, x is u8 so no need to mask
                        self.memory[self.i as usize + 1] = (self.v[x] / 10) % 10; // Tens digit
                        self.memory[self.i as usize + 2] = self.v[x] % 10; // Ones digit
                    }
                    0x0055 => {
                        // Fx55 - LD [I], Vx
                        // Store registers V0 through Vx in memory starting at location I
                        for i in 0..=x {
                            self.memory[self.i as usize + i] = self.v[i];
                        }
                    }
                    0x0065 => {
                        // Fx65 - LD Vx, [I]
                        // Read registers V0 through Vx from memory starting at location I
                        for i in 0..=x {
                            self.v[i] = self.memory[self.i as usize + i];
                        }
                    }
                    _ => {
                        // Invalid opcode
                        // Panic!
                        panic!("Invalid opcode: {:#X}", opcode);
                    }
                }
            }
            _ => {
                // Invalid opcode
                // Panic!
                panic!("Invalid opcode: {:#X}", opcode);
            }
        }
    }

    fn update_timers(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }

        if self.st > 0 {
            self.st -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let chip8 = Chip8::new();
        assert_eq!(chip8.i, 0);
        assert_eq!(chip8.pc, MEMORY_START as u16);
        assert_eq!(chip8.memory.len(), MEMORY_SIZE);
        assert_eq!(chip8.v, [0; REGISTERS_SIZE]);
        assert_eq!(chip8.stack, [0; STACK_SIZE]);
        assert_eq!(chip8.sp, 0);
        assert_eq!(chip8.dt, 0);
        assert_eq!(chip8.st, 0);
        assert_eq!(chip8.keyboard, [false; KEYBOARD_SIZE]);
        assert_eq!(chip8.display, [false; DISPLAY_SIZE]);
    }

    #[test]
    fn test_load_rom() {
        let mut chip8 = Chip8::new();
        let rom = vec![0x00, 0xE0, 0x00, 0xEE];
        chip8.load_rom(&rom);
        assert_eq!(chip8.memory[MEMORY_START], 0x00);
        assert_eq!(chip8.memory[MEMORY_START + 1], 0xE0);
        assert_eq!(chip8.memory[MEMORY_START + 2], 0x00);
        assert_eq!(chip8.memory[MEMORY_START + 3], 0xEE);
    }

    #[test]
    fn test_run_cycle() {
        let mut chip8 = Chip8::new();
        // test opcode 0x00E0
        chip8.display = [true; DISPLAY_SIZE];
        chip8.memory[MEMORY_START] = 0x00;
        chip8.memory[MEMORY_START + 1] = 0xE0;

        chip8.run_cycle();
        assert_eq!(chip8.display, [false; DISPLAY_SIZE]);
    }

    #[test]
    fn test_process_opcode() {
        let mut chip8 = Chip8::new();

        // 0x00E0 - CLS
        // Clear the display
        chip8.display = [true; DISPLAY_SIZE];
        chip8.process_opcode(0x00E0);
        assert_eq!(chip8.display, [false; DISPLAY_SIZE]);

        // 0x00EE - RET
        // Return from a subroutine
        chip8.sp = 1;
        chip8.stack[0] = 0x0200;
        chip8.process_opcode(0x00EE);

        assert_eq!(chip8.pc, 0x0200);
        assert_eq!(chip8.sp, 0);

        // 0x1nnn - JP addr
        // Jump to location nnn
        chip8.process_opcode(0x1200);
        assert_eq!(chip8.pc, 0x0200);

        // 0x2nnn - CALL addr
        // Call subroutine at nnn
        chip8.process_opcode(0x2200);
        assert_eq!(chip8.pc, 0x0200);
        assert_eq!(chip8.stack[0], 0x0200);
        assert_eq!(chip8.sp, 1);

        // 0x3xkk - SE Vx, byte
        // Skip next instruction if Vx = kk
        chip8.v[0] = 0x01;
        chip8.process_opcode(0x3001);
        assert_eq!(chip8.pc, 0x0202);

        // 0x4xkk - SNE Vx, byte
        // Skip next instruction if Vx != kk
        chip8.v[0] = 0x01;
        chip8.process_opcode(0x4002);
        assert_eq!(chip8.pc, 0x0204);

        // 0x5xy0 - SE Vx, Vy
        // Skip next instruction if Vx = Vy
        chip8.v[0] = 0x01;
        chip8.v[1] = 0x01;
        chip8.process_opcode(0x5010);
        assert_eq!(chip8.pc, 0x0206);

        // reset pc
        chip8.pc = MEMORY_START as u16;

        // 0x6xkk - LD Vx, byte
        // Set Vx = kk
        chip8.process_opcode(0x6001);
        assert_eq!(chip8.v[0], 0x01);

        // 0x7xkk - ADD Vx, byte
        // Set Vx = Vx + kk
        chip8.v[0] = 0x01;
        chip8.process_opcode(0x7001);
        assert_eq!(chip8.v[0], 0x02);

        // 0x8xy0 - LD Vx, Vy
        // Set Vx = Vy
        chip8.v[0] = 0x01;
        chip8.v[1] = 0x02;
        chip8.process_opcode(0x8010);
        assert_eq!(chip8.v[0], 0x02);

        // 0x8xy1 - OR Vx, Vy
        // Set Vx = Vx OR Vy
        chip8.v[0] = 0x01;
        chip8.v[1] = 0x02;
        chip8.process_opcode(0x8011);
        assert_eq!(chip8.v[0], 0x03);

        // 0x8xy2 - AND Vx, Vy
        // Set Vx = Vx AND Vy
        chip8.v[0] = 0x01;
        chip8.v[1] = 0x02;
        chip8.process_opcode(0x8012);
        assert_eq!(chip8.v[0], 0x00);

        // 0x8xy3 - XOR Vx, Vy
        // Set Vx = Vx XOR Vy
        chip8.v[0] = 0x01;
        chip8.v[1] = 0x02;
        chip8.process_opcode(0x8013);
        assert_eq!(chip8.v[0], 0x03);

        // 0x8xy4 - ADD Vx, Vy
        // Set Vx = Vx + Vy, set VF = carry
        chip8.v[0] = 0xFF;
        chip8.v[1] = 0x01;
        chip8.process_opcode(0x8014);
        assert_eq!(chip8.v[0], 0x00);
        assert_eq!(chip8.v[0x000F], 1);

        // 0x8xy5 - SUB Vx, Vy
        // Set Vx = Vx - Vy, set VF = NOT borrow
        chip8.v[0] = 0x02;
        chip8.v[1] = 0x01;
        chip8.process_opcode(0x8015);
        assert_eq!(chip8.v[0], 0x01);
        assert_eq!(chip8.v[0x000F], 1);

        // 0x8xy6 - SHR Vx {, Vy}
        // Set Vx = Vx SHR 1
        chip8.v[0] = 0x03;
        chip8.process_opcode(0x8006);
        assert_eq!(chip8.v[0], 0x01);
        assert_eq!(chip8.v[0x000F], 1);

        // 0x8xy7 - SUBN Vx, Vy
        // Set Vx = Vy - Vx, set VF = NOT borrow
        chip8.v[0] = 0x01;
        chip8.v[1] = 0x02;
        chip8.process_opcode(0x8017);
        assert_eq!(chip8.v[0], 0x01);
        assert_eq!(chip8.v[0x000F], 1);

        // 0x8xyE - SHL Vx {, Vy}
        // Set Vx = Vx SHL 1
        chip8.v[0] = 0x01;
        chip8.process_opcode(0x800E);
        assert_eq!(chip8.v[0], 0x02);
        assert_eq!(chip8.v[0x000F], 0);

        // reset pc
        chip8.pc = MEMORY_START as u16;

        // 0x9xy0 - SNE Vx, Vy
        // Skip next instruction if Vx != Vy
        chip8.v[0] = 0x01;
        chip8.v[1] = 0x02;
        chip8.process_opcode(0x9010);
        assert_eq!(chip8.pc, 0x0202);

        // 0xAnnn - LD I, addr
        // Set I = nnn
        chip8.process_opcode(0xA123);
        assert_eq!(chip8.i, 0x0123);

        // reset pc
        chip8.pc = MEMORY_START as u16;

        // 0xBnnn - JP V0, addr
        // Jump to location nnn + V0
        chip8.v[0] = 0x01;
        chip8.process_opcode(0xB123);
        assert_eq!(chip8.pc, 0x0124);

        // reset pc
        chip8.pc = MEMORY_START as u16;

        // 0xCxkk - RND Vx, byte
        // Set Vx = random byte AND kk
        let old_vx = chip8.v[0];
        chip8.process_opcode(0xC0FF);
        assert_ne!(chip8.v[0], old_vx);

        // 0xDxyn - DRW Vx, Vy, nibble
        chip8.i = 0x0;
        chip8.v[0] = 0x0;
        chip8.v[1] = 0x1;
        chip8.process_opcode(0xD015);

        // Row 0 (y = 1)
        assert_eq!(chip8.display[0 + 1 * DISPLAY_WIDTH], true);
        assert_eq!(chip8.display[1 + 1 * DISPLAY_WIDTH], true);
        assert_eq!(chip8.display[2 + 1 * DISPLAY_WIDTH], true);
        assert_eq!(chip8.display[3 + 1 * DISPLAY_WIDTH], true);

        // Row 1 (y = 2)
        assert_eq!(chip8.display[0 + 2 * DISPLAY_WIDTH], true);
        assert_eq!(chip8.display[3 + 2 * DISPLAY_WIDTH], true);

        // Row 2 (y = 3)
        assert_eq!(chip8.display[0 + 3 * DISPLAY_WIDTH], true);
        assert_eq!(chip8.display[3 + 3 * DISPLAY_WIDTH], true);

        // Row 3 (y = 4)
        assert_eq!(chip8.display[0 + 4 * DISPLAY_WIDTH], true);
        assert_eq!(chip8.display[3 + 4 * DISPLAY_WIDTH], true);

        // Row 4 (y = 5)
        assert_eq!(chip8.display[0 + 5 * DISPLAY_WIDTH], true);
        assert_eq!(chip8.display[1 + 5 * DISPLAY_WIDTH], true);
        assert_eq!(chip8.display[2 + 5 * DISPLAY_WIDTH], true);
        assert_eq!(chip8.display[3 + 5 * DISPLAY_WIDTH], true);

        assert_eq!(chip8.v[0x000F], 0); // No collision detected

        chip8.process_opcode(0xD015); // Draw the same sprite again
        assert_eq!(chip8.v[0x000F], 1); // Collision detected

        // reset pc
        chip8.pc = MEMORY_START as u16;

        // 0xEx9E - SKP Vx
        // Skip next instruction if key with the value of Vx is pressed
        chip8.keyboard[0] = true;
        chip8.v[0] = 0x00;
        chip8.process_opcode(0xE09E);
        assert_eq!(chip8.pc, 0x0202);

        // 0xExA1 - SKNP Vx
        // Skip next instruction if key with the value of Vx is not pressed
        chip8.keyboard[0] = false;
        chip8.v[0] = 0x00;
        chip8.process_opcode(0xE0A1);
        assert_eq!(chip8.pc, 0x0204);

        // reset pc
        chip8.pc = MEMORY_START as u16;

        // 0xFx07 - LD Vx, DT
        // Set Vx = delay timer value
        chip8.dt = 0x01;
        chip8.process_opcode(0xF007);
        assert_eq!(chip8.v[0], 0x01);

        // 0xFx0A - LD Vx, K
        // Wait for a key press, store the value of the key in Vx
        chip8.keyboard[0] = false;
        chip8.process_opcode(0xF00A);
        assert_eq!(chip8.pc, 0x01FE); // pc should be decremented by 2 as this is a blocking operation

        chip8.keyboard[0] = true;
        chip8.process_opcode(0xF00A);
        assert_eq!(chip8.v[0], 0x00);

        // reset pc
        chip8.pc = MEMORY_START as u16;

        // 0xFx15 - LD DT, Vx
        // Set delay timer = Vx
        chip8.v[0] = 0x01;
        chip8.process_opcode(0xF015);
        assert_eq!(chip8.dt, 0x01);

        // 0xFx18 - LD ST, Vx
        // Set sound timer = Vx
        chip8.v[0] = 0x01;
        chip8.process_opcode(0xF018);
        assert_eq!(chip8.st, 0x01);

        // 0xFx1E - ADD I, Vx
        // Set I = I + Vx
        chip8.i = 0x01;
        chip8.v[0] = 0x01;
        chip8.process_opcode(0xF01E);
        assert_eq!(chip8.i, 0x02);

        // 0xFx29 - LD F, Vx
        // Set I = location of sprite for digit Vx
        chip8.v[0] = 0x01;
        chip8.process_opcode(0xF029);
        assert_eq!(chip8.i, 0x05);

        // 0xFx33 - LD B, Vx
        // Store Binary-Coded Decimal (BCD) representation of Vx in memory locations I, I+1, and I+2
        chip8.i = 0x00;
        chip8.v[0] = 123;
        chip8.process_opcode(0xF033);
        assert_eq!(chip8.memory[0], 1);
        assert_eq!(chip8.memory[1], 2);
        assert_eq!(chip8.memory[2], 3);

        // 0xFx55 - LD [I], Vx
        // Store registers V0 through Vx in memory starting at location I
        chip8.i = 0x00;
        chip8.v[0] = 0x01;
        chip8.v[1] = 0x02;
        chip8.process_opcode(0xF155);
        assert_eq!(chip8.memory[0], 0x01);
        assert_eq!(chip8.memory[1], 0x02);

        // 0xFx65 - LD Vx, [I]
        // Read registers V0 through Vx from memory starting at location I
        chip8.i = 0x00;
        chip8.memory[0] = 0x01;
        chip8.memory[1] = 0x02;
        chip8.v[0] = 0x00;
        chip8.v[1] = 0x00;
        chip8.process_opcode(0xF165);
        assert_eq!(chip8.v[0], 0x01);
        assert_eq!(chip8.v[1], 0x02);
    }
}
