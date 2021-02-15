/// a self instruction is built from 3 bit groups,
/// the topmost two bits split the instruction space into 4 broad instruction groups,
/// the other 6 bits form two 3-bit groups which have a different meaning based on the instruction group:
///  +---+---+ +---+---+---+ +---+---+---+
///  | x | x | | y | y | y | | z | z | z |
///  +---+---+ +---+---+---+ +---+---+---+
///    7   6     5   4   3     2   1   0


///    a  f           af'
///    b  c => bc     bc'
///    d  e => de     de'
///    h  l => hl     hl'
///
///    i       ix
///    r       iy
///            sp
///            pc
///  flags (f): sz-h-pnc
///
/// 
pub use crate:: memory::Memory;

use crate::registers::{
  Registers, Register8Bit, Register16Bit, Flags,
};
use crate::registers::Register8Bit::{
  A, B, C, D, E, H, I, L
};
use crate::registers::Register16Bit::{
  AF, BC, DE, HL, SP, IX, IY
};

#[derive(Clone, Copy, Debug)]
pub enum Address {
    BC, DE, HL, NextU16
}

impl ReadU8 for Address {
  fn read_u8(&self, cpu: &mut Z80) -> u8 {
      let address = cpu.get_address(self);
      cpu.mem.r8(address)
  }
}

impl WriteU8 for Address {
  fn write_u8(&self, cpu: &mut Z80, value: u8) {
      let address = cpu.get_address(self);
      cpu.mem.w8(address, value);
  }
}

impl WriteU16 for Address {
  fn write_u16(&self, cpu: &mut Z80, value: u16) {
      let address = cpu.get_address(self);
      cpu.mem.w16(address, value);
  }
}

pub trait ReadU8 {
  fn read_u8(&self, cpu: &mut Z80) -> u8;
}

pub trait WriteU8 {
  fn write_u8(&self, cpu: &mut Z80, value: u8);
}

pub trait ReadU16 {
  fn read_u16(&self, cpu: &mut Z80) -> u16;
}

pub trait WriteU16 {
  fn write_u16(&self, cpu: &mut Z80, value: u16);
}

pub struct NextU8;
impl ReadU8 for NextU8 {
  fn read_u8(&self, cpu: &mut Z80) -> u8 {
      cpu.next_u8()
  }
}

pub struct NextU16;
impl ReadU16 for NextU16 {
  fn read_u16(&self, cpu: &mut Z80) -> u16 {
      cpu.next_u16()
  }
}

impl ReadU8 for Register8Bit {
  fn read_u8(&self, cpu: &mut Z80) -> u8 {
      use Register8Bit::*;
      match *self {
          A => cpu.r.a,
          B => cpu.r.b,
          C => cpu.r.c,
          D => cpu.r.d,
          E => cpu.r.e,
          H => cpu.r.h,
          L => cpu.r.l,
          I => cpu.r.i,
      }
  }
}

impl WriteU8 for Register8Bit {
  fn write_u8(&self, cpu: &mut Z80, value: u8) {
      use Register8Bit::*;
      match *self {
          A => cpu.r.a = value,
          B => cpu.r.b = value,
          C => cpu.r.c = value,
          D => cpu.r.d = value,
          E => cpu.r.e = value,
          H => cpu.r.h = value,
          L => cpu.r.l = value,
          I => cpu.r.i = value
      }
  }
}

impl ReadU16 for Register16Bit {
  fn read_u16(&self, cpu: &mut Z80) -> u16 {
      use Register16Bit::*;
      match *self {
          AF | BC | DE | HL | HL2 | IX | IY | BC2  | DE2 => cpu.r.get_u16(*self),
          SP => cpu.r.sp,
      }
  }
}

impl WriteU16 for Register16Bit {
  fn write_u16(&self, cpu: &mut Z80, value: u16) {
      use Register16Bit::*;
      match *self {
          AF | BC | DE | HL | HL2 | IX | IY | BC2  | DE2 => cpu.r.set_u16(*self, value),
          SP => cpu.r.sp = value,
      }
  }
}

pub struct Z80 {

  pub mem: Memory,
  pub r: Registers,
  _vblank_interrupt: bool,
  enable_hw_interrupt: bool,
  enable_int: bool,
  pub port_a_addr: u8,
  halted: bool,
}

#[derive(Clone, Copy, Debug)]
pub enum Condition {
    NOTZERO, ZERO, NOTCARRY, CARRY, PARITYODD, PARITYEVEN, NEGATIVE, POSITIVE, 
}

impl Condition {
    fn check(&self, flags: Flags) -> bool {
        use self::Condition::*;
        match *self {
            NOTZERO => !flags.contains(Flags::ZERO),
            ZERO => flags.contains(Flags::ZERO),
            NOTCARRY => !flags.contains(Flags::CARRY),
            CARRY => flags.contains(Flags::CARRY),
            NEGATIVE => flags.contains(Flags::NEGATIVE),
            POSITIVE => !flags.contains(Flags::NEGATIVE),
            PARITYEVEN => !flags.contains(Flags::PARITY),
            PARITYODD => flags.contains(Flags::PARITY),
        }
    }
}

impl Z80 {
  pub fn new(memory: Memory) -> Z80 {
    Z80{  
          // sp: 0x4FEF,
          r: Registers::new(),
          mem: memory,
          _vblank_interrupt: false,
          enable_hw_interrupt: false,
          enable_int: false,
          port_a_addr: 0,
          halted: false,
        }
  }

  fn next_u8(&self) -> u8 {
    self.mem.r8(self.r.pc + 1)
  }

  fn next_u16(&self) -> u16 {
    self.mem.r16(self.r.pc + 1)
  }

  fn get_address(&mut self, address: &Address) -> u16 {
    use self::Address::*;
    match *address {
      HL => self.r.get_u16(Register16Bit::HL),
      BC => self.r.get_u16(Register16Bit::BC),
      DE => self.r.get_u16(Register16Bit::DE),
      NextU16 => self.next_u16()
    }
  }

