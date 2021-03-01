#[macro_use] extern crate bitflags;

pub mod z80;
pub mod memory;
pub mod pixel;
pub mod gfx_decoder;
pub mod registers;

pub const WIDTH: usize = 224;
