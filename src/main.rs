#![feature(bigint_helper_methods)]

use crate::c8::Chip8;
use screen::{Screen, ColorScheme};
use std::time::Duration;
use clap::Parser;
use std::path::Path;
use std::time::Instant;
use std::thread;

// Modules
pub mod c8;
pub mod screen;

// CONSTANTS
pub const SCREEN_LINES: usize = 32;
pub const SCREEN_COLUMNS: usize = 64;
pub const TIMER_HZ: u64 = 500;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// The file you want the emulator to execute.
    binary: String,
    /// The color scheme you want to use.
    #[clap(arg_enum, default_value_t=ColorScheme::BlackWhite)]
    color_scheme: ColorScheme
}

fn main() {
  let cli = Cli::parse();
  let binary_path = Path::new(&cli.binary);

  if !binary_path.exists() || !binary_path.is_file() {
     println!("Error: Invalid path.");
     return;
  }

  let mut screen = Screen::new(cli.color_scheme);
  let mut chip8 = Chip8::new();

  chip8.load_file(&binary_path);
  screen.setup();

  let mut counter = 0;
  loop {
    eprint!("\x1b[{};{}H[main] {}", 25, 0, counter);
    let timer = Instant::now();

    // Loop until the terminal screen is 32x64.
    screen.require_screen_size(SCREEN_LINES, SCREEN_COLUMNS);

    // Only poll keypresses every 30 frames
    if counter % 30 == 0 {
      if screen.update_keys() {
        break;
      }
    }
    //eprint!("\x1b[{};{}H[main] {:?}", 23, 0, screen.pressed_keys);

    let previous_display = chip8.get_display().clone();
    chip8.fde_loop(&screen.pressed_keys);
    
    screen.write(&previous_display, chip8.get_display());
   
    let elapsed: Duration = timer.elapsed();
    // Beep if we can't keep up!
    if elapsed >= Duration::from_millis((1000/TIMER_HZ)*2) {
      screen.beep();
    }
    // If we're too fast, sleep for the remaining time.
    if elapsed < Duration::from_millis(1000 / TIMER_HZ) {
      thread::sleep(Duration::from_millis(1000 / TIMER_HZ) - elapsed);
    }
    if chip8.update_timers() {
      screen.beep();
    }
    counter += 1;
  }
}