  ///This stack usually starts at $0000 so as to place at the very end of memory
  ///(the first push to the stack decrements the stack pointer causing it to wrap around to $FFFF).
  fn push(&mut self, val:u16) {
    let sp = self.r.sp;
    let addr = sp.wrapping_sub(2);
    self.r.sp = addr;
    self.mem.w16(addr, val);
  }

  pub fn step(&mut self) {
    self.r.pc += 1;
  }

  pub fn step_n(&mut self, n: u16) {
    self.r.pc += n;
  }

  pub fn exec(&mut self) -> u8 {

    if self.enable_int && self._vblank_interrupt && self.enable_hw_interrupt {
      //Check for interrupts
      self._vblank_interrupt = false;
      self.enable_hw_interrupt = false;
      self.enable_int = false;
      let interrup_handler_addr = self.mem.r16((self.r.i as u16) << 8 | self.port_a_addr as u16);
      self.call(interrup_handler_addr);
    }

    let op = self.mem.r8(self.r.pc);
    // println!("opcode {}! at {}", format!("{:#x}", op), format!("{:#x}", self.r.pc));
  
    match op {
        0x00 => { self.nop(); },
        0x0f => { self.rrca(A) },
        0xf3 => { self.di() },
        0x01 => { self.ld_16_nn(BC,NextU16) },
        0x11 => { self.ld_16_nn(DE,NextU16) },
        0x21 => { self.ld_16_nn(HL,NextU16) },
        0x31 => { self.ld_16_nn(SP,NextU16) },
        0x07 => {self.rlca(A) },
        // 0x08 => { self.ex(AF, AF2) },
        0x17 => {self.rla(A) },
        0x1f => {self.rra(A) },
        0x09 => { self.add_hl_ss(BC, HL) },
        0x19 => { self.add_hl_ss(DE, HL) },
        0x29 => { self.add_hl_ss(HL, HL) },
        0x10 => { self.djnz(NextU8) },
        0x02 => { self.ld_8_nn(Address::BC, A) },
        0x12 => { self.ld_8_nn(Address::DE, A) },
        0x18 => { self.jr_e(NextU8) },
        0x20 => { self.jr_conditional(Condition::NOTZERO) },
        0x30 => { self.jr_conditional(Condition::NOTCARRY) },
        0x38 => { self.jr_conditional(Condition::CARRY) },
        0x28 => { self.jr_conditional(Condition::ZERO) },
        0x5 => { self.dec_r(B) },
        0x0d => { self.dec_r(C) },
        0x15 => { self.dec_r(D) },
        0x1d => { self.dec_r(E) },
        0x25 => { self.dec_r(H) },
        0x2d => { self.dec_r(L) },
        0x3d => { self.dec_r(A) },
        0x35 => { self.dec_r(Address::HL) },
        0x03 => {self.inc_ss(BC)},
        0x13 => {self.inc_ss(DE)},
        0x23 => {self.inc_ss(HL)},
        0x34 => {self.inc_hl(Address::HL)},
        0x2a => { self.ld_hl_nn(NextU16) },
        0x0b => { self.dec_ss(BC) },
        0x1b => { self.dec_ss(DE) },
        0x2b => { self.dec_ss(HL) },
        0x3b => { self.dec_ss(SP) },
        0x04 => { self.inc_r(B) },
        0x0c => { self.inc_r(C) },
        0x14 => { self.inc_r(D) },
        0x1c => { self.inc_r(E) },
        0x24 => { self.inc_r(H) },
        0x2c => { self.inc_r(L) },
        0x3c => { self.inc_r(A) },
        0x2f => { self.cpl() },
        0x32 => { self.ld_nn_a(A, NextU16)},
        0x22 => { self.ld_nn_hl(NextU16, Address::HL)},
        0x36 => { 
          self.ld_hl_r(Register16Bit::HL, NextU8);
          self.step();
        },
        0x3a => { self.ld_a_nn(NextU16)},
        0x06 => { 
          self.ld_r_r(B, NextU8);
          self.step();
        },
        0x0e => { 
          self.ld_r_r(C, NextU8);
          self.step();
        },
        0x16 => { 
          self.ld_r_r(D, NextU8);
          self.step();
        },
        0x1e => { 
          self.ld_r_r(E, NextU8);
          self.step();
        },
        0x26 => { 
          self.ld_r_r(H, NextU8);
          self.step();
        },
        0x2e => { 
          self.ld_r_r(L, NextU8);
          self.step();
        },
        0x3e => { 
          self.ld_r_r(A, NextU8);
          self.step();
        },
        0xb6 => { self.or_r(Address::HL) },
        0x88 => { self.adc_r(B) },
        0x89 => { self.adc_r(C) },
        0x8a => { self.adc_r(D) },
        0x8b => { self.adc_r(E) },
        0x8c => { self.adc_r(H) },
        0x8d => { self.adc_r(L) },
        0x8f => { self.adc_r(A) } 
        0xce => { 
          self.adc_r(NextU8);
          self.step();
         },
        0x6f => { self.ld_r_r(L,A)},
        0x44 => { self.ld_r_r(B,H)},
        0x47 => { self.ld_r_r(B,A)},
        0x48 => { self.ld_r_r(C,B)},
        0x49 => { self.ld_r_r(C,C)},
        0x4a => { self.ld_r_r(C,D)},
        0x4c => { self.ld_r_r(C,H)},
        0x4f => { self.ld_r_r(C,A)},
        0x50 => { self.ld_r_r(D,B)},
        0x51 => { self.ld_r_r(D,C)},
        0x52 => { self.ld_r_r(D,D)},
        0x53 => { self.ld_r_r(D,E)},
        0x54 => { self.ld_r_r(D,H)},
        0x55 => { self.ld_r_r(D,L)},
        0x5c => { self.ld_r_r(C,H)}, 
        0x5d => { self.ld_r_r(E,L)},
        0x5f => { self.ld_r_r(E,A)},
        0x58 => { self.ld_r_r(C,B)},
        0x61 => { self.ld_r_r(H,C)},
        0x62 => { self.ld_r_r(H,D)},
        0x63 => { self.ld_r_r(H,E)},
        0x64 => { self.ld_r_r(H,H)},
        0x65 => { self.ld_r_r(H,L)},
        0x67 => { self.ld_r_r(H,A)},
        0x68 => { self.ld_r_r(H,A)},
        0x6a => { self.ld_r_r(L,D)},
        0x6c => { self.ld_r_r(L,H)},
        0x78 => { self.ld_r_r(A,B)},
        0x79 => { self.ld_r_r(A,C)},
        0x7a => { self.ld_r_r(A,D)},
        0x7b => { self.ld_r_r(A,E)},
        0x7c => { self.ld_r_r(A,H)},
        0x7d => { self.ld_r_r(A,L)},
        0x4e => { self.ld_r_hl(C, Address::HL)},
        0x1a => { self.ld_r_hl(A, Address::DE)},
        0x0a => { self.ld_r_hl(A, Address::BC)},
        0x46 => { self.ld_r_hl(B, Address::HL)},
        0x56 => { self.ld_r_hl(D, Address::HL)},
        0x5e => { self.ld_r_hl(E, Address::HL)},
        0x66 => { self.ld_r_hl(H, Address::HL)},
        0x6e => { self.ld_r_hl(L, Address::HL)},
        0x7e => { self.ld_r_hl(A, Address::HL)},
        0x70 => { self.ld_hl_r(Register16Bit::HL,B)},
        0x71 => { self.ld_hl_r(Register16Bit::HL,C)},
        0x72 => { self.ld_hl_r(Register16Bit::HL,D)},
        0x73 => { self.ld_hl_r(Register16Bit::HL,E)},
        0x74 => { self.ld_hl_r(Register16Bit::HL,H)},
        0x75 => { self.ld_hl_r(Register16Bit::HL,L)},
        0x77 => { self.ld_hl_r(Register16Bit::HL,A)},
        0x76 => { self.halt()},
        0x80 => { self.add_a_r(B)},
        0x81 => { self.add_a_r(C)},
        0x82 => { self.add_a_r(D)},
        0x83 => { self.add_a_r(E)},
        0x84 => { self.add_a_r(H)},
        0x85 => { self.add_a_r(L)},
        0x86 => { self.add_a_r(Address::HL)},
        0x87 => { self.add_a_r(A)}, 
        0x90 => { self.sub_r(B) },
        0x91 => { self.sub_r(C) },
        0x92 => { self.sub_r(D) },
        0x93 => { self.sub_r(E) },
        0x94 => { self.sub_r(H) },
        0x95 => { self.sub_r(L) },
        0x97 => { self.sub_r(A) },
        0xa0 => {self.and_r(B)},
        0xa1 => {self.and_r(C)},
        0xa2 => {self.and_r(D)},
        0xa3 => {self.and_r(E)}
        0xa4 => {self.and_r(H)}
        0xa5 => {self.and_r(L)},
        0xa7 => {self.and_r(A)},
        0xe6 => { 
          self.and_r(NextU8);
          self.step();
        },
        0xa8 => { self.xor_r(B) },
        0xa9 => { self.xor_r(C) },
        0xaa => { self.xor_r(D) },
        0xab => { self.xor_r(E) },
        0xac => { self.xor_r(H) },
        0xad => { self.xor_r(L) },
        0xaf => { self.xor_r(A) },
        0xae => { self.xor_r(Address::HL) },
        0xee => { 
          self.xor_r(NextU8);
          self.step();
        },
        0xb0 => { self.or_r(B) },
        0xb1 => { self.or_r(C) },
        0xb2 => { self.or_r(D) },
        0xb3 => { self.or_r(E) },
        0xb4 => { self.or_r(H) },
        0xb5 => { self.or_r(L) },
        0xb7 => { self.or_r(A) },
        0xc0 => {self.ret_cc(Condition::NOTZERO)},
        0xc8 => {self.ret_cc(Condition::ZERO)},
        0xd0 => {self.ret_cc(Condition::NOTCARRY)},
        0xd8 => {self.ret_cc(Condition::CARRY)},
        0xe0 => {self.ret_cc(Condition::PARITYEVEN)},
        0xe8 => {self.ret_cc(Condition::PARITYODD)},
        0xf0 => {self.ret_cc(Condition::POSITIVE)},
        0xf8 => {self.ret_cc(Condition::NEGATIVE)},
        0xc3 => { self.jmp(NextU16); },
        0xc6 => {
           self.add_a_r(NextU8); 
           self.step();
        },
        0xc9 => {self.ret()},
        0xcb => { 
          let op = self.next_u8();
          self.step();
          match op {
            0x20 => { self.sla_m(B) },
            0x21 => { self.sla_m(C) },
            0x22 => { self.sla_m(D) },
            0x23 => { self.sla_m(E) },
            0x24 => { self.sla_m(H) },
            0x25 => { self.sla_m(L) },
            0x27 => { self.sla_m(A) },
            0x46 => { self.bit(0, Address::HL) },
            0x47 => { self.bit(0, A) },
            0x4e => { self.bit(1, Address::HL) },
            0x56 => { self.bit(2, Address::HL) },
            0x5e => { self.bit(3, Address::HL) },
            0x66 => { self.bit(4, Address::HL) },
            0x6e => { self.bit(5, Address::HL) },
            0x76 => { self.bit(6, Address::HL) },
            0x7e => { self.bit(7, Address::HL) },
            0x38 => { self.srl_m(B) },
            0x39 => { self.srl_m(C) },
            0x3a => { self.srl_m(D) },
            0x3b => { self.srl_m(E) },
            0x3c => { self.srl_m(H) },
            0x3d => { self.srl_m(L) },
            0x3f => { self.srl_m(A) }
            // 0x10 => { }
            _ => {  panic!("unknown opcode {}! at {}", format!("{:#x}", op), format!("{:#x}", self.r.pc)); }

          }
         },
        0xcd => { 
          let addr = self.next_u16();
          self.call(addr); 
        },
        0xd3 => { self.out(NextU8) },
        0xd6 => {
          self.sub_r(NextU8);
          self.step();
        },
        0xdd => {
          let op = self.next_u8();
          self.step();
          match op {
            0x21 => { self.ld_16_nn(IX, NextU16) },
            0x70 => { self.ld_ix_plus_d_r(IX, B, NextU8) }
            0x71 => { self.ld_ix_plus_d_r(IX, C, NextU8) }
            0x72 => { self.ld_ix_plus_d_r(IX, D, NextU8) }
            0x73 => { self.ld_ix_plus_d_r(IX, E, NextU8) }
            0x75 => { self.ld_ix_plus_d_r(IX, H, NextU8) }
            0x76 => { self.ld_ix_plus_d_r(IX, L, NextU8) }
            0x77 => { self.ld_ix_plus_d_r(IX, A, NextU8) }
            0x7e => { self.ld_r_ix_d(IX, NextU8, A) }
            0x36 => { self.ld_16_plus_d_n(IX) }
            0x86 => { self.add_a_ix_d(IX, NextU8) },
            0x09 => { self.add_ix_pp(BC, IX) },
            0x19 => { self.add_ix_pp(DE, IX) },
            0x29 => { self.add_ix_pp(IX, IX) },
            0x39 => { self.add_ix_pp(SP, IX) },
            0xE5 => { self.push_16(IX)},
            0xE1 => { self.pop_16(SP, IX)},
            0x35 => { self.dec_ix_n(IX, NextU8) }
            _ => {  panic!("unknown opcode {}! at {}", format!("{:#x}", op), format!("{:#x}", self.r.pc)); }
          }
        },
        0xeb => {self.ex_de_hl(DE, HL)},
        0xed => {
            let op = self.next_u8();
            self.step();
            match op {
              0x42 => { self.sbc_hl_ss(BC) },
              0x52 => { self.sbc_hl_ss(DE) },
              0x62 => { self.sbc_hl_ss(HL) },
              0x72 => { self.sbc_hl_ss(SP) },
              0x43 => { self.ed(BC, NextU16) },
              0x53 => { self.ed(DE, NextU16) },
              0x63 => { self.ed(HL, NextU16) },
              0x73 => { self.ed(SP, NextU16) },
              0x7b => { 
                self.ld_16_nn(SP, NextU16);
                self.r.pc -=1;  
              },
              0x47 => { 
                self.ld_r_r(I, A);
                self.r.pc -=1; 
              },
              0x5e => { self.im2() },
              0xb0 => { self.ldir(HL, DE, BC)}
              _ => {  panic!("unknown opcode {}! at {}", format!("{:#x}", op), format!("{:#x}", self.r.pc)); }
            }
        },
        0xfd => {
          let op = self.next_u8();
          self.step();
          match op {
            0x21 => { self.ld_16_nn(IY, NextU16) },
            0x36 => { self.ld_16_plus_d_n(IY) },
            0x46 => { 
              self.r.pc += 2;
              self.ld_r_16_d(B, IX, NextU8)
            },
            0x4e => { 
              self.r.pc += 2;
              self.ld_r_16_d(C, IX, NextU8)
            },
            0x56 => { 
              self.r.pc += 2;
              self.ld_r_16_d(D, IX, NextU8)
            },
            0x5e => { 
              self.r.pc += 2;
              self.ld_r_16_d(E, IX, NextU8)
            },
            0x66 => { 
              self.r.pc += 2;
              self.ld_r_16_d(H, IX, NextU8)
            },
            0x6e => { 
              self.r.pc += 2;
              self.ld_r_16_d(L, IX, NextU8)
            },
            0x75 => {
              self.ld_ix_plus_d_r(IY, L, NextU8);
            }
            0x7e => { 
              self.r.pc += 2;
              self.ld_r_16_d(A, IX, NextU8)
            },
            0xE5 => { self.push_16(IY)},
            0xE1 => { self.pop_16(SP, IY)},
            _ => {  panic!("unknown opcode {}! at {}", format!("{:#x}", op), format!("{:#x}", self.r.pc)); }
          }
        },
        0xd9 => { self.exx() },
        0xc1 => {self.pop_16(SP, BC)},
        0xd1 => {self.pop_16(SP, DE)},
        0xe1 => {self.pop_16(SP, HL)},
        0xf1 => {self.pop_16(SP, AF)},
        0xc2 => { self.jp_cc_nn(Condition::NOTZERO, NextU16) },
        0xca => { self.jp_cc_nn(Condition::ZERO, NextU16) },
        0xd2 => { self.jp_cc_nn(Condition::NOTCARRY, NextU16) },
        0xda => { self.jp_cc_nn(Condition::CARRY, NextU16) },
        0xe2 => { self.jp_cc_nn(Condition::PARITYODD, NextU16) },
        0xea => { self.jp_cc_nn(Condition::PARITYEVEN, NextU16) },
        0xf2 => { self.jp_cc_nn(Condition::POSITIVE, NextU16) },
        0xfa => { self.jp_cc_nn(Condition::NEGATIVE, NextU16) },
        0xe9 => { self.jmp(Register16Bit::HL) },
        0xc4 => { self.call_cc_nn(Condition::NOTZERO, NextU16) },
        0xcc => { self.call_cc_nn(Condition::ZERO, NextU16) },
        0xd4 => { self.call_cc_nn(Condition::NOTCARRY, NextU16) },
        0xdc => { self.call_cc_nn(Condition::CARRY, NextU16) },
        0xe4 => { self.call_cc_nn(Condition::PARITYEVEN, NextU16) },
        0xec => { self.call_cc_nn(Condition::PARITYODD, NextU16) },
        0xf4 => { self.call_cc_nn(Condition::POSITIVE, NextU16) },
        0xfc => { self.call_cc_nn(Condition::NEGATIVE, NextU16) },
        0xc5 => { self.push_16(BC) },
        0xd5 => { self.push_16(DE) },
        0xe5 => { self.push_16(HL) },
        0xf5 => { self.push_16(AF) },
        0xc7 => { self.rst_p(0x00) },
        0xcf => { self.rst_p(0x08) },
        0xd7 => { self.rst_p(0x10) },
        0xdf => { self.rst_p(0x18) },
        0xe7 => { self.rst_p(0x20) },
        0xef => { self.rst_p(0x28) },
        0xf7 => { self.rst_p(0x30) },
        0xff => { self.rst_p(0x38) },
        0xf9 => { self.ld_sp_hl(SP, HL) },
        0xfb => { self.ei() },
        0xfe => { 
          self.cp(NextU8);
          self.step();
        },
        // CP
        0xBF => self.cp(A),
        0xB8 => self.cp(B),
        0xB9 => self.cp(C),
        0xBA => self.cp(D),
        0xBB => self.cp(E),
        0xBC => self.cp(H),
        0xBD => self.cp(L),
        0xBE => self.cp(Address::HL),
        _ => {  panic!("unknown opcode {}! at {}", format!("{:#x}", op), format!("{:#x}", self.r.pc)); },
    }
    16
  }

