/// A self instruction is built from 3 bit groups,
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
// use Register;

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
    ((self.HL & 0xFF00) >> 8) as u8
  }

   pub fn get_HL_L(&mut self) -> u8 {
    (self.HL & 0x00FF) as u8
  }

  pub fn set_HL_L(&mut self, L: u8) {
    let H = self.HL & 0xFF00;
    self.HL = H | L as u16;
  }

  pub fn set_HL_H(&mut self, H: u8) {
    let L = self.HL & 0x00FF;
    self.HL = (H as u16) << 8 | L;
  }

  pub fn exec(&mut self) {
    let op = self.mem.r8(self.pc);
    println!("{}", format!("{:#X}", op));
    self.step();
    match op {
        0x00 => { self.nop(); },
        0x01 => { self.LD_BC_nn() },
        0x09 => { self.ADD_HL_BC() },
        0x20 => { self.jr_nz() },
        0x11 | 0x22 | 0x21 => { self.LD_dd_nn(op) },
        0x1D => { self.DEC_r(op) },
        0x2A => { self.LD_HL_nn() },
        0x2B => { self.DEC_ss() },
        0x2C | 0x24 | 0x3C => { self.INC_r(op) },
        0x2F => { self.CPL() },
        0x36 => { self.LD_HL_n()}
        0x06 | 0x3E | 0x2E => { self.LD_r_n(op) },
        0xB6 => { self.OR_HL() },
        0xCE => { self.ADC_r () },
        0x4E | 0x46 | 0x56 | 0x5E | 0x66 | 0x6E => { self.LD_r_HL(op) },
        0x51 | 0x5C | 0x64 | 0x65 | 0x6C
        | 0x61 | 0x62 | 0x63 | 0x68  => { self.LD_hh(op) },

        // 0x6C | 0x61 | 0x62 | 0x63 => { LD_rr(op, ) },
        0x70 | 0x73 | 0x77 => { self.LD_HL_r(op) },
        0x83 => { self.ADD_A_r(op) },
        0x97 => { self.SUB_s(op) },
        0xA9 => { self.XOR_r(op) },
        0xB4 => { self.OR_r(op) },
        0xC3 => { self.JMP(); },
        0xCD => { self.CALL() },
        0xEA | 0xE2 | 0xDA | 0xC2 => { self.JP_cc_nn(op) },
        0xF4 => { self.CALL_cc_nn() },
        0xF5 | 0xC5 | 0xD5 => { self.PUSH_qq(op) },
        0xFF | 0xC7 => { self.RST_p(op) },
        0xF9 => { self.LD_SP_HL() },
        _ => {  panic!("Unknown CP/M call {}!"); },
    }
  }


  pub fn LD_r_HL(&mut self, op: u8) {
    let data = self.mem.r8(self.HL);
    match op {
      0x4E => {
        self.set_C(data);
      },
      0x46 => {
        self.set_B(data);
      },
      0x7E => {
        self.set_A(data);
      },
      0x56 => {
        self.set_D(data);
      },
      0x5E => {
        self.set_E(data);
      },
      0x66 => {
        self.set_HL_H(data);
      },
      0x6E => {
        self.set_HL_L(data);
      },
      _ => {  panic!("Unknown CP/M call {}!"); },
    }
  }

  /// Opcode execution
  /***
   * The n integer is loaded to the memory address specified by the contents of the HL register pair.
   */
  pub fn LD_HL_n(&mut self) {
    let n = self.mem.r8(self.pc());
    let m = self.HL;
    self.mem.w8(m, n);
    self.step();
  }

  pub fn CPL(&mut self) {
      let A = self.a;
      self.set_A(!A);
      self.Flags_setH(true);
      self.Flags_setN(true);
      self.step();
  }

  pub fn OR_HL(&mut self) {
      let A = self.a;
      let _HL_ = self.mem.r8(self.HL);
      let result = A | _HL_;
      self.set_A(result);

      self.Flags_setS((result as i8) < 0);
      self.Flags_setZ(result == 0);
      self.Flags_setN(false);
      self.Flags_setC(false);
      self.Flags_setH(false);
      let is_even = result & 0b000_0001 == 1;
      self.Flags_setPE(is_even); // FIX !!!!
      self.step();
  }

  pub fn ADC_r(&mut self) {
      let A = self.a;
      let n = self.mem.r8(self.pc());
      let C = self.Flags_getC();
      if C {
          self.set_A(A + n + 1);
          self.Flags_setS(A + n + 1 == 0);
          self.Flags_setZ(A + n + 1 == 1);
          self.Flags_setPE(A + n + 1 > 127);
      } else {
          self.set_A(A + n);
          self.Flags_setS(A + n == 0);
          self.Flags_setZ(A + n == 1);
          self.Flags_setPE(A + n > 127);
      }
      // A = self.a;
      self.Flags_setN(false);
      self.Flags_setC(A == 0x7F);
      //H is set if carry from bit 3; otherwise, it is reset.
      self.Flags_setH((A & 0b0000_1111) == 0xF);
      self.step();
  }

  pub fn DEC_r(&mut self, op: u8) {
      let sel = (op & 0b0111_0000) >> 4 as u8;
      let mut r = 0;
      match sel {
          0b000 => {
              r = self.b as i8;
              self.set_B((r -1) as u8);
          },
          0b001 => {
              r = self.c as i8;
              self.set_C((r -1) as u8);
          },
          0b010 => {
              r = self.d as i8;
              self.set_D((r -1) as u8);
          },
          0b011 => {
              r = self.e as i8;
              self.set_E((r -1) as u8);
          },
          0b100 => {
              r = self.get_HL_H() as i8;
              self.set_HL_H((r -1) as u8);
          },
          0b101 => {
              r = self.get_HL_L() as i8;
              self.set_HL_L((r -1) as u8);
          },
          0b111 => {
              r = self.a as i8;
              self.set_A((r -1) as u8);
          },
            _ => {
              panic!("Unimplemented instruction");
          }
      }

      self.Flags_setS(r == 0);
      self.Flags_setZ(r == 1);
      self.Flags_setN(true);
      self.Flags_setPE(r as u8 == 0x80);
      self.step();
  }

  pub fn ADD_A_r( &mut self, op: u8) {
      let sel = (op & 0b0000_0111) as u8;
      match sel {
          0b111 => {
              let A = self.a;
              self.set_A(A + A);
          },
          0b000 => {
              let B = self.b;
              let A = self.a;
              self.set_A(A + B);
          },
          0b001 => {
              let C = self.c;
              let A = self.a;
              self.set_A(A + C);
          },
          0b010 => {
              let D = self.d;
              let A = self.a;
              self.set_A(A + D);
          },
          0b011 => {
              let E = self.e;
              let A = self.a;
              self.set_A(A + E);
          },
          0b100 => {
              let H = self.get_HL_H();
              let A = self.a;
              self.set_A(A + H);
          },
          0b101 => {
              let L = self.get_HL_L();
              let A = self.a;
              self.set_A(A + L);
          },
          _ => {
              panic!("Unimplemented instruction");
          }
      }
    self.step();
  }

  pub fn PUSH_qq( &mut self, op: u8) {
      let sel = (op & 0b0011_0000) >> 4 as u8;
      match sel {
          0b11 => {
              let AF = self.AF();
              self.push(AF);
          },
          0b00 => {
              let BC = self.BC();
              self.push(BC);
          },
          0b01 => {
              let DE = self.DE();
              self.push(DE);
          },
          0b10 => {
              let HL = self.HL;
              self.push(HL);
          },
          _ => {
              panic!("Unimplemented instruction");
          }
      }
    self.step();
  }
  pub fn CALL_cc_nn(&mut self) {
      // F4: if Sign Positive (P), then push PC onto stack and put nn contents on PC
      if !self.Flags_getS() {
          let addr = self.mem.r16(self.pc());
          let pc = self.pc();
          self.push(pc);
          self.set_pc(addr);
      } else {
          self.step();
      }
  }

  pub fn SUB_s(&mut self, op: u8) {
      let n = self.mem.r8(self.pc()) as i8;
      let r = self.a as i8 - n;
      self.set_A(r as u8);
      self.Flags_setS(r < 0);
      self.Flags_setZ(r == 0);
      self.Flags_setN(true);
      self.Flags_setPE(r as u8 == 0x80);
      self.step();
  }

  pub fn CALL(&mut self) {
      let pc = self.pc();
      let addr = self.mem.r16(self.pc());
      self.push(pc -1);
      self.set_pc(addr);
  }

  pub fn ADD_HL_BC(&mut self) {
      let BC = self.BC() as i16;
      let HL = self.HL as i16;
      self.set_HL((HL + BC) as u16);
      self.Flags_setN(false);
      self.Flags_setC((HL & 0b0100_0000_0000_0000) >> 14 == 1);
      self.Flags_setH((HL & 0b0000_1000_0000_0000) >> 10 == 1);
  }

  /// cc   Condition Relevant  Flag
  /// 000  Non-Zero (NZ)         Z
  /// 001   Zero (Z)             Z
  /// 010   No Carry (NC)        C
  /// 011   Carry (C)            C
  /// 100   Parity Odd (PO)     P/V
  /// 101   Parity Even (PE)    P/V
  /// 110   Sign Positive (P)    S
  /// 111   Sign Negative (M)    S
  pub fn JP_cc_nn(&mut self, op: u8) {
      let sel = (op & 0b00111000) >> 3 as u8;
      let addr = self.mem.r16(self.pc());

      match sel {
          0b000 => {
              if !self.Flags_getZ() {
                  self.set_pc(addr)
              } else {


              }
          }
          0b101 => {
              if self.Flags_getPE() {
                  self.set_pc(addr);
              } else {


              }
          },
          0b011 => {
              if self.Flags_getC() {
                  self.set_pc(addr);
              } else {


              }
          },
          0b100 => {
              if !self.Flags_getPE() {
                  self.set_pc(addr);
              } else {


              }
          },
          _ => {
              panic!("Unimplemented instruction");
          }
      }
  }

  pub fn LD_SP_HL(&mut self) {
      self.set_sp(self.HL);
  }

  /// If address 4545h contains 37h and address 4546h contains A1h,
  /// then upon the execution of an LD HL, (4545h) instruction,
  /// the HL register pair contains A137h.
  pub fn LD_HL_nn(&mut self) {
      let addr = self.pc();
      let n = self.mem.r8(addr);
      let nn = self.mem.r8(addr + 1);
      self.set_HL_L(n);
      self.set_HL_H(nn);
      self.step();
  }

  pub fn LD_dd_nn(&mut self, op: u8) {
      let addr = self.pc();
      let nn = self.mem.r16(addr);

      match op {
          0x11 => {
              self.set_DE(nn);
          },
          0x21 => {
              self.set_HL(nn);
          }
          0x22 => {
              self.set_HL(nn);
          },
          _ => {  panic!("Unknown CP/M call {}!"); }
      }
    self.step();
    self.step();
  }

  pub fn XOR_r(&mut self, op: u8) {
      let C = self.c;
      let A = self.a;
      let result = (C ^ A) as i8;
      self.set_A(result as u8);
      // Check flags
      self.Flags_setZ(result == 0);
      self.Flags_setS(result < 0);
      self.Flags_setN(false);
      self.Flags_setC(false);
      self.Flags_setH(false);

  }

  pub fn OR_r(&mut self, op: u8) {
      let H = (self.HL & 0x0F) as u8;
      let A = self.a;
      let result = (H | A) as i8;
      self.set_A(result as u8);
      // Check flags
      self.Flags_setZ(result == 0);
      self.Flags_setS(result < 0);
      self.Flags_setN(false);
      self.Flags_setC(false);
      self.Flags_setH(false);

  }

  pub fn RST_p(&mut self, op: u8) {
      let pc = self.pc();
      self.push(pc);
      match op {
          0xFF => {
              self.set_pc(0x0038);
          },
          0xC7 => {
              self.set_pc(0x0000);
          },
          _ => {
              println!("Unimplemented instruction");
          }
      }
  }

  /*
  The 8-bit integer n is loaded to any register r, in which r identifies registers A, B, C, D, E, H, or L,
  assembled as follows in the object code:
  */

  pub fn LD_r_n(&mut self, op: u8) {
      let sel = (op & 0b00111110) >> 3 as u8;
      let addr = self.pc();
      let n = self.mem.r8(addr);
      match sel {
          0b111 => {
              self.set_A(n);
          },
          0b000 => {
              self.set_B(n);
          },
          0b001 => {
              self.set_C(n);
          },
          0b010 => {
              self.set_D(n);
          },
          0b011 => {
              self.set_E(n);
          },
          0b100 => {
              self.set_HL_H(n);
          },
          0b101 => {
              self.set_HL_L(n);
          },
          _ => {  panic!("Unknown CP/M call {}!"); }
      }
  }

  /*
  LD   (HL), r
  The contents of register r are loaded to the memory location specified by the contents of the HL register pair.
  */
  pub fn LD_HL_r(&mut self, op: u8) {
    let r = op & 0b0000_0111;
    let addr = self.HL;
    match r {
      0b000_0111 => {
        self.mem.w8(addr, self.a);
      },
      0b000_0000 => {
        self.mem.w8(addr, self.b);
      },
      0b000_0001 => {
        self.mem.w8(addr, self.c);
      },
      0b000_0010 => {
        self.mem.w8(addr, self.d);
      },
      0b000_0011 => {
        self.mem.w8(addr, self.d);
      },
      0b000_0100=> {
        let H = self.get_HL_H();
        self.mem.w8(addr,H);
      },
      0b000_0101 => {
        let L = self.get_HL_L();
        self.mem.w8(addr,L);
      },
      _ => {  panic!("Unknown CP/M call {}!"); }
    }
  }

  pub fn LD_lb(&mut self) {
      self.set_HL_L(self.b);
  }

  pub fn LD_rr(&mut self, op: u8) {
      match op {
          0x6C => {
              /* LD  L,H */
              let H = (self.HL & 0xF0) as u8;
              self.set_HL_L(H);
          },
          0x61 => {
              /* LD   H,C */
              let c = self.c;
              self.set_HL_H(c);
          },
          0x62 => {
              /* LD   H,D */
              self.set_HL_H(self.d);
          },
          0x63 => {
              /* LD   H,E */
              self.set_HL_H(self.e);
          }
          _ => {  panic!("Unknown CP/M call {}!"); }
      }
  }

  pub fn INC_r(&mut self, op: u8) {
      let mut incr = 0;
      let mut r = 0;
      match op {
          0x24 => {
              /* INC  H */
              r = self.get_HL_H();
              incr = r + 1;
              self.set_HL_H(incr);
          }
          0x2C => {
              r = (self.HL & 0x0F) as u8;
              incr = r + 1;
              self.set_HL_L(incr);
          },
          0x3C => {
              r = self.a;
              incr = r + 1;
              self.set_A(incr);

          },
          _ => {  panic!("Unknown CP/M call {}!"); }

      }

      // Update flags
      self.Flags_setZ(incr == 0);
      self.Flags_setS((incr as i8) < 0);
      self.Flags_setN(false);
      self.Flags_setC(false);
      self.Flags_setH(false);
      self.Flags_setPE(r == 0x7F);
  }

  pub fn LD_BC_nn(&mut self) {

      let addr = self.pc();
      let lowByte = self.mem.r8(addr);
      let highByte = self.mem.r8(addr + 1);
      self.set_C(lowByte);
      self.set_B(highByte);


  }

  pub fn jr_nz(&mut self) {
      if !self.Flags_getZ() {
          let addr = self.pc();
          let offset = self.mem.r8(addr);
          let displacement = addr.wrapping_add(offset as u16);
          self.set_pc(displacement);
      } else {

      }

  }

  pub fn LD_hh(&mut self, op: u8) {
      let r = (op & 0b0011_1000) >> 3 as u8;
      let r1 = (op & 0b0000_0111) >> 3 as u8;
      match (r, r1) {
          (0b111, 0b000) => {
              self.set_A(self.b);
          },
          (0b111, 0b001) => {
              self.set_A(self.c);
          },
          (0b111, 0b010) => {
              self.set_A(self.d);
          },
          (0b111, 0b011) => {
              self.set_A(self.e);
          },
          (0b111, 0b100) => {
              let H = self.get_HL_H();
              self.set_A(H);
          },
          (0b111, 0b101) => {
              let L = self.get_HL_L();
              self.set_A(L);
          },
          (0b000, 0b111) => {
              self.set_B(self.a);
          },
          (0b000, 0b001) => {
              self.set_B(self.c);
          },
          (0b000, 0b010) => {
              self.set_B(self.d);
          },
          (0b000, 0b011) => {
              self.set_B(self.e);
          },
          (0b000, 0b100) => {
              let H = self.get_HL_H();
              self.set_B(H);
          },
          (0b000, 0b101) => {
              let L = self.get_HL_L();
              self.set_B(L);
          },
          (0b001, 0b111) => {
              self.set_C(self.a);
          },
          (0b001, 0b000) => {
              self.set_C(self.b);
          },
          (0b001, 0b010) => {
              self.set_C(self.d);
          },
          (0b001, 0b011) => {
              self.set_C(self.e);
          },
          (0b001, 0b100) => {
              let H = self.get_HL_H();
              self.set_C(H);
          },
          (0b001, 0b101) => {
              let L = self.get_HL_L();
              self.set_C(L);
          },
          (0b010, 0b111) => {
              self.set_D(self.a);
          },
          (0b010, 0b000) => {
              self.set_D(self.b);
          },
          (0b010, 0b001) => {
              self.set_D(self.c);
          },
          (0b010, 0b011) => {
              self.set_D(self.e)
          },
          (0b010, 0b100) => {
              let H = self.get_HL_H();
              self.set_D(H);
          },
          (0b010, 0b101) => {
              let L = self.get_HL_L();
              self.set_D(L);
          },
          (0b011, 0b111) => {
              self.set_E(self.a);
          },
          (0b011, 0b000) => {
              self.set_E(self.b);
          },
          (0b011, 0b001) => {
              self.set_E(self.c);
          },
          (0b011, 0b010) => {
              self.set_E(self.d);
          },
          (0b011, 0b011) => {
              self.set_E(self.e)
          },
          (0b011, 0b100) => {
              let H = self.get_HL_H();
              self.set_E(H);
          },
          (0b011, 0b101) => {
              let L = self.get_HL_L();
              self.set_E(L);
          },
          (0b100, 0b111) => {
              self.set_HL_H(self.a);
          },
          (0b100, 0b000) => {
              self.set_HL_H(self.b);
          },
          (0b100, 0b001) => {
              self.set_HL_H(self.c);
          },
          (0b100, 0b010) => {
              self.set_HL_H(self.d);
          },
          (0b100, 0b011) => {
              self.set_HL_H(self.e);
          },
          (0b100, 0b100) => {
              let H = self.get_HL_H();
              self.set_HL_H(H);
          },
          (0b100, 0b101) => {
              let L = self.get_HL_L();
              self.set_HL_L(L);
          },
          (0b101, 0b111) => {
              self.set_HL_L(self.a);
          },
          (0b101, 0b000) => {
              self.set_HL_L(self.b);
          },
          (0b101,0b001) => {
              self.set_HL_L(self.c);
          },
          (0b101,0b010) => {
              self.set_HL_L(self.d);
          },
          (0b101, 0b011) => {
              self.set_HL_L(self.e);
          },
          (0b101,0b100) => {
              let H = self.get_HL_H();
              self.set_HL_L(H);
          },
          (0b101, 0b101) => {
              let H = self.get_HL_H();
              self.set_HL_L(H);
          },
          _ => {  panic!("Unknown CP/M call {}!"); }
      }

  }

  pub fn nop(&mut self) {

  }

  pub fn JMP(&mut self) {
      //This will make the PC point to the next two bytes of the instruction
      let addr = self.pc();
      let data = self.mem.r16(addr);
      self.set_pc(data);
  }

  fn DEC_ss(&mut self) {
    let HL = self.HL;
    let r = HL - 1;
    self.set_HL(r);
  }
}