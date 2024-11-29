#![allow(dead_code)]
#![allow(unused_variables)]

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
const DISPLAY_SIZE: usize = 64 * 32;
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

pub struct Chip8 {
    // Index Register
    pub i: u16,
    // Program Counter
    pub pc: u16,
    // Memory
    pub memory: [u8; MEMORY_SIZE],
    // Registers
    pub v: [u8; REGISTERS_SIZE],
    // Stack
    pub stack: [u16; STACK_SIZE],
    // Stack Pointer
    pub sp: u16,
    // Delay Timer
    pub dt: u8,
    // Sound Timer
    pub st: u8,
    // Keyboard Array
    pub keyboard: [bool; KEYBOARD_SIZE],
    // Display Array
    pub display: [bool; DISPLAY_SIZE],
}

impl Chip8 {
    pub fn new() -> Self {
        let mut chip8 = Chip8 {
            i: 0,
            pc: 0,
            memory: [0; MEMORY_SIZE],
            v: [0; REGISTERS_SIZE],
            stack: [0; STACK_SIZE],
            sp: 0,
            dt: 0,
            st: 0,
            keyboard: [false; KEYBOARD_SIZE],
            display: [false; DISPLAY_SIZE],
        };

        // Load the character sprites into memory
        for i in 0..CHAR_SPRITES.len() {
            chip8.memory[i] = CHAR_SPRITES[i];
        }

        chip8
    }

    pub fn load_rom() {
        // Load the ROM into memory
    }

    pub fn run_cycle() {
        // Fetch opcode from memory at location pc
        // Process opcode
        // Update timers
    }