  #[inline]
  fn sbc_hl_ss<RW: ReadU16>(&mut self, ss: RW) {
    let hl = self.r.get_u16(Register16Bit::HL);
    let value = ss.read_u16(self);
    let carried = if self.r.f.contains(Flags::CARRY) { 1 } else { 0 };
    let result = hl.wrapping_sub(value).wrapping_sub(carried);
    let mut borrow = false;
    for mask in [0b000_0001, 0b0000_0010, 0b0000_0100, 0b0000_1000,
    0b0001_0000, 0b0010_0000, 0b0100_0000, 0b1000_0000].iter() {
      
      if (value & mask) > (hl & mask) {
        borrow = true;
        break;
      }
    }

    self.r.set_u16(Register16Bit::HL, result);
    self.r.f = Flags::ZERO.check(result == 0) |
    Flags::NEGATIVE | Flags::SIGN.check(result & 0x80 != 0);
        Flags::CARRY.check(borrow);
    self.step_n(2);
  }

  #[inline]
  fn exx(&mut self) {
    let hl = self.r.get_u16(Register16Bit::HL);
    let de = self.r.get_u16(Register16Bit::DE);
    let bc = self.r.get_u16(Register16Bit::BC);
    let hl2 = self.r.get_u16(Register16Bit::HL2);
    let de2 = self.r.get_u16(Register16Bit::DE2);
    let bc2 = self.r.get_u16(Register16Bit::BC2);
    self.r.set_u16(Register16Bit::HL, hl2);
    self.r.set_u16(Register16Bit::DE, de2);
    self.r.set_u16(Register16Bit::BC, bc2);
    self.r.set_u16(Register16Bit::HL2, hl);
    self.r.set_u16(Register16Bit::DE2, de);
    self.r.set_u16(Register16Bit::BC2, bc);

    self.step();
  }

