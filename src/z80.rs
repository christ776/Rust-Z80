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
///    i       IX
///    r       IY
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
  AF, AF2, BC, DE, HL, SP, IX, IY
};

#[derive(Clone, Copy, Debug)]
pub enum Address {
    BC, DE, HL, NextU16
}

impl ReadU8 for Address {
  fn read_u8(&self, cpu: &mut Z80, mem: &dyn Memory) -> u8 {
      let address = cpu.get_address(self, mem);
      mem.r8(address)
  }
}

impl WriteU8 for Address {
  fn write_u8(&self, cpu: &mut Z80, value: u8, mem: &mut dyn Memory) {
      let address = cpu.get_address(self, mem);
      mem.w8(address, value);
  }
}

impl WriteU16 for Address {
  fn write_u16(&self, cpu: &mut Z80, value: u16, mem: &mut dyn Memory) {
      let address = cpu.get_address(self, mem);
      mem.w16(address, value);
  }
}

pub trait ReadU8 {
  fn read_u8(&self, cpu: &mut Z80, mem: &dyn Memory) -> u8;
}

pub trait WriteU8 {
  fn write_u8(&self, cpu: &mut Z80, value: u8, mem: &mut dyn Memory);
}

pub trait ReadU16 {
  fn read_u16(&self, cpu: &mut Z80, mem: &dyn Memory) -> u16;
}

pub trait WriteU16 {
  fn write_u16(&self, cpu: &mut Z80, value: u16, mem: &mut dyn Memory);
}

pub struct NextU8;
impl ReadU8 for NextU8 {
  fn read_u8(&self, cpu: &mut Z80, mem: &dyn Memory) -> u8 {
      cpu.next_u8(mem)
  }
}

pub struct NextU16;
impl ReadU16 for NextU16 {
  fn read_u16(&self, cpu: &mut Z80, mem: &dyn Memory) -> u16 {
      cpu.next_u16(mem)
  }
}

impl ReadU8 for Register8Bit {
  fn read_u8(&self, cpu: &mut Z80, _: &dyn Memory) -> u8 {
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
  fn write_u8(&self, cpu: &mut Z80, value: u8, _: &mut dyn Memory) {
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
  fn read_u16(&self, cpu: &mut Z80, _: &dyn Memory) -> u16 {
      use Register16Bit::*;
      match *self {
          AF | AF2 | BC | DE | HL | HL2 | IX | IY | BC2  | DE2 => cpu.r.get_u16(*self),
          SP => cpu.r.sp,
      }
  }
}

impl WriteU16 for Register16Bit {
  fn write_u16(&self, cpu: &mut Z80, value: u16, _: &mut dyn Memory) {
      use Register16Bit::*;
      match *self {
          AF | AF2 | BC | DE | HL | HL2 | IX | IY | BC2  | DE2 => cpu.r.set_u16(*self, value),
          SP => cpu.r.sp = value,
      }
  }
}

pub struct Z80 {

  // pub mem: &'a mut dyn Memory,
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
            NEGATIVE => flags.contains(Flags::SIGN),
            POSITIVE => !flags.contains(Flags::SIGN),
            PARITYEVEN => flags.contains(Flags::PARITY),
            PARITYODD => !flags.contains(Flags::PARITY),
        }
    }
}

impl Z80 {
  // pub fn new(memory: & mut (dyn Memory)) -> Z80 {
  pub fn new() -> Z80 {
    Z80{  
          // sp: 0x4FEF,
          r: Registers::new(),
          _vblank_interrupt: false,
          enable_hw_interrupt: false,
          enable_int: false,
          port_a_addr: 0,
          halted: false,
        }
  }

  fn next_u8(&self, mem: &dyn Memory) -> u8 {
    mem.r8(self.r.pc + 1)
  }

  fn next_u16(&self, mem: &dyn Memory) -> u16 {
    mem.r16(self.r.pc + 1)
  }

  fn get_address(&mut self, address: &Address, mem: &dyn Memory) -> u16 {
    use self::Address::*;
    match *address {
      HL => self.r.get_u16(Register16Bit::HL),
      BC => self.r.get_u16(Register16Bit::BC),
      DE => self.r.get_u16(Register16Bit::DE),
      NextU16 => self.next_u16(mem)
    }
  }

  ///This stack usually starts at $0000 so as to place at the very end of memory
  ///(the first push to the stack decrements the stack pointer causing it to wrap around to $FFFF).
  fn push(&mut self, val:u16, mem: &mut dyn Memory) {
    let sp = self.r.sp;
    let addr = sp.wrapping_sub(2);
    self.r.sp = addr;
    mem.w16(addr, val);
  }

  pub fn step(&mut self) {
    self.r.pc += 1;
  }

  pub fn step_n(&mut self, n: u16) {
    self.r.pc += n;
  }

