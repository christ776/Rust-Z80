use std::{convert::TryFrom, fmt};

bitflags!{
/// flags (f): sz-h-pnc
/// Symbol Field Name
/// C   Carry Flag
/// N   Add/Subtract
/// P/V Parity/Overflow Flag
/// H Half Carry Flag
/// Z Zero Flag
/// S Sign Flag
/// X Not Used
///p/v - parity or overflow
/// parity set if even number of bits set
/// overflow set if the 2-complement result does not fit in the register
  pub struct Flags: u8 {
      const CARRY = 0x01;
      const NEGATIVE = 0x02;
      const PARITY = 0x04;
      const COPY_BIT_3 = 0x08;
      const HALFCARRY = 0x10;
      const COPY_BIT_5 = 0x20;
      const ZERO = 0x40;
      const SIGN = 0x80;
  }
}

impl Flags {
  pub fn check(&self, condition: bool) -> Flags {
      if condition {
          *self
      } else {
          Flags::empty()
      }
  }
}

#[derive(Clone, Copy, Debug)]
pub enum Register8Bit {
    A = 7,
    B = 0,
    C = 1,
    D = 2,
    E = 3,
    H = 4,
    I = 9,
    L = 5,
}

impl TryFrom<u8> for Register8Bit {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            x if x == Register8Bit::A as u8 => Ok(Register8Bit::A),
            x if x == Register8Bit::B as u8 => Ok(Register8Bit::B),
            x if x == Register8Bit::C as u8 => Ok(Register8Bit::C),
            x if x == Register8Bit::D as u8 => Ok(Register8Bit::D),
            x if x == Register8Bit::E as u8 => Ok(Register8Bit::E),
            x if x == Register8Bit::H as u8 => Ok(Register8Bit::H),
            x if x == Register8Bit::L as u8 => Ok(Register8Bit::L),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Register16Bit {
    AF, AF2, BC, DE, HL, SP, IX, IY, BC2, DE2, HL2 
}

#[derive(Clone, Copy, Debug)]
pub struct Registers {
    pub a: u8,
    pub a2: u8,
    pub f: Flags,
    pub f2: Flags,
    pub b: u8,
    pub b2: u8,
    pub c: u8,
    pub c2: u8,
    pub d: u8,
    pub d2: u8,
    pub e: u8,
    pub e2: u8,
    pub h: u8,
    pub i: u8,
    pub h2: u8,
    pub l: u8,
    pub l2: u8,
    pub sp: u16,
    pub pc: u16,
    pub ixl : u8,
    pub ixh: u8,
    pub iyl: u8,
    pub iyh: u8,
}

impl Registers {
  pub fn new() -> Registers {
      Registers {
          a: 0x0,
          a2: 0x0,
          f: Flags::empty(),
          f2: Flags::empty(),
          b: 0x00,
          b2: 0x00,
          c: 0x0,
          c2: 0x0,
          d: 0x00,
          d2: 0x00,
          e: 0x0,
          e2: 0x0,
          h: 0x0,
          i: 0,
          h2: 0x0,
          l: 0x0,
          l2: 0x0,
          sp: 0x4FBF,
          pc: 0x0,
          ixh: 0,
          ixl: 0,
          iyl: 0,
          iyh: 0
      }
  }

  pub fn get_u16(&self, reg: Register16Bit) -> u16 {
      use self::Register16Bit::*;
      match reg {
          AF => ((self.a as u16) << 8) | (self.f.bits() as u16),
          AF2 => ((self.a2 as u16) << 8) | (self.f2.bits() as u16),
          BC => ((self.b as u16) << 8) | (self.c as u16),
          BC2 => ((self.b2 as u16) << 8) | (self.c2 as u16),
          DE => ((self.d as u16) << 8) | (self.e as u16),
          DE2 => ((self.d2 as u16) << 8) | (self.e2 as u16),
          HL => ((self.h as u16) << 8) | (self.l as u16),
          IX =>  ((self.ixh as u16) << 8) | (self.ixl as u16),
          IY =>  ((self.iyh as u16) << 8) | (self.iyl as u16),
          HL2 => ((self.h2 as u16) << 8) | (self.l2 as u16),
          SP => self.sp,
      }
  }

  pub fn set_u16(&mut self, reg: Register16Bit, value: u16) {
      use self::Register16Bit::*;
      match reg {
          AF => { self.a = (value >> 8) as u8; self.f = Flags::from_bits_truncate(value as u8) },
          AF2 => { self.a2 = (value >> 8) as u8; self.f2 = Flags::from_bits_truncate(value as u8) },
          BC => { self.b = (value >> 8) as u8; self.c = value as u8; },
          BC2 => { self.b2 = (value >> 8) as u8; self.c2 = value as u8; },
          DE => { self.d = (value >> 8) as u8; self.e = value as u8; },
          DE2 => { self.d2 = (value >> 8) as u8; self.e2 = value as u8; },
          HL => { self.h = (value >> 8) as u8; self.l = value as u8; },
          HL2 => { self.h2 = (value >> 8) as u8; self.l2 = value as u8; },
          IX => { self.ixh = (value >> 8) as u8; self.ixl = value as u8; },
          IY => { self.iyh = (value >> 8) as u8; self.iyl = value as u8; },
          SP => self.sp = value,
      }
  }
}

impl fmt::Display for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "pc:{:04x} sp:{:04x} \
                   a:{:02x} f:{:04b} \
                   b:{:02x} c:{:02x} \
                   d:{:02x} e:{:02x} \
                   h:{:02x} l:{:02x}",
                   self.pc, self.sp,
                   self.a, self.f.bits() >> 4,
                   self.b, self.c,
                   self.d, self.e,
                   self.h, self.l)
    }
}