  pub fn vblank(&mut self) {
    self._vblank_interrupt = true
  }

  /* I/O Instructions */
  fn out<R: ReadU8>(&mut self, r: R) {
    let n = r.read_u8(self);
    if n == 0 {
      self.port_a_addr = self.r.a;
    }
    self.step();
    self.step();
  }

  fn im2(&mut self) {
    self.step();
  }

  fn ei(&mut self) {
    self.enable_int = true;
    self.step();
  }

  fn di(&mut self) {
    self.enable_int = false;
    self.step();
  }

  #[inline]
  fn bit<R: ReadU8>(&mut self, bit: u8, r: R) {
    let value = r.read_u8(self);
    let mask = 1 << bit;
    self.r.f = Flags::ZERO.check((value & mask) == 0) |
                Flags::HALFCARRY |
                (Flags::CARRY & self.r.f);
    self.step();
  }

  #[inline]
  fn sla_m<R: ReadU8>(&mut self, r: R) {
    let c = r.read_u8(self) & 0x80 != 0;
    let result = r.read_u8(self) << 1;
    self.r.f = Flags::CARRY.check(c) |
      Flags::SIGN.check(result & 0x80 != 0) |
      Flags::ZERO.check(result == 0);
    self.step();
  }

  #[inline]
  fn cp<R: ReadU8>(&mut self, r: R) {
    let value = r.read_u8(self);
    let (result, overflow) = self.r.a.overflowing_sub(value);
    let mut borrow = false;
    for mask in [0b000_0001, 0b0000_0010, 0b0000_0100, 0b0000_1000,
    0b0001_0000, 0b0010_0000, 0b0100_0000, 0b1000_0000].iter() {
      
      if (value & mask) > (self.r.a & mask) {
        borrow = true;
        break;
      }
    }

    self.r.f = Flags::ZERO.check(result == 0) |
                Flags::NEGATIVE |
                Flags::SIGN.check(result & 0x80 != 0) |
                Flags::HALFCARRY.check((self.r.a & 0xF) < (value & 0xF)) |
                Flags::PARITY.check(overflow) |
                Flags::CARRY.check(borrow);
    self.step();
  }

