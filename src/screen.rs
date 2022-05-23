extern crate termion;
use crate::{SCREEN_LINES, SCREEN_COLUMNS};
use std::io::{Write, stdout};
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;

pub fn setup_screen() {
  print!("{}{}{}", termion::clear::All, termion::cursor::Hide, termion::cursor::Goto(1,1));
}

pub fn require_screen_size(expected_lines: usize, expected_columns: usize) {
  while !is_correct_screen_size(expected_lines, expected_columns) {
    let (lines, columns) = get_screen_size();
 
    println!("{}{}Expected at least {}x{} screen, current screen is {}x{}",
      termion::clear::All, termion::cursor::Goto(1,1),
      expected_lines, expected_columns, lines, columns);
  }
}

pub fn get_screen_size() -> (usize, usize) {
  let (columns, lines) = termion::terminal_size().unwrap();
  
  (lines as usize, columns as usize)
}

pub fn write_array(display: &[[bool; SCREEN_COLUMNS]; SCREEN_LINES]) {
  let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());
  write!(stdout, "{}{}", termion::clear::All, termion::cursor::Goto(1,1)).unwrap();
  
  for i in 0..SCREEN_LINES {
    for j in 0..SCREEN_COLUMNS {
      // I *MUST* be able to store termion::color::White and Black
      // in a variable. But I can't seem to find how to.
      if display[i][j] {
        write!(stdout, "{}{} ", 
          termion::cursor::Goto((j+1) as u16, (i+1) as u16), 
          termion::color::Bg(termion::color::White)).unwrap();
      } else {
        write!(stdout, "{}{} ",
          termion::cursor::Goto((j+1) as u16, (i+1) as u16), 
          termion::color::Bg(termion::color::Black)).unwrap();
      }
    }
  }
  write!(stdout, "{}{}",
    termion::cursor::Goto((SCREEN_COLUMNS+1) as u16, (SCREEN_LINES+1) as u16),
    termion::color::Bg(termion::color::LightBlack)).unwrap();
  stdout.flush().unwrap();
}

pub fn is_correct_screen_size(expected_lines: usize, expected_columns: usize) -> bool {
  let (lines, columns) = get_screen_size();

  lines>=expected_lines && columns>=expected_columns
}

