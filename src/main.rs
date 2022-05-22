use crate::c8::Chip8;
use termion::async_stdin;
use std::io::Read;
use std::thread;
use std::time::Duration;

// Modules
pub mod c8;
pub mod screen;

// CONSTANTS
pub const SCREEN_LINES: usize = 32;
pub const SCREEN_COLUMNS: usize = 64;

fn main() {
  let mut stdin = async_stdin().bytes();

  let mut chip8 = Chip8::new();

  screen::setup_screen();


  loop {
    // Loop until the terminal screen is 32x64.
    screen::require_screen_size(SCREEN_LINES, SCREEN_COLUMNS);

    let b = stdin.next();
    match b {
      Some(Ok(b'q')) => break,
      _ => {}
    }

    chip8.fde_loop();

    screen::write_array(chip8.getdisplay());
    
    //TODO: Calculate sleep to make this a 60Hz loop.
    thread::sleep(Duration::from_millis(50));
  }
}
