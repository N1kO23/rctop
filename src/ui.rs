use crossterm::style::{ Color, ResetColor, SetBackgroundColor, SetForegroundColor };
use crossterm::terminal::{
    Clear,
    ClearType::{All, CurrentLine},
    ScrollUp,
};
use crossterm::{
    cursor::{MoveTo, Hide, Show},
    execute,
    event::{Event, read, KeyCode},
};

use std::{
  process,
  io::{stdout},
};
use std::sync::{Arc, Mutex};

use crate::datafetcher::SystemData;
use crate::utils;

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Prepare the terminal for the UI
pub fn init() {
  execute!(stdout(), Hide).unwrap();
}

/// Reset the terminal back to its original state and clear the programs output
pub fn exit() {
  println!("Exiting...");
  execute!(stdout(), Show, Clear(All), ResetColor, MoveTo(0, 0), ScrollUp(5)).unwrap();
}

pub fn reset() -> crossterm::Result<()> {
  execute!(stdout(), Clear(All), ResetColor, MoveTo(0, 0)).unwrap();
  update_top_header()?;
  Ok(())
}

pub fn update_top_header() -> crossterm::Result<()> {
  let mut term_size = crossterm::terminal::size().unwrap();
  let mut top_left_str: String = String::new();
  let mut top_right_str: String = String::new();
  top_left_str += &format!(
    "RCTOP v{} [Width: {}, Height: {}]",
    VERSION, term_size.0, term_size.1
  );
  //top_right_str += &format!("Uptime: {}", utils::parse_time(&sys.uptime));
  execute!(
      stdout(),
      MoveTo(0, 0),
      Clear(CurrentLine),
      SetBackgroundColor(Color::DarkCyan),
      SetForegroundColor(Color::Black),
      MoveTo(0, 0)
  )?;
  print!("{}", top_left_str);
  print!("{}", top_right_str);
  execute!(
      stdout(),
      ResetColor,
  )?;
  Ok(())
}

pub fn update_menu_header(selection: &mut (usize, usize), term_size: (u16, u16)) {
}

pub fn print_system_data(thr_data: &Arc<Mutex<SystemData>>, selection: &mut (usize, usize), term_size: (u16, u16)) {
  let mut shared_data = thr_data.lock().unwrap();
}