extern crate termion;
use crate::{SCREEN_LINES, SCREEN_COLUMNS};
use std::io::{Write, stdout, Stdout, Read, Bytes};
use termion::{async_stdin, AsyncReader};
use termion::input::MouseTerminal;
use termion::raw::{IntoRawMode, RawTerminal};

#[derive (Debug)]
enum MyColor {
  Black,
  White,
  Orange,
  Yellow,
  Green
}

impl termion::color::Color for MyColor {
  fn write_fg(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      MyColor::Black => termion::color::Black.write_fg(f),
      MyColor::White => termion::color::White.write_fg(f),
      MyColor::Orange => termion::color::Rgb(174, 94, 22).write_fg(f),
      MyColor::Yellow => termion::color::Rgb(253, 195, 45).write_fg(f),
      MyColor::Green => termion::color::Green.write_fg(f),
    }
  }

  fn write_bg(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      MyColor::Black => termion::color::Black.write_bg(f),
      MyColor::White => termion::color::White.write_bg(f),
      MyColor::Orange => termion::color::Rgb(174, 94, 22).write_bg(f),
      MyColor::Yellow => termion::color::Rgb(253, 195, 45).write_bg(f),
      MyColor::Green => termion::color::Green.write_bg(f),
    }
  }
}

#[derive(clap::ArgEnum, Clone)]
pub enum ColorScheme {
  BlackWhite,
  OrangeYellow,
  BlackGreen
}
impl ColorScheme {
  fn get_fg(&self) -> termion::color::Fg<MyColor> {
    termion::color::Fg(match self {
      ColorScheme::BlackWhite => MyColor::Black,
      ColorScheme::OrangeYellow => MyColor::Orange,
      ColorScheme::BlackGreen => MyColor::Green
    })
  }
  fn get_bg(&self) -> termion::color::Bg<MyColor> {
    termion::color::Bg(match self  {
      ColorScheme::BlackWhite => MyColor::White,
      ColorScheme::OrangeYellow => MyColor::Yellow,
      ColorScheme::BlackGreen => MyColor::Black
    })
  }
 // pub fn from_str(color_scheme_str: &str) -> Option<ColorScheme> {
 //   match color_scheme_str {
 //     "black_white" => Some(ColorScheme::BlackWhite),
 //     "orange_yellow" => Some(ColorScheme::OrangeYellow),
 //     "black_green" => Some(ColorScheme::BlackGreen),
 //     _ => None
 //   }
 // }
}

const KEYMAP: [char; 16] = [
  'x', '1', '2', '3', 
  'q', 'w', 'e', 'a', 
  's', 'd', 'z', 'c', 
  '4', 'r', 'f', 'v'
];

pub struct Screen {
  stdout: MouseTerminal<RawTerminal<Stdout>>,
  stdin: Bytes<AsyncReader>,
  pub pressed_keys: [bool; 16],
  
  previous_screen_size: (usize, usize),
  color_scheme: ColorScheme
}

impl Screen {
  pub fn new(color_scheme: ColorScheme) -> Screen {
    Screen {
      stdout: MouseTerminal::from(stdout().into_raw_mode().unwrap()),
      stdin: async_stdin().bytes(),
      pressed_keys: [false; 16],
      
      previous_screen_size: (0,0),
      color_scheme,
    }
  }

  pub fn setup(&mut self) {
    write!(self.stdout, "{}{}{}", termion::clear::All, termion::cursor::Hide, termion::cursor::Goto(1,1)).unwrap();
  }

  // Updates pressed_keys and returns true if the pressed key means exit.
  // NOTE: Pressing a key currently presses the virtual key during 1 frame (I
  // think this is a terminal limitation), and pressing any key outside of the 
  // mapped keyboard exits the program.
  pub fn update_keys(&mut self) -> bool {
    self.pressed_keys = [false; 16];
    loop {
      let bopt = self.stdin.next();
      match bopt {
        Some(Ok(b)) => match KEYMAP.iter().position(|&x| x == (b as char)) {
          Some(index) => self.pressed_keys[index] = true,
          None => return true
        },
        Some(Err(_)) => {},
        None => break
      }
    }

    // Debug
    //eprint!("\x1b[{};{}H[screen] {:?}", 22, 0, self.pressed_keys);
    false
  }

