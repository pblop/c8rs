#![feature(bigint_helper_methods)]

use crate::c8::Chip8;
use screen::Screen;
use std::thread;
use std::time::Duration;
use clap::Parser;
use std::path::Path;
use std::time::Instant;

// Modules
pub mod c8;
pub mod screen;

// CONSTANTS
pub const SCREEN_LINES: usize = 32;
pub const SCREEN_COLUMNS: usize = 64;
pub const HZ: u64 = 60;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// The file you want the emulator to execute.
    binary: String,
}

fn main() {
  let cli = Cli::parse();
  let binary_path = Path::new(&cli.binary);

  if !binary_path.exists() || !binary_path.is_file() {
     println!("Error: Invalid path.");
  }

  let mut screen = Screen::new();
  let mut chip8 = Chip8::new();

  chip8.load_file(&binary_path);
  screen.setup();
  
  loop {
    let timer = Instant::now();

    // Loop until the terminal screen is 32x64.
    screen.require_screen_size(SCREEN_LINES, SCREEN_COLUMNS);

    if screen.update_keys() {
      break;
    }

    let updated_screen = chip8.fde_loop(screen.pressed_keys);
    
    if updated_screen {
      screen.write_array(chip8.get_display());
    }

    match Duration::from_millis(1000/HZ).checked_sub(timer.elapsed()) {
      Some(sleep_duration) => thread::sleep(sleep_duration),
      None => {}
    }
  }
}