  pub fn exec(&mut self, memory: &mut dyn Memory) -> u8 {

    if self.enable_int && self._vblank_interrupt && self.enable_hw_interrupt {
      //Check for interrupts
      self._vblank_interrupt = false;
      self.enable_hw_interrupt = false;
      self.enable_int = false;
      let interrup_handler_addr = memory.r16((self.r.i as u16) << 8 | self.port_a_addr as u16);
      let pc = self.r.pc;
      self.push(pc, memory);
      self.r.pc = interrup_handler_addr;
    }

    let pc = self.r.pc;
    let op = memory.r8(pc);
  
    match op {
        0x00 => { self.nop(); },
        0x0f => { self.rrca(A, memory) },
        0xf3 => { self.di() },
        0x01 => { self.ld_dd_nn(BC,NextU16, memory) },
        0x11 => { self.ld_dd_nn(DE,NextU16, memory) },
        0x21 => { self.ld_dd_nn(HL,NextU16, memory) },
        0x31 => { self.ld_dd_nn(SP,NextU16, memory) },
        0x07 => {self.rlca(A, memory) },
        0x08 => { self.ex_ss_ss(AF, AF2, memory) },
        0x17 => {self.rla(A, memory) },
        0x1f => {self.rra(A, memory) },
        0x09 => { self.add_hl_ss(HL, BC, memory) },
        0x19 => { self.add_hl_ss(HL, DE, memory) },
        0x29 => { self.add_hl_ss(HL, HL, memory) },
        0x39 => { self.add_hl_ss(HL, SP, memory) },
        0x10 => { self.djnz(NextU8, memory) },
        0x02 => { self.ld_8_nn(Address::BC, A, memory) },
        0x12 => { self.ld_8_nn(Address::DE, A, memory) },
        0x18 => { self.jr_e(NextU8, memory) },
        0x20 => { self.jr_conditional(Condition::NOTZERO, memory) },
        0x30 => { self.jr_conditional(Condition::NOTCARRY, memory) },
        0x38 => { self.jr_conditional(Condition::CARRY, memory) },
        0x28 => { self.jr_conditional(Condition::ZERO, memory) },
        0x37 => { self.scf() }
        0x27 => { self.daa() }
        0x5 => { self.dec_r(B, memory) },
        0x0d => { self.dec_r(C, memory) },
        0x15 => { self.dec_r(D, memory) },
        0x1d => { self.dec_r(E, memory) },
        0x25 => { self.dec_r(H, memory) },
        0x2d => { self.dec_r(L, memory) },
        0x3d => { self.dec_r(A, memory) },
        0x35 => { self.dec_r(Address::HL, memory) },
        0x03 => {self.inc_ss(BC, memory)},
        0x13 => {self.inc_ss(DE, memory)},
        0x23 => {self.inc_ss(HL, memory)},
        0x33 => {self.inc_ss(SP, memory)},
        0x34 => {self.inc_r(Address::HL, memory)},
        0x2a => { self.ld_hl_nn(NextU16, memory) },
        0x0b => { self.dec_ss(BC, memory) },
        0x1b => { self.dec_ss(DE, memory) },
        0x2b => { self.dec_ss(HL, memory) },
        0x3b => { self.dec_ss(SP, memory) },
        0x04 => { self.inc_r(B, memory) },
        0x0c => { self.inc_r(C, memory) },
        0x14 => { self.inc_r(D, memory) },
        0x1c => { self.inc_r(E, memory) },
        0x24 => { self.inc_r(H, memory) },
        0x2c => { self.inc_r(L, memory) },
        0x3c => { self.inc_r(A, memory) },
        0x2f => { self.cpl() },
        0x32 => { self.ld_nn_a(A, NextU16, memory)},
        0x22 => { self.ld_nn_hl(NextU16, HL, memory)},
        0x36 => { 
          self.ld_hl_r(Register16Bit::HL, NextU8, memory);
          self.step();
        },
        0x3a => { self.ld_a_nn(NextU16, memory)},
        0x06 => { 
          self.ld_r_r(B, NextU8, memory);
          self.step();
        },
        0x0e => { 
          self.ld_r_r(C, NextU8, memory);
          self.step();
        },
        0x16 => { 
          self.ld_r_r(D, NextU8, memory);
          self.step();
        },
        0x1e => { 
          self.ld_r_r(E, NextU8, memory);
          self.step();
        },
        0x26 => { 
          self.ld_r_r(H, NextU8, memory);
          self.step();
        },
        0x2e => { 
          self.ld_r_r(L, NextU8, memory);
          self.step();
        },
        0x3e => { 
          self.ld_r_r(A, NextU8, memory);
          self.step();
        },
        0xb6 => { self.or_r(Address::HL, memory) },
        0x88 => { self.adc_r(B, memory)},
        0x89 => { self.adc_r(C, memory)},
        0x8a => { self.adc_r(D, memory)},
        0x8b => { self.adc_r(E, memory)},
        0x8c => { self.adc_r(H, memory)},
        0x8d => { self.adc_r(L, memory)},
        0x8e => { self.adc_r(Address::HL, memory)},
        0x8f => { self.adc_r(A, memory)} 
        0xce => { 
          self.adc_r(NextU8, memory);
          self.step();
         },
        0x6f => { self.ld_r_r(L,A, memory)},
        0x44 => { self.ld_r_r(B,H, memory)},
        0x40 => { self.ld_r_r(B,B, memory)},
        0x47 => { self.ld_r_r(B,A, memory)},
        0x48 => { self.ld_r_r(C,B, memory)},
        0x49 => { self.ld_r_r(C,C, memory)},
        0x4a => { self.ld_r_r(C,D, memory)},
        0x4c => { self.ld_r_r(C,H, memory)},
        0x4f => { self.ld_r_r(C,A, memory)},
        0x50 => { self.ld_r_r(D,B, memory)},
        0x51 => { self.ld_r_r(D,C, memory)},
        0x52 => { self.ld_r_r(D,D, memory)},
        0x53 => { self.ld_r_r(D,E, memory)},
        0x54 => { self.ld_r_r(D,H, memory)},
        0x55 => { self.ld_r_r(D,L, memory)},
        0x5b => { self.ld_r_r(E,E, memory)},
        0x5c => { self.ld_r_r(C,H, memory)}, 
        0x5d => { self.ld_r_r(E,L, memory)},
        0x5f => { self.ld_r_r(E,A, memory)},
        0x58 => { self.ld_r_r(C,B, memory)},
        0x61 => { self.ld_r_r(H,C, memory)},
        0x62 => { self.ld_r_r(H,D, memory)},
        0x63 => { self.ld_r_r(H,E, memory)},
        0x64 => { self.ld_r_r(H,H, memory)},
        0x65 => { self.ld_r_r(H,L, memory)},
        0x67 => { self.ld_r_r(H,A, memory)},
        0x68 => { self.ld_r_r(H,A, memory)},
        0x6a => { self.ld_r_r(L,D, memory)},
        0x6c => { self.ld_r_r(L,H, memory)},
        0x78 => { self.ld_r_r(A,B, memory)},
        0x79 => { self.ld_r_r(A,C, memory)},
        0x7a => { self.ld_r_r(A,D, memory)},
        0x7b => { self.ld_r_r(A,E, memory)},
        0x7c => { self.ld_r_r(A,H, memory)},
        0x7d => { self.ld_r_r(A,L, memory)},
        0x4e => { self.ld_r_hl(C, Address::HL, memory)},
        0x1a => { self.ld_r_hl(A, Address::DE, memory)},
        0x0a => { self.ld_r_hl(A, Address::BC, memory)},
        0x46 => { self.ld_r_hl(B, Address::HL, memory)},
        0x56 => { self.ld_r_hl(D, Address::HL, memory)},
        0x5e => { self.ld_r_hl(E, Address::HL, memory)},
        0x66 => { self.ld_r_hl(H, Address::HL, memory)},
        0x6e => { self.ld_r_hl(L, Address::HL, memory)},
        0x7e => { self.ld_r_hl(A, Address::HL, memory)},
        0x70 => { self.ld_hl_r(Register16Bit::HL,B, memory)},
        0x71 => { self.ld_hl_r(Register16Bit::HL,C, memory)},
        0x72 => { self.ld_hl_r(Register16Bit::HL,D, memory)},
        0x73 => { self.ld_hl_r(Register16Bit::HL,E, memory)},
        0x74 => { self.ld_hl_r(Register16Bit::HL,H, memory)},
        0x75 => { self.ld_hl_r(Register16Bit::HL,L, memory)},
        0x77 => { self.ld_hl_r(Register16Bit::HL,A, memory)},
        0x76 => { self.halt()},
        0x80 => { self.add_a_r(B, memory)},
        0x81 => { self.add_a_r(C, memory)},
        0x82 => { self.add_a_r(D, memory)},
        0x83 => { self.add_a_r(E, memory)},
        0x84 => { self.add_a_r(H, memory)},
        0x85 => { self.add_a_r(L, memory)},
        0x86 => { self.add_a_r(Address::HL, memory)},
        0x87 => { self.add_a_r(A, memory)}, 
        0x90 => { self.sub_r(B, memory) },
        0x91 => { self.sub_r(C, memory) },
        0x92 => { self.sub_r(D, memory) },
        0x93 => { self.sub_r(E, memory) },
        0x94 => { self.sub_r(H, memory) },
        0x95 => { self.sub_r(L, memory) },
        0x96 => { self.sub_r(Address::HL, memory) },
        0x97 => { self.sub_r(A, memory) },
        0x98 => { self.sbc_r(B, memory)},
        0x99 => { self.sbc_r(C, memory)},
        0x9A => { self.sbc_r(D, memory)},
        0x9B => { self.sbc_r(E, memory)},
        0x9C => { self.sbc_r(H, memory)},
        0x9D => { self.sbc_r(L, memory)},
        0x9E => { self.sbc_r(Address::HL, memory)},
        0x9F => { self.sbc_r(A, memory)},
        0xDE => { self.sbc_r(NextU8, memory); self.step(); },
        0xa0 => {self.and_r(B, memory)},
        0xa1 => {self.and_r(C, memory)},
        0xa2 => {self.and_r(D, memory)},
        0xa3 => {self.and_r(E, memory)}
        0xa4 => {self.and_r(H, memory)}
        0xa5 => {self.and_r(L, memory)},
        0xa6 => {self.and_r(Address::HL, memory)},
        0xa7 => {self.and_r(A, memory)},
        0xe6 => { 
          self.and_r(NextU8, memory);
          self.step();
        },
        0xa8 => { self.xor_r(B, memory) },
        0xa9 => { self.xor_r(C, memory) },
        0xaa => { self.xor_r(D, memory) },
        0xab => { self.xor_r(E, memory) },
        0xac => { self.xor_r(H, memory) },
        0xad => { self.xor_r(L, memory) },
        0xaf => { self.xor_r(A, memory) },
        0xae => { self.xor_r(Address::HL, memory) },
        0xee => { 
          self.xor_r(NextU8, memory);
          self.step();
        },
        0xb0 => { self.or_r(B, memory) },
        0xb1 => { self.or_r(C, memory) },
        0xb2 => { self.or_r(D, memory) },
        0xb3 => { self.or_r(E, memory) },
        0xb4 => { self.or_r(H, memory) },
        0xb5 => { self.or_r(L, memory) },
        0xb7 => { self.or_r(A, memory) },
        0xc0 => {self.ret_cc(Condition::NOTZERO, memory)},
        0xc8 => {self.ret_cc(Condition::ZERO, memory)},
        0xd0 => {self.ret_cc(Condition::NOTCARRY, memory)},
        0xd8 => {self.ret_cc(Condition::CARRY, memory)},
        0xe0 => {self.ret_cc(Condition::PARITYODD, memory)},
        0xe8 => {self.ret_cc(Condition::PARITYEVEN, memory)},
        0xf0 => {self.ret_cc(Condition::POSITIVE, memory)},
        0xf8 => {self.ret_cc(Condition::NEGATIVE, memory)},
        0xc3 => { self.jmp(NextU16, memory); },
        0xc6 => {
           self.add_a_r(NextU8, memory); 
           self.step();
        },
        0xc9 => {self.ret(memory)},
        0xcb => { 
          let op = self.next_u8(memory);
          self.step();
          match op {
            0x20 => { self.sla_m(B, memory) },
            0x21 => { self.sla_m(C, memory) },
            0x22 => { self.sla_m(D, memory) },
            0x23 => { self.sla_m(E, memory) },
            0x24 => { self.sla_m(H, memory) },
            0x25 => { self.sla_m(L, memory) },
            0x27 => { self.sla_m(A, memory) },
            0x46 => { self.bit(0, Address::HL, memory) },
            0x47 => { self.bit(0, A, memory) },
            0x4e => { self.bit(1, Address::HL, memory) },
            0x56 => { self.bit(2, Address::HL, memory) },
            0x5e => { self.bit(3, Address::HL, memory) },
            0x66 => { self.bit(4, Address::HL, memory) },
            0x6e => { self.bit(5, Address::HL, memory) },
            0x76 => { self.bit(6, Address::HL, memory) },
            0x7e => { self.bit(7, Address::HL, memory) },
            0xC0..=0xff => {
              let bit = (op & 0x38) >> 3;
              let r = op & 0x07;
              match r {
                0x0 =>  self.set_r(bit, B, memory),
                0x1 =>  self.set_r(bit, C, memory),
                0x2 =>  self.set_r(bit, D, memory),
                0x4 =>  self.set_r(bit, H, memory),
                0x3 =>  self.set_r(bit, E, memory),
                0x5 =>  self.set_r(bit, L, memory),
                0x7 =>  self.set_r(bit, A, memory),
                _ => {}
              }
            }
            0x38 => { self.srl_m(B, memory) },
            0x39 => { self.srl_m(C, memory) },
            0x3a => { self.srl_m(D, memory) },
            0x3b => { self.srl_m(E, memory) },
            0x3c => { self.srl_m(H, memory) },
            0x3d => { self.srl_m(L, memory) },
            0x3f => { self.srl_m(A, memory) }
            0| 1 | 2 | 3 | 4 | 5 | 7 => {
              let r = op & 0x07;
              match r {
                0x0 =>  self.rlca(B, memory),
                0x1 =>  self.rlca(C, memory),
                0x2 =>  self.rlca(D, memory),
                0x4 =>  self.rlca(H, memory),
                0x3 =>  self.rlca(E, memory),
                0x5 =>  self.rlca(L, memory),
                0x7 =>  self.rlca(A, memory),
                _ => {}
              }
            }
            0x0E => { self.rrca(Address::HL, memory)}
            0x06 => { self.rlca(Address::HL, memory)}
            // 0x10 => { }
            _ => {  panic!("unknown opcode {}! at {}", format!("{:#x}", op), format!("{:#x}", self.r.pc)); }

          }
         },
        0xcd => { 
          let addr = self.next_u16(memory);
          self.call(addr, memory); 
        },
        0xd3 => { self.out(NextU8, memory) },
        0xd6 => { self.sub_r(NextU8, memory); self.step(); },
        0xf6 => { self.or_r(NextU8, memory); self.step(); },
        0xdd => {
          let op = self.next_u8(memory);
          let ix = self.r.get_u16(Register16Bit::IX);
          let ix_h = (ix >> 8) as u8;
          let ix_l = (ix & 0x00ff) as u8;
          self.step();
          match op {
            0x21 => { self.ld_dd_nn(IX, NextU16, memory) },
            0x22 => { self.ld_nn_hl(NextU16, IX, memory)},
            0x23 => {self.inc_ss(IX, memory)},
            0x2a => { self.ld_dd_nn_content(IX, NextU16, memory)},
            0x2b => {self.dec_ss(IX, memory)},
            0x34 => { self.inc_r_d(IX, NextU8 , memory)}
            0x70 => { self.ld_16_plus_d_r(IX, B, NextU8, memory) }
            0x71 => { self.ld_16_plus_d_r(IX, C, NextU8, memory) }
            0x72 => { self.ld_16_plus_d_r(IX, D, NextU8, memory) }
            0x73 => { self.ld_16_plus_d_r(IX, E, NextU8, memory) }
            0x74 => { self.ld_16_plus_d_r(IX, H, NextU8, memory) }
            0x75 => { self.ld_16_plus_d_r(IX, L, NextU8, memory) }
            0x77 => { self.ld_16_plus_d_r(IX, A, NextU8, memory) }
            0x84 => { self.add_a_n(ix_h) }
            0x85 => { self.add_a_n(ix_l) },
            0x8c => { self.adc_n(ix_h) }
            0x8d => { self.adc_n(ix_l) }
            0x8e => { self.adc_ixy_d(IX, NextU8, memory) }
            0x94 => { self.sub_n(ix_h) }
            0x95 => { self.sub_n(ix_l) }
            0x96 => { self.sub_ixy_d(IX, NextU8, memory) }
            0x9C => { self.sbc_n(ix_h) }
            0x9D => { self.sbc_n(ix_l) }
            0x9E => { self.sbc_ixy_d(IX, NextU8, memory) }
            0xA4 => { self.and_n(ix_h) }
            0xA5 => { self.and_n(ix_h) }
            0xA6 => { self.and_ixy_d(IX, NextU8, memory)}
            0xAC => { self.xor_n(ix_h) }
            0xAD => { self.xor_n(ix_l) }
            0xAE => { self.xor_ixy_d(IX, NextU8, memory)}
            0xB4 => { self.or_n(ix_h) }
            0xB5 => { self.or_n(ix_l) }
            0xB6 => { self.or_ixy_d(IX, NextU8, memory)}
            0xBC => { self.cp_n(ix_h) }
            0xBD => { self.cp_n(ix_l) }
            0xBE => { self.cp_ixy_d(IX, NextU8, memory)}
            0x46 => { self.ld_r_ix_d(IX, NextU8, B, memory) },
            0x4e => { self.ld_r_ix_d(IX, NextU8, C, memory) },
            0x56 => { self.ld_r_ix_d(IX, NextU8, D, memory) },
            0x5e => { self.ld_r_ix_d(IX, NextU8, E, memory) },
            0x66 => { self.ld_r_ix_d(IX, NextU8, H, memory) },
            0x6e => { self.ld_r_ix_d(IX, NextU8, L, memory) },
            0x7e => { self.ld_r_ix_d(IX, NextU8, A, memory) },
            0x36 => { self.ld_16_plus_d_n(IX, NextU16, memory) },
            0x86 => { self.add_a_ix_d(IX, NextU8, memory) },
            0x09 => { self.add_hl_ss(IX, BC, memory) },
            0x19 => { self.add_hl_ss(IX, DE, memory) },
            0x29 => { self.add_hl_ss(IX, IX, memory) },
            0x39 => { self.add_hl_ss(IX, SP, memory) },
            0xcb => { self.bit_ix_d(7, IX, NextU8, memory)}
            0xE5 => { self.push_16(IX, memory)},
            0xE1 => { self.pop(IX, memory)},
            0xE9 => { self.jmp(IX, memory) },
            0xf9 => { self.ld_sp_hl(SP, IX, memory) },
            0x35 => { self.dec_ix_n(IX, NextU8, memory) }
            _ => {  panic!("unknown opcode 0xdd{}! at {}", format!("{:#x}", op), format!("{:#x}", self.r.pc)); }
          }
        },
        0xeb => {self.ex_ss_ss(DE, HL, memory)},
        0xed => {
            let op = self.next_u8(memory);
            self.step();
            match op {
              0x42 => { self.sbc_hl_ss(BC, memory) },
              0x52 => { self.sbc_hl_ss(DE, memory) },
              0x62 => { self.sbc_hl_ss(HL, memory) },
              0x72 => { self.sbc_hl_ss(SP, memory) },
              0x43 => { self.ed(BC, NextU16, memory) },
              0x44 => { self.neg() }
              0x53 => { self.ed(DE, NextU16, memory) },
              0x63 => { self.ed(HL, NextU16, memory) },
              0x73 => { self.ed(SP, NextU16, memory) },
              0x4b => { self.ld_dd_nn_content(BC, NextU16, memory) },
              0x5b => { self.ld_dd_nn_content(DE, NextU16, memory) },
              0x6b => { self.ld_dd_nn_content(HL, NextU16, memory) },
              0x7b => { self.ld_dd_nn_content(SP, NextU16, memory) },
              0x47 => { self.ld_r_r(I, A, memory) },
              0x4a => { self.adc_16(BC, memory) },
              0x5a => { self.adc_16(DE, memory) },
              0x6a => { self.adc_16(HL, memory) },
              0x7a => { self.adc_16(SP, memory) },
              0x56 => { self.im1() },
              0x5e => { self.im2() },
              0xa1 => { self.cpi(memory) },
              0xb0 => { self.ldir(HL, DE, BC, memory)}
              _ => {  panic!("unknown opcode {}! at {}", format!("{:#x}", op), format!("{:#x}", self.r.pc)); }
            }
        },
        0xfd => {
          let op = self.next_u8(memory);
          let iy = self.r.get_u16(Register16Bit::IY);
          let iy_l = (iy & 0x00ff) as u8;
          let iy_h = (iy >> 8) as u8;
          self.step();
          match op {
            0x22 => { self.ld_nn_hl(NextU16, IY, memory)},
            0x23 => {self.inc_ss(IY, memory)},
            0x2a => { self.ld_dd_nn_content(IY, NextU16, memory)},
            0x2b => {self.dec_ss(IY, memory)},
            0x09 => { self.add_hl_ss(IY, BC, memory) },
            0x19 => { self.add_hl_ss(IY, DE, memory) },
            0x29 => { self.add_hl_ss(IY, IY, memory) },
            0x39 => { self.add_hl_ss(IY, SP, memory) },
            0x21 => { self.ld_dd_nn(IY, NextU16, memory) },
            0x34 => { self.inc_r_d(IY, NextU8 , memory)},
            0x35 => { self.dec_ix_n(IY, NextU8, memory) }
            0x36 => { self.ld_16_plus_d_n(IY, NextU16, memory) },
            0x46 => { self.ld_r_16_d(B, IY, NextU8, memory) },
            0x4e => { self.ld_r_16_d(C, IY, NextU8, memory) },
            0x56 => { self.ld_r_16_d(D, IY, NextU8, memory) },
            0x5e => { self.ld_r_16_d(E, IY, NextU8, memory) },
            0x66 => { self.ld_r_16_d(H, IY, NextU8, memory) },
            0x6e => { self.ld_r_16_d(L, IY, NextU8, memory) },
            0x70 => { self.ld_16_plus_d_r(IY, B, NextU8, memory) },
            0x71 => { self.ld_16_plus_d_r(IY, C, NextU8, memory) },
            0x72 => { self.ld_16_plus_d_r(IY, D, NextU8, memory) },
            0x73 => { self.ld_16_plus_d_r(IY, E, NextU8, memory) },
            0x74 => { self.ld_16_plus_d_r(IY, H, NextU8, memory) },
            0x75 => { self.ld_16_plus_d_r(IY, L, NextU8, memory) },
            0x77 => { self.ld_16_plus_d_r(IY, A, NextU8, memory) }
            0x7e => { self.ld_r_16_d(A, IY, NextU8, memory) },
            0x84 => { self.add_a_n(iy_h) }
            0x85 => { self.add_a_n(iy_l) }
            0x8c => { self.adc_n(iy_h) }
            0x8d => { self.adc_n(iy_l) }
            0x8e => { self.adc_ixy_d(IY, NextU8, memory) }
            0x94 => { self.sub_n(iy_h) }
            0x95 => { self.sub_n(iy_l) }
            0x96 => { self.sub_ixy_d(IY, NextU8, memory) }
            0x9C => { self.sbc_n(iy_h) }
            0x9D => { self.sbc_n(iy_l) }
            0x9E => { self.sbc_ixy_d(IY, NextU8, memory) }
            0xA4 => { self.and_n(iy_h) }
            0xA5 => { self.and_n(iy_h) }
            0xA6 => { self.and_ixy_d(IY, NextU8, memory)}
            0xAC => { self.xor_n(iy_h) }
            0xAD => { self.xor_n(iy_l) }
            0xAE => { self.xor_ixy_d(IY, NextU8, memory)}
            0xB4 => { self.or_n(iy_h) }
            0xB5 => { self.or_n(iy_l) }
            0xB6 => { self.or_ixy_d(IY, NextU8, memory)}
            0xBC => { self.cp_n(iy_h) }
            0xBD => { self.cp_n(iy_l) }
            0xBE => { self.cp_ixy_d(IY, NextU8, memory)}
            0x86 => { self.add_a_ix_d(IY, NextU8, memory) },
            0xE5 => { self.push_16(IY, memory)},
            0xE1 => { self.pop(IY, memory)},
            0xE9 => { self.jmp(IY, memory) },
            0xf9 => { self.ld_sp_hl(SP, IY, memory) },
            _ => {  panic!("unknown opcode {}! at {}", format!("{:#x}", op), format!("{:#x}", self.r.pc)); }
          }
        },
        0xd9 => { self.exx() },
        0xc1 => {self.pop(BC, memory)},
        0xd1 => {self.pop(DE, memory)},
        0xe1 => {self.pop(HL, memory)},
        0xf1 => {self.pop(AF, memory)},
        0xc2 => { self.jp_cc_nn(Condition::NOTZERO, NextU16, memory) },
        0xca => { self.jp_cc_nn(Condition::ZERO, NextU16, memory) },
        0xd2 => { self.jp_cc_nn(Condition::NOTCARRY, NextU16, memory) },
        0xda => { self.jp_cc_nn(Condition::CARRY, NextU16, memory) },
        0xe2 => { self.jp_cc_nn(Condition::PARITYODD, NextU16, memory) },
        0xea => { self.jp_cc_nn(Condition::PARITYEVEN, NextU16, memory) },
        0xf2 => { self.jp_cc_nn(Condition::POSITIVE, NextU16, memory) },
        0xfa => { self.jp_cc_nn(Condition::NEGATIVE, NextU16, memory) },
        0xe9 => { self.jmp(Register16Bit::HL, memory) },
        0xc4 => { self.call_cc_nn(Condition::NOTZERO, NextU16, memory) },
        0xcc => { self.call_cc_nn(Condition::ZERO, NextU16, memory) },
        0xd4 => { self.call_cc_nn(Condition::NOTCARRY, NextU16, memory) },
        0xdc => { self.call_cc_nn(Condition::CARRY, NextU16, memory) },
        0xe4 => { self.call_cc_nn(Condition::PARITYODD, NextU16, memory) },
        0xec => { self.call_cc_nn(Condition::PARITYEVEN, NextU16, memory) },
        0xf4 => { self.call_cc_nn(Condition::POSITIVE, NextU16, memory) },
        0xfc => { self.call_cc_nn(Condition::NEGATIVE, NextU16, memory) },
        0xc5 => { self.push_16(BC, memory) },
        0xd5 => { self.push_16(DE, memory) },
        0xe5 => { self.push_16(HL, memory) },
        0xf5 => { self.push_16(AF, memory) },
        0xc7 => { self.rst_p(0x00, memory) },
        0xcf => { self.rst_p(0x08, memory) },
        0xd7 => { self.rst_p(0x10, memory) },
        0xdf => { self.rst_p(0x18, memory) },
        0xe7 => { self.rst_p(0x20, memory) },
        0xef => { self.rst_p(0x28, memory) },
        0xf7 => { self.rst_p(0x30, memory) },
        0xff => { self.rst_p(0x38, memory) },
        0xf9 => { self.ld_sp_hl(SP, HL, memory) },
        0xfb => { self.ei() },
        0xfe => { 
          self.cp(NextU8, memory);
          self.step();
        },
        // CP
        0xBF => self.cp(A, memory),
        0xB8 => self.cp(B, memory),
        0xB9 => self.cp(C, memory),
        0xBA => self.cp(D, memory),
        0xBB => self.cp(E, memory),
        0xBC => self.cp(H, memory),
        0xBD => self.cp(L, memory),
        0xBE => self.cp(Address::HL, memory),
        _ => {  panic!("unknown opcode {}! at {}", format!("{:#x}", op), format!("{:#x}", self.r.pc)); },
    }
    16
  }

