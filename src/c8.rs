use std::path::Path;
use std::fs;

use crate::{SCREEN_LINES, SCREEN_COLUMNS};

pub struct Chip8 {
  ram: [u8; 0x1000],     // 4kiB of RAM

                         // Registers
  v: [u8; 0x10],         // V0-VF
  i: u16,                // I, index register
  pc: u16,               // PC, program counter
  sp: u8,                // SP, stack pointer
  dt: u8,                // DT, delay timer
  st: u8,                // ST, sound timer

  stack: [u16; 0x10],
  display: [[bool; SCREEN_COLUMNS]; SCREEN_LINES] // Display is 64px wide by 32px tall
}

impl Chip8 {
  pub fn new() -> Chip8 {
    Chip8 {
      ram: [0x00; 0x1000],
      v: [0x00; 0x10],
      i: 0x0000,
      pc: 0x00,
      sp: 0x00,
      dt: 0x00,
      st: 0x00,
      stack: [0x0000; 0x10],
      display: [[true; SCREEN_COLUMNS]; SCREEN_LINES]
    }
  }

  pub fn fde_loop(&mut self) {
    
  }
  
  pub fn load_file(&mut self, path: &Path) {
    let contents = fs::read(path).unwrap();
    
    for (i, item) in contents.iter().enumerate() {
      self.ram[i+0x200] = *item;
    }
  }

  pub fn getdisplay(&self) -> &[[bool; SCREEN_COLUMNS]; SCREEN_LINES] {
    &self.display
  }
}

