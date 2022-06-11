use std::path::Path;
use std::fs;

use crate::{SCREEN_LINES, SCREEN_COLUMNS};

const FONT: [u8; 5*16] = [
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

pub struct Chip8 {
  ram: [u8; 0x1000],     // 4kiB of RAM

                         // Registers
  v: [u8; 0x10],         // V0-VF
  i: u16,                // I, index register
  pc: u16,               // PC, program counter
  sp: u8,                // SP, stack pointer
  dt: u8,                // DT, delay timer
  st: u8,                // ST, sound timer

  stack: [u16; 0x100],   // 256-word deep stack
  display: [[bool; SCREEN_COLUMNS]; SCREEN_LINES] // Display is 64px wide by 32px tall
}

impl Chip8 {
  pub fn new() -> Chip8 {
    let mut m_c8 = Chip8 {
      ram: [0x00; 0x1000],
      v: [0x00; 0x10],
      i:  0x0000,
      pc: 0x0200,
      sp: 0x00,
      dt: 0x00,
      st: 0x00,
      stack: [0x0000; 0x100],
      display: [[true; SCREEN_COLUMNS]; SCREEN_LINES]
    };

    m_c8.load_font();

    m_c8
  }

  pub fn load_font(&mut self) {
    // I load the font at address 0x000.
    // TODO: Maybe it's better to load it at 0x050.
    for (i, item) in FONT.iter().enumerate() {
      self.ram[i] = *item;
    }
  }

  // In this stack, the SP points to the next, unfilled,
  // position in the stack.
  fn st_push(&mut self, value: u16) {
    self.stack[self.sp as usize] += value;
    // This bottom line will overflow and error out when
    // the result of this operation is greater then 0x100.
    self.sp += 1;
  }
  fn st_pop(&mut self) -> u16 {
    self.sp -= 1;
    self.stack[self.sp as usize]
  }

  pub fn fde_loop(&mut self, pressed_keys: [bool; 16]) -> bool {
    let mut display_updated = false;

    // =======      Fetch       =======
    let instruction_bytes = &self.ram[(self.pc as usize)..((self.pc+2) as usize)];
    let instruction = (instruction_bytes[0] as u16) << 8 | instruction_bytes[1] as u16;

    self.pc += 2;
    
    // ======= Decode & Execute =======
    match instruction & 0xf000 {
      0x0000 => {
        match instruction {
          0x00E0 => {     // 00E0: CLS
            self.clear_display();
          },
          0x00EE => {     // 00EE: RET
            self.pc = self.st_pop();
          },
          _ => {          // 0nnn: SYS addr
            // TODO. This instruction is not very important.
          }
        }
      },
      0x1000 => {         // 1nnn: JP addr
        self.pc = instruction & 0x0fff;
      },
      0x2000 => {         // 2nnn: CALL addr
        self.st_push(self.pc);
        self.pc = instruction & 0x0fff;
      },
      0x3000 => {         // 3xkk:  SE Vx, byte
        let x: usize = ((instruction & 0x0f00) >> 8) as usize;
        let kk: u8 = (instruction & 0x00ff) as u8;
        
        if self.v[x] == kk {
          self.pc += 2;
        }
      },
      0x4000 => {         // 3xkk:  SNE Vx, byte
        let x: usize = ((instruction & 0x0f00) >> 8) as usize;
        let kk: u8 = (instruction & 0x00ff) as u8;
        
        if self.v[x] != kk {
          self.pc += 2;
        }
      },
      0x5000 => {
                          // 5xy0: SE Vx, Vy
        if instruction & 0x000f == 0x0000 {
          let x: usize = ((instruction & 0x0f00) >> 8) as usize;
          let y: usize = ((instruction & 0x00f0) >> 4) as usize;

          if self.v[x] == self.v[y] {
            self.pc += 2;
          }
        } else {
          // Unknown instruction
        }
      },
      0x6000 => {         // 6xkk: LD Vx, byte
        let x: usize = ((instruction & 0x0f00) >> 8) as usize;
        let kk: u8 = (instruction & 0x00ff) as u8;

        self.v[x] = kk;
      },
      0x7000 => {         // 7xkk: ADD Vx, byte
        // NOTE: This ADD instruction DOES NOT affect the carry bit in VF.
        let x: usize = ((instruction & 0x0f00) >> 8) as usize;
        let kk: u8 = (instruction & 0x00ff) as u8;

        self.v[x] = self.v[x].wrapping_add(kk);
      },
      0x8000 => {
        let x: usize = ((instruction & 0x0f00) >> 8) as usize;
        let y: usize = ((instruction & 0x00f0) >> 4) as usize;

        match instruction & 0x000f {
          0x0000 => {     // 8xy0: LD Vx, Vy
            self.v[x] = self.v[y];
          },
          0x0001 => {     // 8xy1: OR Vx, Vy
            self.v[x] |= self.v[y];
          },
          0x0002 => {     // 8xy2: AND Vx, Vy
            self.v[x] &= self.v[y];
          },
          0x0003 => {     // 8xy3: XOR Vx, Vy
            self.v[x] ^= self.v[y];
          },
          0x0004 => {     // 8xy4: ADD Vx, Vy
            // NOTE: This ADD instruction DOES affect the carry bit in VF.
            
            let carry: bool;
            (self.v[x], carry) = self.v[x].carrying_add(self.v[y], false);
            self.v[0xf] = carry as u8;
          },
          0x0005 => {     // 8xy5: SUB Vx, Vy
            let borrow: bool;
    
            (self.v[x], borrow) = self.v[x].borrowing_sub(self.v[y], false);

            self.v[0xf] = !borrow as u8;
          },
          0x0006 => {     // 8xy6: SHR Vx, Vy
            // Quirky: We set Vx = Vy.
            self.v[x] = self.v[y];

            self.v[0xf] = self.v[x] & 0x0001;
            self.v[x] >>= 1;
          },
          0x0007 => {     // 8xy7: SUBN Vx, Vy
            let borrow: bool;
    
            (self.v[y], borrow) = self.v[y].borrowing_sub(self.v[x], false);

            self.v[0xf] = !borrow as u8;
          },
          0x000E => {     // 8xyE: SHL Vx, Vy
            // Quirky: We set Vx = Vy.
            self.v[x] = self.v[y];

            self.v[0xf] = (self.v[x] & 0x80) >> 7;
            self.v[x] <<= 1;
          },
          _ => {} // Only these instructions exist, if we reach this point, there's a problem somewhere.
        }
      },
      0x9000 => {
                  // 9xy0: SNE Vx, Vy
        if instruction & 0x000f == 0x0000 {
          let x: usize = ((instruction & 0x0f00) >> 8) as usize;
          let y: usize = ((instruction & 0x00f0) >> 4) as usize;

          if self.v[x] != self.v[y] {
            self.pc += 2;
          }
        } else {
          // Unknown instruction
        }

      },
      0xA000 => {          // Annn: LD I, addr
        self.i = instruction & 0x0fff;
      },
      0xB000 => {          // Bnnn: JP V0, addr
        // Quirky, jumps to nnn + V0
        self.pc = instruction & 0x0fff + (self.v[0x0] as u16);
      },
      0xC000 => {          // Cxkk: RND Vx, byte
      },
      0xD000 => {          // Dxyn: DRW Vx, Vy, nibble
        // I'm using _x and _y here as to differentiate these two variables from
        // the x and y values I'm grabbing from these registers.
        let _x: usize = ((instruction & 0x0f00) >> 8) as usize;
        let _y: usize = ((instruction & 0x00f0) >> 4) as usize;
        let nibble = instruction & 0x000f;

        // Grab the x and y coordinates from Vx and Vy.
        let x = self.v[_x] % (SCREEN_COLUMNS as u8);
        let y = self.v[_y] % (SCREEN_LINES as u8);
        
        self.v[0xf] = 0;
        
        let sprite_rowdata = &self.ram[(self.i as usize) .. ((self.i+nibble) as usize)]; 
        for i in 0..(nibble as u8) {  // For i in each row.
          let rowdata = sprite_rowdata[i as usize];
          let new_y = (y+i) as usize;
         
          // If we're over the border of the screen, stop drawing.
          if new_y >= SCREEN_LINES {
            break;
          }

          for j in 0..8 {     // For each bit (1 bit per column).
            let new_x = (x+j) as usize;

            // If we're over the border of the screen, stop drawing
            if new_x >= SCREEN_COLUMNS {
              break;
            }
            
            // We write bits from the left of the byte to the right of the byte.
            // Thus, we first need to mask 0b10000000 (0x80), then 0b01000000, ...
            let mask_bit = (rowdata & (0x80 >> j)) != 0;
            if self.display[new_y][new_x] == true && mask_bit == true {
              self.v[0xf] = 1;
            }

            self.display[new_y][new_x] ^= mask_bit;
          }
        }

        // TODO: This could be optimised some more by checking if we're actually updating
        // the screen before setting this to true.
        display_updated = true;
      },
      0xE000 => {
        let x: usize = ((instruction & 0x0f00) >> 8) as usize;
        match instruction & 0x00ff {
          0x009E => {      // Ex9E: SKP Vx
            if pressed_keys[self.v[x] as usize] {
              self.pc += 2;
            }
          },
          0x00A1 => {      // ExA1: SKNP Vx
            if !pressed_keys[self.v[x] as usize] {
              self.pc += 2;
            }
          },
          // Unknown instruction.
          _ => {}
        }
      },
      0xF000 => {
        let x: usize = ((instruction & 0x0f00) >> 8) as usize;
        match instruction & 0x00ff {
          0x0007 => {      // Fx07: LD Vx, DT
            self.v[x] = self.dt;
          },
          0x000A => {      // Fx0A: LD Vx, K
            
          },
          0x0015 => {      // Fx15: LD DT, Vx
            self.dt = self.v[x];
          },
          0x0018 => {      // Fx18: LD ST, Vx
            self.st = self.v[x];
          },
          0x000A => {      // Fx0A: LD Vx, K
            
          },
          // Unknown instruction.
          _ => {}
        }

      },

      // This shouldn't be executed, I've tested for all the possible combinations above.
      _ => {}
    }

    display_updated
  }
  
  pub fn load_file(&mut self, path: &Path) {
    let contents = fs::read(path).unwrap();
    
    for (i, item) in contents.iter().enumerate() {
      self.ram[i+0x200] = *item;
    }
  }

  pub fn get_display(&self) -> &[[bool; SCREEN_COLUMNS]; SCREEN_LINES] {
    &self.display
  }

  pub fn clear_display(&mut self) {
    for i in 0..self.display.len() {
      for j in 0..self.display[0].len() {
        self.display[i][j] = false;
      }
    }
  }
}

