extern crate termion;
use crate::{SCREEN_LINES, SCREEN_COLUMNS};
use std::io::{Write, stdout, Stdout};
use termion::input::MouseTerminal;
use termion::raw::{IntoRawMode, RawTerminal};

pub struct Screen {
  stdout: MouseTerminal<RawTerminal<Stdout>>
}

impl Screen {
  pub fn new() -> Screen {
    Screen {
      stdout: MouseTerminal::from(stdout().into_raw_mode().unwrap())
    }
  }

  pub fn setup(&mut self) {
    write!(self.stdout, "{}{}{}", termion::clear::All, termion::cursor::Hide, termion::cursor::Goto(1,1)).unwrap();
  }

  pub fn require_screen_size(&mut self, expected_lines: usize, expected_columns: usize) {
    while !self.is_correct_screen_size(expected_lines, expected_columns) {
      let (lines, columns) = self.get_screen_size();
  
      write!(self.stdout, "{}{}Expected at least {}x{} screen, current screen is {}x{}\n",
        termion::clear::All, termion::cursor::Goto(1,1),
        expected_lines, expected_columns, lines, columns).unwrap();
    }
  }

  pub fn get_screen_size(&mut self) -> (usize, usize) {
    let (columns, lines) = termion::terminal_size().unwrap();
    
    (lines as usize, columns as usize)
  }

  pub fn write_array(&mut self, display: &[[bool; SCREEN_COLUMNS]; SCREEN_LINES]) {
    write!(self.stdout, "{}{}", termion::clear::All, termion::cursor::Goto(1,1)).unwrap();
    for i in 0..SCREEN_LINES {
      for j in 0..SCREEN_COLUMNS {
        // I *MUST* be able to store termion::color::White and Black
        // in a variable. But I can't seem to find how to.
        if display[i][j] {
          write!(self.stdout, "{}{} ", 
            termion::cursor::Goto((j+1) as u16, (i+1) as u16), 
            termion::color::Bg(termion::color::White)).unwrap();
        } else {
          write!(self.stdout, "{}{} ",
            termion::cursor::Goto((j+1) as u16, (i+1) as u16), 
            termion::color::Bg(termion::color::Black)).unwrap();
        }
      }
    }
    write!(self.stdout, "{}{}",
      termion::cursor::Goto((SCREEN_COLUMNS+1) as u16, (SCREEN_LINES+1) as u16),
      termion::color::Bg(termion::color::LightBlack)).unwrap();
    self.stdout.flush().unwrap();
  }

  pub fn is_correct_screen_size(&mut self, expected_lines: usize, expected_columns: usize) -> bool {
    let (lines, columns) = self.get_screen_size();

    lines>=expected_lines && columns>=expected_columns
  }
}
