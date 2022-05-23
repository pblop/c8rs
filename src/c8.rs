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

  stack: [u16; 0x10],
  display: [[bool; SCREEN_COLUMNS]; SCREEN_LINES] // Display is 64px wide by 32px tall
}

impl Chip8 {
  pub fn new() -> Chip8 {
    let mut m_c8 = Chip8 {
      ram: [0x00; 0x1000],
      v: [0x00; 0x10],
      i: 0x0000,
      pc: 0x00,
      sp: 0x00,
      dt: 0x00,
      st: 0x00,
      stack: [0x0000; 0x10],
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