  /**
   * Extended Instructions, see more at http://clrhome.org/table/
   */
  fn ed<R: ReadU16, RW: ReadU16 + WriteU16>(&mut self, r: RW, addr: R) {
    let value = r.read_u16(self);
    let a = addr.read_u16(self);
    self.mem.w16(a, value);
    self.step_n(3);
  }

  /** Transfers a byte of data from the memory location pointed to by hl to the memory location pointed to by de.
      then hl and de are incremented and bc is decremented. If bc is not zero, this operation is repeated.
      Interrupts can trigger while this instruction is processing.
  */
  #[inline]
  fn ldir<RW: ReadU16 + WriteU16>(&mut self, hl: RW, de: RW, bc: RW) {
    
    loop {
      let bc_ = bc.read_u16(self);
      if bc_ == 0 { break }
      let hl_ = hl.read_u16(self);
      let de_ = de.read_u16(self);
      self.mem.w8(de_, self.mem.r8(hl_));
      hl.write_u16(self, hl_.wrapping_add(1));
      de.write_u16(self, de_.wrapping_add(1));
      
      bc.write_u16(self, bc_.wrapping_sub(1));
    }
  }

  #[inline]
  fn ex_de_hl<RW: ReadU16 + WriteU16>(&mut self, de: RW, hl: RW) {
    let de_ = de.read_u16(self);
    let hl_ = hl.read_u16(self);
    de.write_u16(self, hl_);
    hl.write_u16(self, de_);
    self.step();
  }