  #[inline]
  fn daa(&mut self) {
    let mut a_l = self.r.a & 0x0f;
    let mut a_h = self.r.a & 0xf0;
    let mut half_carry = false;
    if a_l > 9 || self.r.f.contains(Flags::HALFCARRY) {
      a_l= a_l + 0x06;
      half_carry = true;
    }
    let mut carry = false;
    if a_h > 9 || self.r.f.contains(Flags::CARRY) {
      a_h = a_h + 0x60;
      carry = true;
    }

    self.r.a = a_h | a_l;
    self.r.f = 
        Flags::CARRY.check(carry) |
        Flags::HALFCARRY.check(half_carry)  |
        Flags::SIGN.check(self.r.a & 0x80 != 0) |
        (Flags::NEGATIVE & self.r.f) |
        Flags::PARITY.check(self.r.a.count_ones() & 1 == 0) |
        Flags::ZERO.check(self.r.a == 0);
    self.step();
  }

  #[inline]
  fn scf(&mut self) {
    let current_flags = self.r.f.bits();
    self.r.f = Flags::from_bits_truncate(current_flags | 1);
    self.step();
  }

  #[inline]
  fn cpi(&mut self, mem: &dyn Memory) {
    let hl = self.r.get_u16(Register16Bit::HL);
    let bc = self.r.get_u16(Register16Bit::BC);
    let value = mem.r8(hl);
    let result = self.r.a.wrapping_sub(value);

    self.r.set_u16(Register16Bit::HL, hl + 1);
    self.r.set_u16(Register16Bit::BC, bc - 1);
    self.r.f = Flags::ZERO.check(result == 0) |
        Flags::NEGATIVE | 
        Flags::SIGN.check(result & 0x80 != 0) |
        (Flags::CARRY & self.r.f) |
        Flags::PARITY.check(bc - 1 != 0);
    self.step();
  }

