#[macro_use] extern crate bitflags;

pub mod z80;
pub mod memory;
pub mod registers;
pub mod utils;

pub const WIDTH: usize = 224;
pub const HEIGHT: usize = 288;