  #[inline]
  fn ld_nn_a<R: ReadU8, R16: ReadU16>(&mut self, r: R, a: R16) {
    let addr = a.read_u16(self);
    let value = r.read_u8(self);
    if addr == 0x5000 {
        let enable_int = value & 0x01;
        self.enable_hw_interrupt = enable_int != 0;
    } else {
      self.mem.w8(addr, value);
    }
    self.step_n(3);
  }

  #[inline]
  fn dec_ix_n<R: ReadU16, D: ReadU8>(&mut self, r: R, d: D) {
    let displacement = r.read_u16(self) + d.read_u8(self) as u16;
    let data = self.mem.r8(displacement);
    let r = data.wrapping_sub(1);
    self.mem.w8(displacement, r);
    self.r.f = Flags::NEGATIVE | Flags::ZERO.check(r == 0) | Flags::SIGN.check((r & 0x80) == 0x80);
    self.step();
    self.step();
  }

  #[inline]
  fn pop_16<W: WriteU16, RW: ReadU16 + WriteU16>(&mut self, sp: RW, dest: W) {
    let sp_contents = sp.read_u16(self);
    let data = self.mem.r16(sp_contents);
    dest.write_u16(self, data);
    sp.write_u16(self, sp_contents + 2);
    self.step();
  }

  #[inline]
  fn ld_16_nn<W: WriteU16, R: ReadU16>(&mut self, w: W, r: R) {
    let data = r.read_u16(self);
    w.write_u16(self, data);
    self.step_n(3);
  }

  #[inline]
  fn ld_ix_plus_d_r<R2: ReadU16, R: ReadU8, D: ReadU8>(&mut self, base:R2, r: R, d: D) 
  {
    let offset = d.read_u8(self) as i8;
    let displacement = base.read_u16(self) + offset as u16;
    let value = r.read_u8(self);
    self.mem.w8(displacement, value);
    self.step();
    self.step();
  }

  #[inline]
  fn ld_r_ix_d<R: ReadU8, R16: ReadU16, W: WriteU8>(&mut self, base: R16, d: R, w: W) {
    let offset = d.read_u8(self) as i8;
    let displacement = base.read_u16(self) + offset as u16;
    let value = self.mem.r8(displacement);
    w.write_u8(self, value);
    self.step_n(2);
  }

  #[inline]
  fn ld_16_plus_d_n<R: ReadU16>(&mut self, r: R) 
  {
    let d = self.mem.r8(self.r.pc + 1);
    let n = self.mem.r8(self.r.pc + 2) as u8;
    let displacement = r.read_u16(self) + d as u16;
    self.mem.w8(displacement, n);
    self.step_n(3);
  }

  #[inline]
  fn ld_r_16_d<W8: WriteU8, R16: ReadU16, R: ReadU8>(&mut self, w: W8, baseaddr: R16, d: R) {
    let displacement = d.read_u8(self) as u16;
    let value = baseaddr.read_u16(self);
    let result = self.mem.r8(value.wrapping_add(displacement));
    w.write_u8(self, result);
    self.step_n(3);
  }

  #[inline]
  fn add_a_ix_d<R: ReadU8, R16: ReadU16>(&mut self, baseaddr: R16, d: R) {
    let displacement = d.read_u8(self) as u16;
    let value = baseaddr.read_u16(self);
    let result = self.mem.r8(value.wrapping_add(displacement));
    self.r.a += result;
    self.step_n(3);
  }

  #[inline]
  fn add_ix_pp<R: ReadU16, RW: ReadU16 + WriteU16>(&mut self, r: R, ix: RW) {
    let value = r.read_u16(self);
    let r = (ix.read_u16(self).wrapping_add(value)) & 0x7fff;
    ix.write_u16(self, r);
    self.step();
  }

  #[inline]
  fn rrca<RW: WriteU8 + ReadU8>(&mut self, rw: RW) {
    let value = rw.read_u8(self);
    let r = value.rotate_right(1);
    self.r.f = Flags::CARRY.check(value & 0x01 == 1);
    rw.write_u8(self, r);
    self.step();
  }

  #[inline]
  fn rlca<RW: WriteU8 + ReadU8>(&mut self, rw: RW) {
    let value = rw.read_u8(self);
    let r = value.rotate_left(1);
    self.r.f = Flags::CARRY.check(value & 0x80 != 0);
    rw.write_u8(self, r);
    self.step();
  }

  #[inline]
  fn srl_m<RW: WriteU8 + ReadU8>(&mut self, rw: RW) {
    let value= rw.read_u8(self);
    let r = value.rotate_right(1) & 0x7F;
    self.r.f = Flags::CARRY.check(value & 0x01 == 1) | Flags::ZERO.check(r == 0);
    rw.write_u8(self, r);
    self.step();
  }

  #[inline]
  fn rla<RW: WriteU8 + ReadU8>(&mut self, rw: RW) {
    let c = rw.read_u8(self) & 0b1000_0000;
    let temp = if self.r.f.contains(Flags::CARRY) { 1 } else { 0 };
    let r = rw.read_u8(self).rotate_left(1) | temp;
    self.r.f = Flags::CARRY.check(c != 0) |
        (Flags::ZERO & self.r.f) |  (Flags::SIGN & self.r.f) |
        (Flags::PARITY & self.r.f);
    rw.write_u8(self, r);
    self.step();
  }

  #[inline]
  fn rra<RW: WriteU8 + ReadU8>(&mut self, rw: RW) {
    let c = rw.read_u8(self) & 0b0000_0001;
    let temp = if self.r.f.contains(Flags::CARRY) { 0x80 } else { 0 };
    let r = rw.read_u8(self).rotate_right(1) | temp;
    self.r.f = Flags::CARRY.check(c != 0) |
        (Flags::ZERO & self.r.f) |  (Flags::SIGN & self.r.f) |
        (Flags::PARITY & self.r.f);
    rw.write_u8(self, r);
    self.step();
  }

