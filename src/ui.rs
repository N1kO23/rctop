use crossterm::{
  cursor::{Hide, MoveTo, Show},
  execute,
};
use crossterm::style::{ResetColor};
use crossterm::terminal::{
    Clear,
    ClearType::{All},
};

use std::{
  process,
  io::{stdout},
};

pub fn init() {
  execute!(stdout(), Hide).unwrap();
}

pub fn reset() {
  println!("Received Ctrl + C! Exiting...");
  execute!(stdout(), Show, Clear(All), ResetColor, MoveTo(0, 0)).unwrap();
  process::exit(0);
}