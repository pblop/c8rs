use crate::c8::Chip8;
use termion::async_stdin;
use std::io::Read;
use std::thread;
use std::time::Duration;
use clap::Parser;
use std::path::Path;

// Modules
pub mod c8;
pub mod screen;

// CONSTANTS
pub const SCREEN_LINES: usize = 32;
pub const SCREEN_COLUMNS: usize = 64;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// The file you want the emulator to execute.
    binary: String,
}

fn main() {
  let cli = Cli::parse();
  let binary_path = Path::new(&cli.binary);

  if !binary_path.exists() && binary_path.is_file() {
     println!("Error: Invalid path.");
  }

  let mut stdin = async_stdin().bytes();
  let mut chip8 = Chip8::new();

  chip8.load_file(&binary_path);
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

    screen::write_array(chip8.get_display());
    
    //TODO: Calculate sleep to make this a 60Hz loop.
    thread::sleep(Duration::from_millis(50));
  }
}