  #[inline]
  pub fn ret(&mut self) {
    self.increment_sp_by_2();
  }

  #[inline]
  fn increment_sp_by_2(&mut self) {
    let data = self.mem.r16(self.r.sp);
    self.r.sp = self.r.sp.wrapping_add(2);
    self.r.pc = data;
  }

  #[inline]
  fn ret_cc(&mut self, condition: Condition) {
    if condition.check(self.r.f) {
      self.increment_sp_by_2();
    } else {
      self.step();
    }
  }

  #[inline]
  fn sub_r<R: ReadU8>(&mut self, rw: R) {

    let value = rw.read_u8(self);
    let result = self.r.a.wrapping_sub(value);
    
    let mut borrow = false;
    for mask in [0b000_0001, 0b0000_0010, 0b0000_0100, 0b0000_1000,
    0b0001_0000, 0b0010_0000, 0b0100_0000, 0b1000_0000].iter() {
      
      if (value & mask) > (self.r.a & mask) {
        borrow = true;
        break;
      }
    }
    
    let half_carry = (self.r.a & 0xF) + (value & 0xF) > 0xF;
    self.r.f = Flags::ZERO.check(result == 0) | 
    Flags::HALFCARRY.check(half_carry) | Flags::NEGATIVE | 
    Flags::SIGN.check(result & 0x80 != 0) |
    Flags::PARITY.check(result > 0x80) |
    Flags::CARRY.check(borrow);
    
    self.r.a = result;
    self.step();
  }

  #[inline]
  fn and_r<R: ReadU8>(&mut self, rw: R) {
    let value = rw.read_u8(self);
    let result = self.r.a & value;
    self.r.a = result;
    self.r.f = Flags::ZERO.check(result == 0) | 
    Flags::HALFCARRY | Flags::SIGN.check(result & 0x80 != 0);
    
    self.step();
  }

  fn halt(&mut self) {
    self.halted = true;
  }

  ///The contents of any register r' are loaded to any other register r.
  ///r, r' identifies any of the registers A, B, C, D, E, H, or L
  #[inline]
  fn ld_r_r<R: ReadU8, W: WriteU8>(&mut self, w: W, r: R) {
    let value = r.read_u8(self);
    w.write_u8(self, value);

    self.step();
  }

  #[inline]
  fn inc_ss<RW: WriteU16 + ReadU16>(&mut self, w: RW) {
    let value = w.read_u16(self);
    let result = value.wrapping_add(1);
    w.write_u16(self, result);
    self.r.f = Flags::ZERO.check(result == 0) | 
    Flags::HALFCARRY | Flags::SIGN.check(result & 0x80 != 0);
    self.step();
  }

  #[inline]
  fn inc_hl<RW: WriteU8 + ReadU8>(&mut self, rw: RW) {
    let value = rw.read_u8(self);
    rw.write_u8(self, value.wrapping_add(1));
    self.step();
  }

  #[inline]
  fn ld_a_nn<R: ReadU16>(&mut self, a: R) {
    let addr = a.read_u16(self);
    let data = self.mem.r8(addr);
    self.r.a = data;
    self.step_n(3);
  }

  #[inline]
  fn ld_r_hl<W: WriteU8, R: ReadU8>(&mut self, w: W, r: R) {
    let data = r.read_u8(self);
    w.write_u8(self, data);
    self.step();
  }

  #[inline]
  fn ld_nn_hl<R16: ReadU16, R: ReadU8>(&mut self, a: R16, r: R) {
    let address = a.read_u16(self);
    let value = r.read_u8(self);
    self.mem.w8(address, value);
    self.step_n(3);
  }

  #[inline]
  fn cpl(&mut self) {
      self.r.a = !self.r.a;
      self.r.f = Flags::NEGATIVE | Flags::HALFCARRY;
      self.step();
  }

  #[inline]
  fn adc_r<R: ReadU8>(&mut self, r: R) {

    let value = r.read_u8(self);
    let carried = if self.r.f.contains(Flags::CARRY) { 1 } else { 0 };
    let result = self.r.a.wrapping_add(value).wrapping_add(carried);
  
    let carry = self.r.a as u16 + value as u16 + carried as u16 > 0xFF;
    let half_carry = (self.r.a & 0xF) + (value & 0xF) + carried > 0xF;
    self.r.f = Flags::ZERO.check(result == 0) | Flags::PARITY.check(result > 0x80) |
                Flags::HALFCARRY.check(half_carry) | 
                Flags::CARRY.check(carry) | Flags::SIGN.check(result & 0x80 != 0);
                
    self.r.a = result;
    self.step();
  }

  #[inline]
  fn dec_r<RW: ReadU8 + WriteU8>(&mut self, rw: RW) {
      let value = rw.read_u8(self).wrapping_sub(1);
      let a = rw.read_u8(self);

      rw.write_u8(self, value);
      self.r.f = Flags::NEGATIVE | Flags::SIGN.check(value & 0x80 != 0) 
        | Flags::ZERO.check(value == 0) |
        Flags::HALFCARRY.check((a & 0xF) < (value & 0xF)) |
        Flags::PARITY.check(value > 0x80) | 
        (Flags::CARRY & self.r.f);

      self.step();
  }

  #[inline]
  fn dec_ss<RW: ReadU16 + WriteU16>(&mut self, rw: RW) {

    let (r, overflow) = rw.read_u16(self).overflowing_sub(1);
    rw.write_u16(self, r);
    self.r.f = Flags::SIGN.check(r & 0x8000 != 0) | Flags::NEGATIVE
      | Flags::CARRY.check(overflow) | Flags::ZERO.check(r == 0);

    self.step();
  }

  #[inline]
  fn add_a_r<R: ReadU8>( &mut self, r: R) {
      let a = self.r.a as u8;
      let value = r.read_u8(self);
      let (result, carry) = a.overflowing_add(value);
      self.r.a = result;

      let half_carry = (self.r.a & 0xF) + (value & 0xF) > 0xF;
      self.r.f = Flags::ZERO.check(result == 0) |
                 Flags::SIGN.check(result & 0x80 != 0) |
                 Flags::HALFCARRY.check(half_carry) |
                 Flags::PARITY.check(result > 0x80) |
                 Flags::CARRY.check(carry);
      
      self.step();
  }