  #[inline]
  fn sbc_hl_ss<RW: ReadU16>(&mut self, ss: RW, mem: &dyn Memory) {
    let hl = self.r.get_u16(Register16Bit::HL);
    let value = ss.read_u16(self, mem);
    let carried = if self.r.f.contains(Flags::CARRY) { 1 } else { 0 };
    let result = hl.wrapping_sub(value + carried);

    let carry = (hl as usize) < (value as usize) + (carried as usize);

    self.r.set_u16(Register16Bit::HL, result);
    self.r.f = Flags::ZERO.check(result == 0) |
        Flags::NEGATIVE | 
        Flags::SIGN.check(result & 0x8000 != 0) |
        Flags::CARRY.check(carry) |
        Flags::PARITY.check(((hl as i16) < 0) && (result as i16 > 0));
    self.step();
  }

  #[inline]
  fn sbc_r<R: ReadU8>(&mut self, r: R, mem: &dyn Memory) {
    let value = r.read_u8(self, mem);
    self.sbc_n(value);
  }

  #[inline]
  fn sbc_ixy_d<R16: ReadU16, D: ReadU8>(&mut self, r: R16, d: D, mem: &dyn Memory) {

    let base_addr = r.read_u16(self, mem);
    let displacement = d.read_u8(self, mem) as i8;
    let value = mem.r8(base_addr + displacement as u16);
    self.sbc_n(value);
    self.step();
  }

