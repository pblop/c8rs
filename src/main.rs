use crate::c8::Chip8;

pub mod c8;
pub mod screen;

fn main() {
  let mut chip8 = Chip8::new();

  // Loop until the terminal screen is 32x64.
  while !screen::is_correct_screen_size(32, 64) {
    let (lines, columns) = screen::get_screen_size();
 
    println!("Expected 32x64 screen, current screen is {}x{}", lines, columns);
  }


  println!("Hello, world!");
}
