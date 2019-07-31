#![allow(unused)]

/// A Z80 instruction is built from 3 bit groups,
/// the topmost two bits split the instruction space into 4 broad instruction groups,
/// the other 6 bits form two 3-bit groups which have a different meaning based on the instruction group:
///  +---+---+ +---+---+---+ +---+---+---+
///  | x | x | | y | y | y | | z | z | z |
///  +---+---+ +---+---+---+ +---+---+---+
///    7   6     5   4   3     2   1   0


///    A  F           AF'
///    B  C => BC     BC'
///    D  E => DE     DE'
///    H  L => HL     HL'
///
///    I       IX
///    R       IY
///            SP
///            PC
///  flags (F): SZ-H-PNC
///


pub use crate:: memory::Memory;

// use ::memory::Memory;

// use memory::Memory;

pub enum Register {
  A = 0b111,
  B = 0b000,
  C = 0b001,
  D = 0b010,
  E = 0b011,
  H = 0b100,
  L = 0b101
}

pub struct Z80 {
  pub a: u8,
  pub b: u8,
  pub c: u8,
  pub d: u8,
  pub e: u8,
  pub pc: u16,
  pub sp: u16,
  pub HL: u16,
  pub mem: Memory,
  ///flags (F): SZ-H-PNC
  pub Flags: u8,
}

impl Z80 {
  pub fn new() -> Z80 {
    Z80{ a:0,
          pc: 0,
          b: 0,
          c: 0,
          d: 0,
          e: 0,
          sp: 0,
          HL: 0,
          mem: Memory::new(),
          Flags: 0x0
        }
  }


  pub fn Flags_getZ(&mut self) -> bool {
    (self.Flags & 0b0100_0000) >> 7 != 0
  }

  pub fn Flags_setZ(&mut self, val:bool) {
    if val {
      self.Flags |= 0b0100_0000;
    } else {
      self.Flags &= 0b1011_1111;
    }
  }

  pub fn Flags_setS(&mut self, val:bool) {
    if val {
      self.Flags |= 0b1000_0000;
    } else {
      self.Flags &= 0b0111_1111;
    }
  }

  pub fn Flags_getS(&mut self) -> bool {
    (self.Flags & 0b1000_0000) >> 7 != 0
  }

  ///N - Subtract - Set if the last operation was a subtraction
  pub fn Flags_setN(&mut self, val:bool) {
    if val {
      self.Flags |= 0b0000_0010;
    } else {
      self.Flags &= 0b1111_1101;
    }
  }

  ///C - Carry - Set if the result did not fit in the register
  pub fn Flags_setC(&mut self, val:bool) {
    if val {
      self.Flags |= 0b0000_0001;
    } else {
      self.Flags &= 0b1111_1110;
    }
  }

  ///H - Half Carry - Carry from bit 3 to bit 4
  pub fn Flags_setH(&mut self, val:bool) {
    if val {
      self.Flags |= 0b0001_0000;
    } else {
      self.Flags &= 0b1110_1111;
    }
  }

  ///P/V - Parity or Overflow
  /// Parity set if even number of bits set
  /// Overflow set if the 2-complement result does not fit in the register
  pub fn Flags_setPE(&mut self, val:bool) {
    if val {
      self.Flags |= 0b0000_0100;
    } else {
      self.Flags &= 0b1111_1011;
    }
  }

  pub fn Flags_getPE(&mut self) -> bool {
    (self.Flags & 0b0000_0100) >> 2 != 0
  }

  ///C - Carry - Set if the result did not fit in the register
  pub fn Flags_getC(&mut self) -> bool  {
    return (self.Flags & 0b0000_0001) == 1;
  }

  pub fn push(&mut self, val:u16) {
    let addr = self.sp - 2;
    self.set_sp(addr);
    self.mem.w16(addr, val);
  }

  pub fn set_A(&mut self, a: u8) {
    self.a = a
  }

  pub fn set_B(&mut self, b: u8) {
    self.b = b
  }

  pub fn set_C(&mut self, c: u8) {
    self.c = c
  }

  pub fn set_D(&mut self, d: u8) {
    self.d = d
  }

  pub fn set_E(&mut self, e: u8) {
    self.e = e
  }

  pub fn pc(&self) -> u16 {
    self.pc
  }

  pub fn set_sp(&mut self, sp: u16) {
    self.sp = sp;
  }

  pub fn step(&mut self) -> u16 {
    self.pc += 1;
    self.pc
  }

  pub fn set_pc(&mut self, pc: u16) {
    self.pc = pc;
  }

  pub fn set_DE(&mut self, DE: u16) {
    self.d = (DE >> 8) as u8;
    self.e = (DE & 0x0F) as u8;
  }

  pub fn AF(&mut self) -> u16 {
    ((self.a as u16) << 8 as u16 | self.Flags as u16)
  }

  pub fn set_HL(&mut self, HL: u16) {
    self.HL = HL;
  }

  pub fn BC(& mut self) -> u16 {
    ((self.b as u16) << 8 as u16 | self.c as u16)
  }

   pub fn DE(& mut self) -> u16 {
    ((self.d as u16) << 8 as u16 | self.e as u16)
  }

  pub fn get_HL_H(&mut self) -> u8 {
    ((self.HL & 0xF0) >> 8) as u8
  }

   pub fn get_HL_L(&mut self) -> u8 {
    ((self.HL & 0x0F) >> 8) as u8
  }

  pub fn set_HL_L(&mut self, L: u8) {
    let H = self.HL & 0xF0;
    self.HL = H | L as u16;
  }

  pub fn set_HL_H(&mut self, H: u8) {
    let L = self.HL & 0x0F;
    self.HL = H as u16 | L;
  }
}