  #[inline]
  fn push_16<R: ReadU16>( &mut self, r: R) {
    let value = r.read_u16(self);
    self.push(value);
    self.step();
  }

  #[inline]
  fn call_cc_nn<R: ReadU16>(&mut self, condition: Condition, addr: R) {
    if condition.check(self.r.f) {
      let pc = self.r.pc;
      self.push(pc + 3);
      self.r.pc = addr.read_u16(self);
    } else {
      self.step_n(3);
    }
  }

  #[inline]
  fn call(&mut self, addr: u16) {
      let pc = self.r.pc + 3;
      self.push(pc);
      self.r.pc = addr;
  }

  #[inline]
  fn add_hl_ss<WR: ReadU16 + WriteU16, R: ReadU16>(&mut self, r: R, hl: WR) {
    let value = r.read_u16(self);
    let hl_ = hl.read_u16(self);
    let (result, carry) = hl_.overflowing_add(value);
    hl.write_u16(self, result);
    self.r.f = (self.r.f & Flags::ZERO) |
        (self.r.f & Flags::SIGN) |
        (self.r.f & Flags::PARITY) |
        Flags::CARRY.check(carry);
    self.step();
  }

  /// cc   condition relevant  flag
  /// 000  non-zero (nz)         z
  /// 001   zero (z)             z
  /// 010   no carry (nc)        c
  /// 011   carry (c)            c
  /// 100   parity odd (po)     p/v
  /// 101   parity even (pe)    p/v
  /// 110   sign positive (p)    s
  /// 111   sign negative (m)    s
  #[inline]
  pub fn jp_cc_nn<R: ReadU16>(&mut self, condition:Condition, addr: R) {
    if condition.check(self.r.f) {
      self.jmp(addr);
    } else {
      self.step_n(3);
    }
  }

  #[inline]
  pub fn ld_sp_hl<RW: ReadU16 + WriteU16>(&mut self, sp: RW, hl: RW) {
    let value = hl.read_u16(self);
    sp.write_u16(self, value);
    self.step();
  }

  /// if address 4545h contains 37h and address 4546h contains a1h,
  /// then upon the execution of an ld hl, (4545h) instruction,
  /// the hl register pair contains a137h.
  
  #[inline]
  pub fn ld_hl_nn<R: ReadU16>(&mut self, r : R) {
    let value = r.read_u16(self);
      let nn = self.mem.r16(value);
      self.r.h = (nn >> 8) as u8;
      self.r.l = nn as u8;
      self.step_n(3);
  }

  #[inline]
  pub fn xor_r<R: ReadU8>(&mut self, r: R) {
    let value = r.read_u8(self);
    let result = self.r.a ^ value;
    self.r.a  = result;
    self.r.f = Flags::ZERO.check(result == 0) | Flags::SIGN.check(result & 0x80 != 0);
    self.step();
  }

  #[inline]
  pub fn or_r<R: ReadU8>(&mut self, r: R) {
    let value = r.read_u8(self);
    let result = self.r.a | value;
    self.r.f = Flags::ZERO.check(result == 0) | Flags::SIGN.check(result & 0x80 != 0);
    self.step();
  }

  #[inline]
  pub fn rst_p(&mut self, addr: u8) {
      let pc = self.r.pc + 1;
      self.push(pc);
      self.r.pc = addr as u16
  }

  /*
  ld   (hl), r
  the contents of register r are loaded to the memory location specified by the contents of the hl register pair.
  */
  #[inline]
  pub fn ld_hl_r<R: ReadU8, R16: ReadU16>(&mut self, addr: R16, r: R) {
    let address = addr.read_u16(self);
    let value = r.read_u8(self);
    self.mem.w8(address, value);
    self.step();
  }

  #[inline]
  pub fn inc_r<RW: ReadU8 + WriteU8>(&mut self, rw: RW) {
      let value = rw.read_u8(self);
      let result = value.wrapping_add(1);
      rw.write_u8(self, result);

      // update flags
      self.r.f = Flags::ZERO.check(result == 0) |
      Flags::HALFCARRY.check(value & 0xF == 0xF) |
      Flags::SIGN.check(result & 0x80 != 0) |
      (Flags::CARRY & self.r.f);
      self.step();
  }

  #[inline]
  pub fn ld_8_nn<W: WriteU8, R:ReadU8>(&mut self, w: W, r: R) {
      let data = r.read_u8(self);
      w.write_u8(self, data);
      self.step();
      self.step();
  }

  #[inline]
  pub fn jr_conditional(&mut self, condition: Condition) {
      if condition.check(self.r.f) {
          let addr = self.r.pc;
          let offset = self.next_u8() as i8;
          self.r.pc = addr.wrapping_add(offset as u16);
          self.step_n(2);
      } else {
        self.step();
        self.step();
      }
  }

  #[inline]
  pub fn jr_e<R: ReadU8>(&mut self, d: R) {
    let addr = self.r.pc;
    let offset = d.read_u8(self)  as i8;
    self.r.pc = addr.wrapping_add(offset as u16);
    self.step_n(2);
  }

  pub fn nop(&mut self) {
    self.step()
  }

  #[inline]
  pub fn jmp<R: ReadU16>(&mut self, addr: R) {
      //this will make the pc point to the next two bytes of the instruction
      self.r.pc = addr.read_u16(self);
  }

  #[inline]
  pub fn djnz<R: ReadU8>(&mut self, r: R) {
    let pc = self.r.pc;
    let b = self.r.b;
    let displacement = r.read_u8(self) as i8;
    let r = b.wrapping_sub(1);
    if r == 0 {
      self.step();
      self.step();
    } else {
      self.r.pc = pc.wrapping_add(displacement as u16);
      self.step();
      self.step();
    }
    self.r.b = r;
  }
}