  pub fn require_screen_size(&mut self, expected_lines: usize, expected_columns: usize) {
    while !self.is_correct_screen_size(expected_lines/2, expected_columns) {
      let (lines, columns) = self.get_screen_size();
  
      write!(self.stdout, "{}{}Expected at least {}x{} screen, current screen is {}x{}\n",
        termion::clear::All, termion::cursor::Goto(1,1),
        expected_lines/2, expected_columns, lines, columns).unwrap();
    }
  }

  pub fn get_screen_size(&mut self) -> (usize, usize) {
    let (columns, lines) = termion::terminal_size().unwrap();
    
    (lines as usize, columns as usize)
  }

  pub fn write(&mut self, prev: &[[bool; SCREEN_COLUMNS]; SCREEN_LINES], display: &[[bool; SCREEN_COLUMNS]; SCREEN_LINES]) {
    let curr_screen_size = self.get_screen_size();
    if self.previous_screen_size != curr_screen_size {
      self.write_array(display)      
    } else {
      self.write_changes(prev, display);
    }

    self.previous_screen_size = curr_screen_size;
  }

  pub fn write_array(&mut self, display: &[[bool; SCREEN_COLUMNS]; SCREEN_LINES]) {
    write!(self.stdout, "{}{}", termion::clear::All, termion::cursor::Goto(1,1)).unwrap();

    for i in (0..SCREEN_LINES).step_by(2) {
      for j in 0..SCREEN_COLUMNS {
        // To make pixels look square, I separate every line into two different
        // virtual sub-lines. The first sub-line is the top half of the pixel,
        // and the second sub-line is the bottom half of the pixel.

        // There's no need to check if the bottom half of the pixel should be 
        // rendered, a.k.a. we're not outside of the screen, a.k.a. the first 
        // sub-line is not the last line, because the chip8 screen is always
        // 32x64! (some implementations actually have more resolution, but I
        // think the number of rows is always a multiple of 2,
        // so it's not a problem)

        let first_pixel = display[i][j];
        let second_pixel = display[i + 1][j];
        let character = match (first_pixel, second_pixel) {
          (true, true) => "█",
          (true, false) => "▀",
          (false, true) => "▄",
          (false, false) => " "
        };

        write!(self.stdout, "{}{}{}{}",
          termion::cursor::Goto((j+1) as u16, (i/2+1) as u16),
          self.color_scheme.get_bg(),
          self.color_scheme.get_fg(),
          character).unwrap();
      }
    }
    //write!(self.stdout, "{}{}",
    //  termion::cursor::Goto((SCREEN_COLUMNS+1) as u16, (SCREEN_LINES+1) as u16),
    //  termion::color::Bg(termion::color::LightBlack)).unwrap();
    self.stdout.flush().unwrap();
  }

  pub fn write_changes(&mut self, prev: &[[bool; SCREEN_COLUMNS]; SCREEN_LINES], display: &[[bool; SCREEN_COLUMNS]; SCREEN_LINES]) {
    let mut has_printed = false;

    for i in (0..SCREEN_LINES).step_by(2) {
      for j in 0..SCREEN_COLUMNS {
        // To make pixels look square, I separate every line into two different
        // virtual sub-lines. The first sub-line is the top half of the pixel,
        // and the second sub-line is the bottom half of the pixel.

        // There's no need to check if the bottom half of the pixel should be 
        // rendered, a.k.a. we're not outside of the screen, a.k.a. the first 
        // sub-line is not the last line, because the chip8 screen is always
        // 32x64! (some implementations actually have more resolution, but I
        // think the number of rows is always a multiple of 2,
        // so it's not a problem)

        if prev[i][j] != display[i][j] || prev[i+1][j] == display[i+1][j]
        {
          let first_pixel = display[i][j];
          let second_pixel = display[i + 1][j];
          let character = match (first_pixel, second_pixel) {
            (true, true) => "█",
            (true, false) => "▀",
            (false, true) => "▄",
            (false, false) => " "
          };

          write!(self.stdout, "{}{}{}{}",
            termion::cursor::Goto((j+1) as u16, (i/2+1) as u16),
            self.color_scheme.get_bg(),
            self.color_scheme.get_fg(),
            character).unwrap();

          has_printed = true;
        }
      }
    }

    if has_printed {
      self.stdout.flush().unwrap();
    }
  }


  pub fn is_correct_screen_size(&mut self, expected_lines: usize, expected_columns: usize) -> bool {
    let (lines, columns) = self.get_screen_size();

    lines>=expected_lines && columns>=expected_columns
  }

  pub fn beep(&mut self) {
    write!(self.stdout, "\x07").unwrap();
    self.stdout.flush().unwrap();
  }
}