    pub fn process_opcode(&mut self, opcode: u16) {
        // Variables to store the values of the opcode
        // x - A 4-bit value, the lower 4 bits of the high byte of the instruction
        // y - A 4-bit value, the upper 4 bits of the low byte of the instruction
        // kk or byte - An 8-bit value, the lowest 8 bits of the instruction
        // nnn or addr - A 12-bit value, the lowest 12 bits of the instruction
        // nn - An 8-bit value, the lowest 8 bits of the instruction
        // n / nibble - A 4-bit value, the lowest 4 bits of the instruction
        // Explanation of the bitwise operations:
        // >> 8 performs right shift by 8 bits
        // & 0xF performs bitwise AND with 0x000F, which effectively masks the lower 4 bits
        // for example, if opcode is 0xABCD, then x = ((opcode >> 8) & 0x000F) = ((0xABCD >> 8) & 0x000F) = (0xAB & 0x000F) = 0xB
        let x = ((opcode >> 8) & 0x000F) as usize;
        let y = ((opcode >> 4) & 0x000F) as usize;
        let kk = (opcode & 0x00FF) as u8;
        let nnn = (opcode & 0x0FFF) as u16;
        let nn = (opcode & 0x00FF) as u8;
        let nibble = (opcode & 0x000F) as u8;

        // Mask to extract the most significant nibble to determine the type of instruction
        match opcode & 0xF000 {
            0x0000 => {
                match nibble {
                    0x0000 => {
                        // 00E0 - CLS
                        // Clear the display
                    }
                    0x000E => {
                        // 00EE - RET
                        // Return from a subroutine
                    }
                    _ => {
                        // 0nnn - SYS addr
                        // Jump to a machine code routine at nnn
                    }
                }
            },
            0x1000 => {
                // 1nnn - JP addr
                // Jump to location nnn
            },
            0x2000 => {
                // 2nnn - CALL addr
                // Call subroutine at nnn
            },
            0x3000 => {
                // 3xkk - SE Vx, byte
                // Skip next instruction if Vx = kk
            },
            0x4000 => {
                // 4xkk - SNE Vx, byte
                // Skip next instruction if Vx != kk
            },
            0x5000 => {
                // 5xy0 - SE Vx, Vy
                // Skip next instruction if Vx = Vy
            },
            0x6000 => {
                // 6xkk - LD Vx, byte
                // Set Vx = kk
            },
            0x7000 => {
                // 7xkk - ADD Vx, byte
                // Set Vx = Vx + kk
            },
            0x8000 => {
                match nibble {
                    0x0000 => {
                        // 8xy0 - LD Vx, Vy
                        // Set Vx = Vy
                    },
                    0x0001 => {
                        // 8xy1 - OR Vx, Vy
                        // Set Vx = Vx OR Vy
                    },
                    0x0002 => {
                        // 8xy2 - AND Vx, Vy
                        // Set Vx = Vx AND Vy
                    },
                    0x0003 => {
                        // 8xy3 - XOR Vx, Vy
                        // Set Vx = Vx XOR Vy
                    },
                    0x0004 => {
                        // 8xy4 - ADD Vx, Vy
                        // Set Vx = Vx + Vy, set VF = carry
                    },
                    0x0005 => {
                        // 8xy5 - SUB Vx, Vy
                        // Set Vx = Vx - Vy, set VF = NOT borrow
                    },
                    0x0006 => {
                        // 8xy6 - SHR Vx {, Vy}
                        // Set Vx = Vx SHR 1
                    },
                    0x0007 => {
                        // 8xy7 - SUBN Vx, Vy
                        // Set Vx = Vy - Vx, set VF = NOT borrow
                    },
                    0x000E => {
                        // 8xyE - SHL Vx {, Vy}
                        // Set Vx = Vx SHL 1
                    },
                    _ => {
                        // Invalid opcode
                        // Panic!
                    }
                }
            }
            0x9000 => {
                // 9xy0 - SNE Vx, Vy
                // Skip next instruction if Vx != Vy
            },
            0xA000 => {
                // Annn - LD I, addr
                // Set I = nnn
            },
            0xB000 => {
                // Bnnn - JP V0, addr
                // Jump to location nnn + V0
            },
            0xC000 => {
                // Cxkk - RND Vx, byte
                // Set Vx = random byte AND kk
            },
            0xD000 => {
                // Dxyn - DRW Vx, Vy, nibble
                // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision
            },
            0xE000 => {
                match nn {
                    0x009E => {
                        // Ex9E - SKP Vx
                        // Skip next instruction if key with the value of Vx is pressed
                    },
                    0x00A1 => {
                        // ExA1 - SKNP Vx
                        // Skip next instruction if key with the value of Vx is not pressed
                    },
                    _ => {
                        // Invalid opcode
                        // Panic!
                    }
                }
            },
            0xF000 => {
                match nn {
                    0x0007 => {
                        // Fx07 - LD Vx, DT
                        // Set Vx = delay timer value
                    },
                    0x000A => {
                        // Fx0A - LD Vx, K
                        // Wait for a key press, store the value of the key in Vx
                    },
                    0x0015 => {
                        // Fx15 - LD DT, Vx
                        // Set delay timer = Vx
                    },
                    0x0018 => {
                        // Fx18 - LD ST, Vx
                        // Set sound timer = Vx
                    },
                    0x001E => {
                        // Fx1E - ADD I, Vx
                        // Set I = I + Vx
                    },
                    0x0029 => {
                        // Fx29 - LD F, Vx
                        // Set I = location of sprite for digit Vx
                    },
                    0x0033 => {
                        // Fx33 - LD B, Vx
                        // Store BCD representation of Vx in memory locations I, I+1, and I+2
                    },
                    0x0055 => {
                        // Fx55 - LD [I], Vx
                        // Store registers V0 through Vx in memory starting at location I
                    },
                    0x0065 => {
                        // Fx65 - LD Vx, [I]
                        // Read registers V0 through Vx from memory starting at location I
                    },
                    _ => {
                        // Invalid opcode
                        // Panic!
                    }
                }
            },
            _ => {
                // Invalid opcode
                // Panic!
            }
        }


    }

    pub fn update_timers() {
        // Update delay timer
        // Update sound timer
    }

    pub fn draw(&mut self, x: u8, y: u8, height: u8) {
        // Draw a sprite at position (x, y) with height n
        // Set VF to 1 if any set pixels are changed to unset, and 0 otherwise
    }

}