  #[inline]
  fn sbc_n(&mut self, n: u8) {
    let a = self.r.a;
    let carried = if self.r.f.contains(Flags::CARRY) { 1 } else { 0 };
    let result = a.wrapping_sub(n).wrapping_sub(carried);
    let half_carry = (self.r.a & 0xF) < (n & 0xF) + carried;
    let carry = (self.r.a as u16) < (n as u16) + (carried as u16);
    self.r.a = result;
    self.r.f = Flags::ZERO.check(result == 0) |
    Flags::NEGATIVE | Flags::SIGN.check(result & 0x80 != 0) |
        Flags::CARRY.check(carry) |
        Flags::PARITY.check(((a as i8) < 0) && (result as i8 > 0)) |
        Flags::HALFCARRY.check(half_carry);
    self.step();
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
  fn out<R: ReadU8>(&mut self, r: R, mem: &dyn Memory) {
    let n = r.read_u8(self, mem);
    if n == 0 {
      self.port_a_addr = self.r.a;
    }
    self.step_n(2);
  }

  fn im2(&mut self) {
    self.step();
  }

  fn im1(&mut self) {
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
  fn bit<R: ReadU8>(&mut self, bit: u8, r: R, mem: &dyn Memory) {
    let value = r.read_u8(self, mem);
    let mask = 1 << bit;
    self.r.f = Flags::ZERO.check((value & mask) == 0) |
                Flags::HALFCARRY |
                (Flags::CARRY & self.r.f);
    self.step();
  }

  #[inline]
  fn set_r<RW: ReadU8 + WriteU8>(&mut self, bit: u8, r: RW, mem: &mut dyn Memory) {
    let value = r.read_u8(self, mem);
    let mask = 1 << bit;
    r.write_u8(self, value | mask, mem);
    self.step();
  }

  #[inline]
  fn bit_ix_d<R: ReadU16, D: ReadU8>(&mut self, bit: u8, r: R, d:D, mem: &dyn Memory) {
    let offset = d.read_u8(self, mem) as i8;
    let displacement = r.read_u16(self, mem) + offset as u16;
    let value = mem.r8(displacement);
    let mask = 1 << bit;
    self.r.f = Flags::ZERO.check((value & mask) == 0) |
                Flags::HALFCARRY |
                (Flags::CARRY & self.r.f);
    self.step_n(3);
  }

  #[inline]
  fn sla_m<RW: ReadU8 + WriteU8>(&mut self, r: RW, mem: &mut dyn Memory) {
    let c = r.read_u8(self, mem) & 0x80 != 0;
    let result = r.read_u8(self, mem) << 1;
    self.r.f = Flags::CARRY.check(c) |
      Flags::SIGN.check(result & 0x80 != 0) |
      Flags::PARITY.check(result.count_ones() & 1 == 0) |
      Flags::ZERO.check(result == 0);
    r.write_u8(self, result, mem);
    self.step();
  }

  #[inline]
  fn cp_ixy_d<R16: ReadU16, D: ReadU8>(&mut self, r: R16, d: D, mem: &dyn Memory) {

    let base_addr = r.read_u16(self, mem);
    let displacement = d.read_u8(self, mem) as i8;
    let value = mem.r8(base_addr + displacement as u16);
    self.cp_n(value);
    self.step();
  }

  #[inline]
  fn cp<R: ReadU8>(&mut self, r: R, mem: &dyn Memory) {
    let value = r.read_u8(self, mem);
    self.cp_n(value);
  }

  #[inline]
  fn cp_n(&mut self, n: u8) {
    let result = self.r.a.wrapping_sub(n);

    self.r.f = Flags::ZERO.check(result == 0) |
                Flags::NEGATIVE |
                Flags::SIGN.check(result & 0x80 != 0) |
                Flags::HALFCARRY.check((self.r.a & 0xF) < (n & 0xF)) |
                Flags::PARITY.check(result >= 0x80 || result <= 0x81) | 
                Flags::CARRY.check(self.r.a < n);
    self.step();
  }

  /**
   * Extended Instructions, see more at http://clrhome.org/table/
   */
  fn ed<R: ReadU16, RW: ReadU16 + WriteU16>(&mut self, r: RW, addr: R, mem: &mut dyn Memory) {
    let value = r.read_u16(self, mem);
    let a = addr.read_u16(self, mem);
    mem.w16(a, value);
    self.step_n(3);
  }

  /** Transfers a byte of data from the memory location pointed to by hl to the memory location pointed to by de.
      then hl and de are incremented and bc is decremented. If bc is not zero, this operation is repeated.
      Interrupts can trigger while this instruction is processing.
  */
  #[inline]
  fn ldir<RW: ReadU16 + WriteU16>(&mut self, hl: RW, de: RW, bc: RW, mem: &mut dyn Memory) {
    
    loop {
      let bc_ = bc.read_u16(self, mem);
      if bc_ == 0 { break }
      let hl_ = hl.read_u16(self, mem);
      let de_ = de.read_u16(self, mem);
      mem.w8(de_, mem.r8(hl_));
      hl.write_u16(self, hl_.wrapping_add(1), mem);
      de.write_u16(self, de_.wrapping_add(1), mem);
      
      bc.write_u16(self, bc_.wrapping_sub(1), mem);
    }
    self.r.f = (Flags::ZERO & self.r.f) | (Flags::CARRY & self.r.f) |
        (Flags::SIGN & self.r.f);
    self.step();
  }

  #[inline]
  fn ex_ss_ss<RW: ReadU16 + WriteU16>(&mut self, de: RW, hl: RW, mem: &mut dyn Memory) {
    let de_ = de.read_u16(self, mem);
    let hl_ = hl.read_u16(self, mem);
    de.write_u16(self, hl_, mem);
    hl.write_u16(self, de_, mem);
    self.step();
  }

  #[inline]
  fn ld_nn_a<R: ReadU8, R16: ReadU16>(&mut self, r: R, a: R16, mem: &mut dyn Memory) {
    let addr = a.read_u16(self, mem);
    let value = r.read_u8(self, mem);
    if addr == 0x5000 {
        let enable_int = value & 0x01;
        self.enable_hw_interrupt = enable_int != 0;
    } else {
      mem.w8(addr, value);
    }
    self.step_n(3);
  }

  #[inline]
  fn dec_ix_n<R: ReadU16, D: ReadU8>(&mut self, r: R, d: D, mem: &mut dyn Memory) {
    let offset = d.read_u8(self, mem) as i8;
    let displacement = r.read_u16(self, mem).wrapping_add(offset as u16);
    let data = mem.r8(displacement);
    let r = data.wrapping_sub(1);
    mem.w8(displacement, r);
    self.r.f = Flags::NEGATIVE | Flags::ZERO.check(r == 0) |
      Flags::HALFCARRY.check((data & 0xF) < (r & 0xF)) |
      Flags::SIGN.check((r & 0x80) == 0x80);
    self.step_n(2);
  }

  #[inline]
  fn pop<W: WriteU16>(&mut self, dest: W, mem: &mut dyn Memory) {
    let sp = self.r.sp;
    let data = mem.r16(sp);
    dest.write_u16(self, data, mem);
    self.r.sp = sp.wrapping_add(2);
    self.step();
  }

  #[inline]
  fn ld_dd_nn<W: WriteU16, R: ReadU16>(&mut self, w: W, r: R, mem: &mut dyn Memory) {
    let data = r.read_u16(self, mem);
    w.write_u16(self, data, mem);
    self.step_n(3);
  }

  #[inline]
  fn ld_dd_nn_content<W: WriteU16, R: ReadU16>(&mut self, w: W, r: R, mem: &mut dyn Memory) {
    let data = r.read_u16(self, mem);
    let addr = mem.r16(data);
    w.write_u16(self, addr, mem);
    self.step_n(3);
  }

  #[inline]
  fn ld_16_plus_d_r<R2: ReadU16, R: ReadU8, D: ReadU8>(&mut self, base:R2, r: R, d: D, mem: &mut dyn Memory) 
  {
    let offset = d.read_u8(self, mem) as i8;
    let displacement = base.read_u16(self, mem).wrapping_add(offset as u16);
    let value = r.read_u8(self, mem);
    mem.w8(displacement, value);
    self.step_n(2);
  }

  #[inline]
  fn ld_r_ix_d<R: ReadU8, R16: ReadU16, W: WriteU8>(&mut self, base: R16, d: R, w: W, mem: &mut dyn Memory) {
    let offset = d.read_u8(self, mem) as i8;
    let displacement = base.read_u16(self, mem).wrapping_add(offset as u16);
    let value = mem.r8(displacement);
    w.write_u8(self, value, mem);
    self.step_n(2);
  }

  #[inline]
  fn ld_16_plus_d_n<R: ReadU16, D: ReadU16>(&mut self, r: R, d_plus_n: D,  mem: &mut dyn Memory) 
  {
    let n = ((d_plus_n.read_u16(self, mem) & 0xFF00) >> 8) as u8;
    let d = (d_plus_n.read_u16(self, mem) & 0x00FF) as i8;
    let displacement = r.read_u16(self, mem).wrapping_add(d as u16);
    mem.w8(displacement, n);
    self.step_n(3);
  }

  #[inline]
  fn ld_r_16_d<W8: WriteU8, R16: ReadU16, R: ReadU8>(&mut self, w: W8, baseaddr: R16, d: R, mem: &mut dyn Memory) {
    let displacement = d.read_u8(self, mem) as i8;
    let value = baseaddr.read_u16(self, mem);
    let address = mem.r8(value.wrapping_add(displacement as u16));
    w.write_u8(self, address, mem);
    self.step_n(2);
  }

  #[inline]
  fn add_a_ix_d<R: ReadU8, R16: ReadU16>(&mut self, baseaddr: R16, d: R, mem: &dyn Memory) {
    let displacement = d.read_u8(self, mem) as i8;
    let value = baseaddr.read_u16(self, mem);
    let n = mem.r8(value.wrapping_add(displacement as u16));
    let (result, overflow) =  self.r.a.overflowing_add(n);
    self.r.f = Flags::ZERO.check(result == 0) | 
          Flags::HALFCARRY |
          Flags::PARITY.check(result >= 0x80 || result <= 0x81) |
          Flags::CARRY.check(overflow) |
          Flags::SIGN.check(result & 0x80 != 0);
    self.r.a = result;
    self.step_n(2);
  }

  #[inline]
  fn rrca<RW: WriteU8 + ReadU8>(&mut self, rw: RW, mem: &mut dyn Memory) {
    let value = rw.read_u8(self, mem);
    let r = value.rotate_right(1);
    self.r.f = Flags::CARRY.check(value & 0x01 == 1) |
               Flags::SIGN.check(r & 0x80 != 0) |
              Flags::PARITY.check(r.count_ones() & 1 == 0);
    rw.write_u8(self, r, mem);
    self.step();
  }

  #[inline]
  fn rlca<RW: WriteU8 + ReadU8>(&mut self, rw: RW, mem: &mut dyn Memory) {
    let value = rw.read_u8(self, mem);
    let r = value.rotate_left(1);
    self.r.f = Flags::CARRY.check(value & 0x80 != 0) |
      Flags::ZERO.check(r == 0) |
      Flags::SIGN.check(r & 0x80 != 0) |
      Flags::PARITY.check(r.count_ones() & 1 == 0);
    rw.write_u8(self, r, mem);
    self.step();
  }

  #[inline]
  fn srl_m<RW: WriteU8 + ReadU8>(&mut self, rw: RW, mem: &mut dyn Memory ) {
    let value= rw.read_u8(self,  mem);
    let r = value.rotate_right(1) & 0x7F;
    self.r.f = Flags::CARRY.check(value & 0x01 == 1) |
      Flags::PARITY.check(r.count_ones() & 1 == 0) |
      Flags::ZERO.check(r == 0);
    rw.write_u8(self, r, mem);
    self.step();
  }

  #[inline]
  fn rla<RW: WriteU8 + ReadU8>(&mut self, rw: RW, mem: &mut dyn Memory) {
    let a = self.r.a;
    let c = a & 0b1000_0000;
    let temp = if self.r.f.contains(Flags::CARRY) { 1 } else { 0 };
    let r = a << 1 | temp;
    self.r.f = Flags::CARRY.check(c != 0) |
        (Flags::ZERO & self.r.f) |
        (Flags::SIGN & self.r.f) |
        (Flags::PARITY & self.r.f);
    rw.write_u8(self, r, mem);
    self.step();
  }

  #[inline]
  fn rra<RW: WriteU8 + ReadU8>(&mut self, rw: RW, mem: &mut dyn Memory) {
    let a = self.r.a;
    let c = a & 0b0000_0001;
    let temp = if self.r.f.contains(Flags::CARRY) { 0x80 } else { 0 };
    let r = (a >> 1) | temp;
    self.r.f = Flags::CARRY.check(c != 0) |
        (Flags::ZERO & self.r.f) |
        (Flags::SIGN & self.r.f) |
        (Flags::PARITY & self.r.f);
    rw.write_u8(self, r, mem);
    self.step();
  }

  #[inline]
  pub fn ret(&mut self, mem: &dyn Memory) {
    let sp = self.r.sp;
    let data = mem.r16(sp);
    self.r.pc = data;
    // println!("RET with {}", format!("{:#x}", data));
    self.r.sp = sp.wrapping_add(2);
  }

  #[inline]
  fn ret_cc(&mut self, condition: Condition, mem: &dyn Memory) {
    if condition.check(self.r.f) {
      self.ret(mem);
    } else {
      self.step();
    }
  }

  #[inline]
  fn sub_ixy_d<R16: ReadU16, D: ReadU8>(&mut self, r: R16, d: D, mem: &dyn Memory) {

    let base_addr = r.read_u16(self, mem);
    let displacement = d.read_u8(self, mem) as i8;
    let value = mem.r8(base_addr + displacement as u16);
    self.sub_n(value);
    self.step();
  }

  #[inline]
  fn sub_r<R: ReadU8>(&mut self, rw: R, mem: &dyn Memory) {

    let value = rw.read_u8(self, mem);
    self.sub_n(value);
  }

  #[inline]
  fn sub_n(&mut self, n: u8) {

    let result = self.r.a.wrapping_sub(n) as i16;
    
    let mut borrow = false;
    for mask in [0b000_0001, 0b0000_0010, 0b0000_0100, 0b0000_1000,
    0b0001_0000, 0b0010_0000, 0b0100_0000, 0b1000_0000].iter() {
      
      if (n & mask) > (self.r.a & mask) {
        borrow = true;
        break;
      }
    }
    
    self.r.f = Flags::ZERO.check(result == 0) | 
    Flags::HALFCARRY.check(borrow) | Flags::NEGATIVE | 
    Flags::SIGN.check(result & 0x80 != 0) |
    Flags::PARITY.check(result >= 0x80 || result <= 0x81) | 
    Flags::CARRY.check(borrow);
    
    self.r.a = result as u8;
    self.step();
  }


  #[inline]
  fn neg(&mut self) {

    let n = self.r.a;
    let result = 0 - n;
    
    let mut borrow = false;
    for mask in [0b000_0001, 0b0000_0010, 0b0000_0100, 0b0000_1000,
    0b0001_0000, 0b0010_0000, 0b0100_0000, 0b1000_0000].iter() {
      
      if (n & mask) > (self.r.a & mask) {
        borrow = true;
        break;
      }
    }
    
    self.r.f = Flags::ZERO.check(result == 0) | 
    Flags::HALFCARRY.check(borrow) | Flags::NEGATIVE | 
    Flags::SIGN.check(result & 0x80 != 0) |
    Flags::PARITY.check(result >= 0x80 || result <= 0x81) | 
    Flags::CARRY.check(borrow);
    
    self.r.a = result as u8;
    self.step();
  }

  #[inline]
  fn and_ixy_d<R16: ReadU16, D: ReadU8>(&mut self, r: R16, d: D, mem: &dyn Memory) {

    let base_addr = r.read_u16(self, mem);
    let displacement = d.read_u8(self, mem) as i8;
    let value = mem.r8(base_addr + displacement as u16);
    self.and_n(value);
    self.step();
  }


  #[inline]
  fn and_r<R: ReadU8>(&mut self, rw: R, mem: &dyn Memory) {
    let value = rw.read_u8(self,  mem);
    self.and_n(value);
  }

  #[inline]
  fn and_n(&mut self, n: u8) {
    let result = self.r.a & n;
    self.r.a = result;
    self.r.f = Flags::ZERO.check(result == 0) |
      Flags::PARITY.check(result.count_ones() & 1 != 0) |
      Flags::HALFCARRY |
      Flags::SIGN.check(result & 0x80 != 0);
    
    self.step();
  }

  fn halt(&mut self) {
    self.halted = true;
  }

  ///The contents of any register r' are loaded to any other register r.
  ///r, r' identifies any of the registers A, B, C, D, E, H, or L
  #[inline]
  fn ld_r_r<R: ReadU8, W: WriteU8>(&mut self, w: W, r: R, mem: &mut dyn Memory) {
    let value = r.read_u8(self, mem);
    w.write_u8(self, value, mem);

    self.step();
  }

  #[inline]
  fn inc_ss<RW: WriteU16 + ReadU16>(&mut self, w: RW, mem: &mut dyn Memory) {
    let value = w.read_u16(self, mem);
    let result = value.wrapping_add(1);
    w.write_u16(self, result, mem);
    self.r.f = Flags::ZERO.check(result == 0) | 
    Flags::HALFCARRY | Flags::SIGN.check(result & 0x80 != 0);
    self.step();
  }

  #[inline]
  fn ld_a_nn<R: ReadU16>(&mut self, a: R, mem: &dyn Memory) {
    let addr = a.read_u16(self, mem);
    let data = mem.r8(addr);
    self.r.a = data;
    self.step_n(3);
  }

  #[inline]
  fn ld_r_hl<W: WriteU8, R: ReadU8>(&mut self, w: W, r: R, mem: &mut dyn Memory) {
    let data = r.read_u8(self, mem);
    w.write_u8(self, data, mem);
    self.step();
  }

  #[inline]
  fn ld_nn_hl<R16: ReadU16, R: ReadU16>(&mut self, a: R16, r: R, mem: &mut dyn Memory) {
    let hl = r.read_u16(self, mem);
    let h = (hl & 0xFF00) >> 8;
    let l = hl & 0x00FF;
    let address = a.read_u16(self,  mem);
    mem.w8(address, l as u8);
    mem.w8(address + 1, h as u8);
    self.step_n(3);
  }

  #[inline]
  fn cpl(&mut self) {
      self.r.a = !self.r.a;
      self.r.f = Flags::NEGATIVE | Flags::HALFCARRY;
      self.step();
  }

  #[inline]
  fn adc_r<R: ReadU8>(&mut self, r: R, mem: &dyn Memory) {

    let value = r.read_u8(self, mem);
    self.adc_n(value);
  }

  #[inline]
  fn adc_ixy_d<R16: ReadU16, D: ReadU8>(&mut self, r: R16, d: D, mem: &dyn Memory) {

    let base_addr = r.read_u16(self, mem);
    let displacement = d.read_u8(self, mem) as i8;
    let value = mem.r8(base_addr + displacement as u16);
    self.adc_n(value);
    self.step();
  }

  #[inline]
  fn adc_n(&mut self, n: u8) {

    let carried = if self.r.f.contains(Flags::CARRY) { 1 } else { 0 };
    let result = self.r.a.wrapping_add(n).wrapping_add(carried);
  
    let carry = self.r.a as u16 + n as u16 + carried as u16 > 0xFF;
    let half_carry = (self.r.a & 0xF) + (n & 0xF) + carried > 0xF;
    self.r.f =  Flags::ZERO.check(result == 0) |
                Flags::PARITY.check(result > 0x80) |
                Flags::HALFCARRY.check(half_carry) | 
                Flags::CARRY.check(carry) | 
                Flags::SIGN.check(result & 0x80 != 0);
                
    self.r.a = result;
    self.step();
  }

  #[inline]
  fn adc_16<R: ReadU16>(&mut self, r: R, mem: &dyn Memory) {

    let value = r.read_u16(self, mem);
    let hl = self.r.get_u16(Register16Bit::HL);
    let carried = if self.r.f.contains(Flags::CARRY) { 1 } else { 0 };
    let result = hl.wrapping_add(value).wrapping_add(carried);
  
    let mask = (1u16 << 11).wrapping_sub(1);
    let half_carry = (hl & mask) + (value & mask) > mask;
    self.r.f = Flags::ZERO.check(result == 0) |
                Flags::HALFCARRY.check(half_carry) | 
                Flags::CARRY.check(hl > 0xFFFF - value) | Flags::SIGN.check(result & 0x8000 != 0);
                
    self.r.set_u16(Register16Bit::HL, result);
    self.step();
  }

  #[inline]
  fn dec_r<RW: ReadU8 + WriteU8>(&mut self, rw: RW, mem: &mut dyn Memory) {
      let value = rw.read_u8(self, mem);
      let result = value.wrapping_sub(1);
      let a = rw.read_u8(self, mem);

      rw.write_u8(self, result, mem);
      self.r.f = Flags::NEGATIVE | Flags::SIGN.check(result & 0x80 != 0) 
        | Flags::ZERO.check(result == 0) |
        Flags::HALFCARRY.check((a & 0xF) < (result & 0xF)) |
        Flags::PARITY.check(value == 0x80) | 
        (Flags::CARRY & self.r.f);

      self.step();
  }

  #[inline]
  fn dec_ss<RW: ReadU16 + WriteU16>(&mut self, rw: RW, mem: &mut dyn Memory) {

    let a = rw.read_u16(self, mem);
    let (r, overflow) = a.overflowing_sub(1);
    rw.write_u16(self, r, mem);
    self.r.f = Flags::SIGN.check(r & 0x8000 != 0) | Flags::NEGATIVE |
      Flags::HALFCARRY.check((a & 0xF) < (r & 0xF)) |
      Flags::CARRY.check(overflow) | Flags::ZERO.check(r == 0);

    self.step();
  }

  #[inline]
  fn add_a_r<R: ReadU8>(&mut self, r: R, mem: &dyn Memory) {
      let value = r.read_u8(self, mem);
      self.add_a_n(value);
  }

  #[inline]
  fn add_a_n(&mut self, n: u8) {
      let a = self.r.a as u8;
      let (result, carry) = a.overflowing_add(n);
      self.r.a = result;

      let half_carry = (self.r.a & 0xF) + (n & 0xF) > 0xF;
      self.r.f = Flags::ZERO.check(result == 0) |
                 Flags::SIGN.check(result & 0x80 != 0) |
                 Flags::HALFCARRY.check(half_carry) |
                 Flags::PARITY.check(result > 0x80) |
                 Flags::CARRY.check(carry);
      
      self.step();
  }


  #[inline]
  fn push_16<R: ReadU16>(&mut self, r: R, mem: &mut dyn Memory) {
    let value = r.read_u16(self, mem);
    self.push(value, mem);
    self.step();
  }

  #[inline]
  fn call_cc_nn<R: ReadU16>(&mut self, condition: Condition, addr: R, mem: &mut dyn Memory) {
    if condition.check(self.r.f) {
      let addr  = addr.read_u16(self, mem);
      self.call(addr, mem);
    } else {
      self.step_n(3);
    }
  }

  #[inline]
  fn call(&mut self, addr: u16, mem: &mut dyn Memory) {
      let pc = self.r.pc + 3;
      self.push(pc, mem);
      self.r.pc = addr;
  }

  #[inline]
  fn add_hl_ss<WR: ReadU16 + WriteU16, R: ReadU16>(&mut self, hl: WR, r: R, mem: &mut dyn Memory) {
    let value = r.read_u16(self, mem);
    let hl_ = hl.read_u16(self, mem);
    let result = hl_.wrapping_add(value);
    let mask = (1u16 << 11).wrapping_sub(1);
    let half_carry = (hl_ & mask) + (value & mask) > mask;
    hl.write_u16(self, result, mem);
    self.r.f = (self.r.f & Flags::ZERO) |
        (self.r.f & Flags::SIGN) |
        (self.r.f & Flags::PARITY) |
        Flags::HALFCARRY.check(half_carry) |
        Flags::CARRY.check(hl_ > 0xFFFF - value);
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
  pub fn jp_cc_nn<R: ReadU16>(&mut self, condition:Condition, addr: R, mem: &dyn Memory) {
    if condition.check(self.r.f) {
      self.jmp(addr, mem);
    } else {
      self.step_n(3);
    }
  }

  #[inline]
  pub fn ld_sp_hl<RW: ReadU16 + WriteU16>(&mut self, sp: RW, hl: RW, mem: &mut dyn Memory) {
    let value = hl.read_u16(self, mem);
    sp.write_u16(self, value, mem);
    self.step();
  }

  /// if address 4545h contains 37h and address 4546h contains a1h,
  /// then upon the execution of an ld hl, (4545h) instruction,
  /// the hl register pair contains a137h.
  
  #[inline]
  pub fn ld_hl_nn<R: ReadU16>(&mut self, r : R, mem: &dyn Memory) {
    let value = r.read_u16(self, mem);
      let nn = mem.r16(value);
      self.r.h = (nn >> 8) as u8;
      self.r.l = (nn & 0x00FF) as u8;
      self.step_n(3);
  }

  #[inline]
  fn xor_ixy_d<R16: ReadU16, D: ReadU8>(&mut self, r: R16, d: D, mem: &dyn Memory) {

    let base_addr = r.read_u16(self, mem);
    let displacement = d.read_u8(self, mem) as i8;
    let value = mem.r8(base_addr + displacement as u16);
    self.xor_n(value);
    self.step();
  }  

  #[inline]
  pub fn xor_r<R: ReadU8>(&mut self, r: R, mem: &dyn Memory) {
    let value = r.read_u8(self, mem);
    self.xor_n(value);
  }

  #[inline]
  pub fn xor_n(&mut self, n: u8) {
    let result = self.r.a ^ n;
    self.r.a  = result;
    self.r.f = Flags::ZERO.check(result == 0) |
     Flags::PARITY.check(result.count_ones() & 1 != 0) |
     Flags::SIGN.check(result & 0x80 != 0);
    self.step();
  }

  #[inline]
  fn or_ixy_d<R16: ReadU16, D: ReadU8>(&mut self, r: R16, d: D, mem: &dyn Memory) {

    let base_addr = r.read_u16(self, mem);
    let displacement = d.read_u8(self, mem) as i8;
    let value = mem.r8(base_addr + displacement as u16);
    self.or_n(value);
    self.step();
  }

  #[inline]
  pub fn or_r<R: ReadU8>(&mut self, r: R, mem: &dyn Memory) {
    let value = r.read_u8(self, mem);
    self.or_n(value);
  }

  #[inline]
  pub fn or_n(&mut self, n: u8) {
    let result = self.r.a | n;
    self.r.a  = result;
    self.r.f = Flags::ZERO.check(result == 0) |
      Flags::PARITY.check(result.count_ones() & 1 != 0) |
      Flags::SIGN.check(result & 0x80 != 0);
    self.step();
  }

  #[inline]
  pub fn rst_p(&mut self, addr: u8, mem: &mut dyn Memory) {
      let pc = self.r.pc + 1;
      self.push(pc, mem);
      self.r.pc = addr as u16
  }

  /*
  ld   (hl), r
  the contents of register r are loaded to the memory location specified by the contents of the hl register pair.
  */
  #[inline]
  pub fn ld_hl_r<R: ReadU8, R16: ReadU16>(&mut self, addr: R16, r: R, mem: &mut dyn Memory) {
    let address = addr.read_u16(self, mem);
    let value = r.read_u8(self, mem);
    mem.w8(address, value);
    self.step();
  }

  #[inline]
  pub fn inc_r<RW: ReadU8 + WriteU8>(&mut self, rw: RW, mem: &mut dyn Memory) {
      let value = rw.read_u8(self, mem);
      let result = value.wrapping_add(1);
      rw.write_u8(self, result, mem);

      // update flags
      self.r.f = Flags::ZERO.check(result == 0) |
      Flags::HALFCARRY.check(value & 0xF == 0xF) |
      Flags::SIGN.check(result & 0x80 != 0) |
      Flags::PARITY.check(value == 0x7f) |
      (Flags::CARRY & self.r.f);
      self.step();
  }

  #[inline]
  pub fn inc_r_d<R: ReadU16, D: ReadU8>(&mut self, rw: R, d: D, mem: &mut dyn Memory) {
      let offset = d.read_u8(self, mem) as i8;  
      let displacement = rw.read_u16(self, mem).wrapping_add(offset as u16);
      let value = mem.r8(displacement);
      let result = value.wrapping_add(1);
      mem.w8(displacement, result);

      // update flags
      self.r.f = Flags::ZERO.check(result == 0) |
      Flags::HALFCARRY.check(value & 0xF == 0xF) |
      Flags::SIGN.check(result & 0x80 != 0) |
      (Flags::CARRY & self.r.f);
      self.step_n(2);
  }

  #[inline]
  pub fn ld_8_nn<W: WriteU8, R:ReadU8>(&mut self, w: W, r: R, mem: &mut dyn Memory) {
      let data = r.read_u8(self,  mem);
      w.write_u8(self, data, mem);
      self.step();
  }

  #[inline]
  pub fn jr_conditional(&mut self, condition: Condition, mem: &dyn Memory) {
    if condition.check(self.r.f) {
        let addr = self.r.pc;
        let offset = self.next_u8(mem) as i8;
        self.r.pc = addr.wrapping_add(offset as u16);
      }
    self.step_n(2);
  }

  #[inline]
  pub fn jr_e<R: ReadU8>(&mut self, d: R, mem: &dyn Memory) {
    let addr = self.r.pc;
    let offset = d.read_u8(self, mem)  as i8;
    self.r.pc = addr.wrapping_add(offset as u16);
    self.step_n(2);
  }

  pub fn nop(&mut self) {
    self.step()
  }

  #[inline]
  pub fn jmp<R: ReadU16>(&mut self, addr: R, mem: &dyn Memory) {
      //this will make the pc point to the next two bytes of the instruction
      self.r.pc = addr.read_u16(self, mem);
  }

  #[inline]
  pub fn djnz<R: ReadU8>(&mut self, r: R, mem: &dyn Memory) {
    let pc = self.r.pc;
    let b = self.r.b;
    let displacement = r.read_u8(self, mem) as i8;
    let r = b.wrapping_sub(1);
    if r != 0 {
      self.r.pc = pc.wrapping_add(displacement as u16);
    }
    self.step_n(2);
    self.r.b = r;
  }
}