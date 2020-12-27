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


pub use crate:: memory::Memory;
// use register;

pub struct Z80 {
  pub a: i8,
  pub b: i8,
  pub c: i8,
  pub d: i8,
  pub e: i8,
  pub pc: u16,
  pub sp: u16,
  pub hl: u16,
  pub ix: u16,
  pub iy: u16,
  pub i: u8,
  pub mem: Memory,
  ///flags (f): sz-h-pnc
  pub flags: u8,
}

impl Z80 {
  pub fn new(mem: Memory) -> Z80 {
    Z80{ a:0,
          pc: 0,
          b: 0,
          c: 0,
          d: 0,
          e: 0,
          sp: 0,
          hl: 0,
          i: 0,
          ix: 0,
          iy: 0,
          mem: mem,
          flags: 0x0
        }
  }

  pub fn flags_get_z(&mut self) -> bool {
    (self.flags & 0b0100_0000) >> 6 != 0
  }

  pub fn flags_set_z(&mut self, val:bool) {
    if val {
      self.flags |= 0b0100_0000;
    } else {
      self.flags &= 0b1011_1111;
    }
  }

  pub fn flags_set_s(&mut self, val:bool) {
    if val {
      self.flags |= 0b1000_0000;
    } else {
      self.flags &= 0b0111_1111;
    }
  }

  pub fn flags_get_s(&mut self) -> bool {
    (self.flags & 0b1000_0000) >> 7 != 0
  }

  ///n - subtract - set if the last operation was a subtraction
  pub fn flags_set_n(&mut self, val:bool) {
    if val {
      self.flags |= 0b0000_0010;
    } else {
      self.flags &= 0b1111_1101;
    }
  }

  pub fn flags_get_n(&mut self) -> bool {
    (self.flags & 0b0000_0010) >> 1 != 0
  }

  ///c - carry - set if the result did not fit in the register
  pub fn flags_set_c(&mut self, val:bool) {
    if val {
      self.flags |= 0b0000_0001;
    } else {
      self.flags &= 0b1111_1110;
    }
  }

  pub fn flags_get_h(&mut self) -> bool {
    (self.flags & 0b0001_0000) >> 4 != 0
  }

  ///h - half carry - carry from bit 3 to bit 4
  pub fn flags_set_h(&mut self, val:bool) {
    if val {
      self.flags |= 0b0001_0000;
    } else {
      self.flags &= 0b1110_1111;
    }
  }

  ///p/v - parity or overflow
  /// parity set if even number of bits set
  /// overflow set if the 2-complement result does not fit in the register
  pub fn flags_set_pe(&mut self, val:bool) {
    if val {
      self.flags |= 0b0000_0100;
    } else {
      self.flags &= 0b1111_1011;
    }
  }

  pub fn flags_get_pe(&mut self) -> bool {
    (self.flags & 0b0000_0100) >> 2 != 0
  }

  ///c - carry - set if the result did not fit in the register
  pub fn flags_get_c(&mut self) -> bool  {
    return (self.flags & 0b0000_0001) == 1;
  }

  ///This stack usually starts at $0000 so as to place at the very end of memory
  ///(the first push to the stack decrements the stack pointer causing it to wrap around to $FFFF).
  pub fn push(&mut self, val:u16) {
    let sp = self.sp;
    let addr = sp.wrapping_sub(2);
    self.set_sp(addr);
    self.mem.w16(addr, val);
  }

  pub fn set_a(&mut self, a: i8) {
    self.a = a
  }

  pub fn set_b(&mut self, b: i8) {
    self.b = b
  }

  pub fn set_c(&mut self, c: i8) {
    self.c = c
  }

  pub fn set_d(&mut self, d: i8) {
    self.d = d
  }

  pub fn set_e(&mut self, e: i8) {
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

  pub fn set_de(&mut self, de: u16) {
    self.d = (de >> 8) as i8;
    self.e = (de & 0x00ff) as i8;
  }

  pub fn af(&mut self) -> u16 {
    (self.a as u8 as u16) << 8 as u16 | self.flags as u8 as u16
  }

   pub fn set_af(&mut self, af: u16) {
    self.a = (af >> 8) as i8;
    self.flags = (af & 0x00ff) as u8;
  }

  pub fn set_hl(&mut self, hl: u16) {
    self.hl = hl;
  }

  pub fn bc(& mut self) -> u16 {
    ((self.b as u8 as u16) << 8 as u16 | self.c as u8 as u16)
  }

  pub fn set_bc(&mut self, bc: u16) {
    self.b = (bc >> 8) as i8;
    self.c = (bc & 0x00ff) as i8;
  }

  pub fn de(& mut self) -> u16 {
    ((self.d as u8 as u16) << 8 as u16 | self.e as u8 as u16)
  }

  pub fn get_hl_h(&mut self) -> i8 {
    ((self.hl & 0xff00) >> 8) as i8
  }

   pub fn get_hl_l(&mut self) -> i8 {
    (self.hl & 0x00ff) as i8
  }

  pub fn set_hl_l(&mut self, l: i8) {
    let h = self.hl & 0xff00;
    self.hl = h | l as u16;
  }

  pub fn set_hl_h(&mut self, h: i8) {
    let l = self.hl & 0x00ff;
    self.hl = (h as u16) << 8 | l;
  }

  pub fn exec(&mut self) {
    let op = self.mem.r8(self.pc) as u8;
    println!("Addr: {} , op:{}", format!("{:#x}", self.pc()), format!("{:#x}", op));
    self.step();
    match op {
        0x00 | 0xf3 => { self.nop(); },
        0x01 => { self.ld_bc_nn() },
        0x07 => {self.rlca()},
        0x09 | 0x19 => { self.add_hl_ss(op) },
        0x10 => { self.djnz() },
        0x12 => { self.ld_de_a()},
        0x20 => { self.jr_nz() },
        0x11 | 0x22 | 0x21 | 0x31 => { self.ld_dd_nn(op) },
        0x5 | 0x0d | 0x1d => { self.dec_r(op) },
        0x03 | 0x13 | 0x23 => {self.inc_ss(op)},
        0x2a => { self.ld_hl_nn() },
        0x1b | 0x2b | 0x0b => { self.dec_ss(op) },
        0x2c | 0x24 | 0x3c => { self.inc_r(op) },
        0x2f => { self.cpl() },
        0x32 => { self.ld_nn_a()},
        0x36 => { self.ld_hl_n()}
        0x3a => { self.ld_a_nn()},
        0x0e | 0x06 | 0x16 | 0x1e | 0x3e | 0x26 | 0x2e => { self.ld_r_n(op)},
        0xb6 => { self.or_hl() },
        0xce => { self.adc_r () },
        0x6f | 0x49 | 0x4f | 0x54 | 0x5d | 0x78 => { self.ld_r_r(op)},
        0x4e | 0x46 | 0x56 | 0x5e | 0x66 | 0x6e | 0x7e => { self.ld_r_hl(op)},
        0x51 | 0x5c | 0x64 | 0x65 | 0x6c
        | 0x61 | 0x62 | 0x63 | 0x68  => { self.ld_hh(op)},

        0x70 | 0x73 | 0x77 | 0x71 => { self.ld_hl_r(op)},
        0x76 => { self.halt()},
        0x83 | 0x87 | 0x80 | 0x81 | 0x82 | 0x84 | 0x85 => { self.add_a_r(op)},
        0x97 => { self.sub_r(op) },
        0xa1 => {self.and_r(op)},
        0xa9 | 0xaf => { self.xor_r(op) },
        0xb0 | 0xb4 => { self.or_r(op) },
        0xc0 | 0xc8 | 0xd0 | 0xd8 | 0xe0 | 0xe8 | 0xf0 | 0xf8 => {self.ret_cc(op)},
        0xc3 => { self.jmp(); },
        0xc6 => { self.add_a_n(); },
        0xc9 => {self.ret()},
        0xcd => { self.call() },
        0xd3 => { self.out(op) },
        0xd6 => {self.sub_n(op)},
        0xdd => {
          let op = self.mem.r8(self.pc) as u8;
          match op {
            0x21 => { self.ld_ix_nn() },
            0xE5 => { self.push_ix()},
            0xE1 => { self.pop_ix()},
            _ => {  panic!("unknown cp/m call {}!"); }
          }
        },
        0xeb => {self.ex_de_hl()},
        0xed => {self.ed()},
        0xfd => {
          let op = self.mem.r8(self.pc) as u8;
          match op {
            0x21 => { self.ld_iy_nn() },
            0xE5 => { self.push_iy()},
            0xE1 => { self.pop_iy()},
            _ => {  panic!("unknown cp/m call {}!"); }
          }
        },
        0xc1 | 0xd1 |0xf1 | 0xe1 => {self.pop_qq(op)},
        0xea | 0xe2 | 0xda | 0xc2 => { self.jp_cc_nn(op) },
        0xcc | 0xc4 | 0xec | 0xd4 | 0xdc | 0xe4| 0xf4 | 0xfc => { self.call_cc_nn(op) },
        0xf5 | 0xc5 | 0xd5 | 0xe5 => { self.push_qq(op) },
        0xff | 0xc7 | 0xcf | 0xdf => { self.rst_p(op) },
        0xf9 => { self.ld_sp_hl() },
        0xfb => { self.ei() },
        0xfe => { self.cp_n()}
        0xbf => { self.cp_r(op)}
        _ => {  panic!("unknown cp/m call {}!"); },
    }
  }

  /* I/O Instructions */
  pub fn out(&mut self, op: u8) {
    // TODO
    self.step();
  }

  pub fn ei(&mut self) {
    //TODO
    self.step();
  }
 
  pub fn and_r(&mut self, op: u8) {
    let a = self.a;
    let mut r = 0;
    match op {
      0xa1 => {
        r = a & self.c;
        self.set_a(r);
      },
       _ => {  panic!("unknown cp/m call {}!"); },
    }

    self.flags_set_n(false);
    self.flags_set_z(r == 0);
    self.flags_set_h(true);
    self.flags_set_s(r < 1);
    self.flags_set_pe(false);
  }

  pub fn cp_r(&mut self, op: u8) {
    let mut s = 0;
    match op {
      0xbf => {
        s = self.a;
      },
       _ => {  panic!("unknown cp/m call {}!"); },
    }
    let r = self.a - s;
    self.flags_set_n(true);
    self.flags_set_z(r == 0);
    let mut borrow = false;
    for mask in [0b000_0001, 0b0000_0010, 0b0000_0100, 0b0000_1000,
                0b0001_0000, 0b0010_0000, 0b0100_0000, 0b1000_0000].iter() {

      if (s as u8 & mask) > (self.b as u8 & mask) {
        borrow = true;
      }
    }
    self.flags_set_c(borrow);
    self.flags_set_s(r < 1);
    self.flags_set_pe(s > 0 && r > 0);
    // self.flags_set_h((n & 0xF0) + (a & 0xF0) & 0x10 == 0x10);
    self.step();
  }

  pub fn cp_n(&mut self) {
    let s = self.mem.r8(self.pc);
    let mut reg = 0;
    match s {
      0x01 => {
        reg = self.c;
      }
      0x0 => {
        reg = self.b;
      },
      _ => {  panic!("unknown cp/m call {}!"); },
    }

    let r = reg - s;
    self.flags_set_n(true);
    self.flags_set_z(r == 0);
    let mut borrow = false;
    for mask in [0b000_0001, 0b0000_0010, 0b0000_0100, 0b0000_1000,
                0b0001_0000, 0b0010_0000, 0b0100_0000, 0b1000_0000].iter() {

      if (s as u8 & mask) > (self.b as u8 & mask) {
        borrow = true;
      }
    }
    self.flags_set_c(borrow);
    self.flags_set_s(r < 1);
    self.flags_set_pe(s > 0 && r > 0);
    // self.flags_set_h((n & 0xF0) + (a & 0xF0) & 0x10 == 0x10);
    self.step();
  }

  /**
   * Extended Instructions, see more at http://clrhome.org/table/
   */
  pub fn ed(&mut self) {
    let op = self.mem.r8(self.pc) as u8;
    match op {
      0xb0 => {
        loop {
          let bc = self.ldir();
          if bc == 0 { break; }
          else {
            let pc = self.pc();
            self.set_pc(pc - 2);
          }
        }
      },
      0x47 => {
        self.i = op;
        self.step();
      },
      0x5e => {
        self.step();
      },

      _ => {  println!("op {:x}",op); panic!("unknown cp/m call {}!"); },
    }
  }

  /** Transfers a byte of data from the memory location pointed to by hl to the memory location pointed to by de.
      then hl and de are incremented and bc is decremented. If bc is not zero, this operation is repeated.
      Interrupts can trigger while this instruction is processing.
  */
  pub fn ldir(&mut self) -> u16 {
    let hl = self.hl;
    let de = self.de();
    let data = self.mem.r8(hl) as u8;
    self.mem.w8(de, data);
    self.set_hl(hl + 1);
    self.set_de(de + 1);
    let bc = self.bc();
    self.set_bc(bc - 1);
    return bc - 1;
  }

  pub fn ex_de_hl(&mut self) {
    let de = self.de();
    let hl = self.hl;
    self.set_de(hl);
    self.set_hl(de);
  }

  pub fn ld_nn_a(&mut self) {
    let addr = self.mem.r16(self.pc());
    self.mem.w8(addr, self.a as u8);
    self.step();
    self.step();
  }

  pub fn pop_ix(&mut self) {
    let data = self.mem.r16(self.sp);
    self.ix = data;
    self.set_sp(self.sp + 2);
    self.step();
  }

  pub fn pop_iy(&mut self) {
    let data = self.mem.r16(self.sp);
    self.iy = data;
    self.set_sp(self.sp + 2);
    self.step();
  }

  pub fn push_ix(&mut self) {
    self.push(self.ix);
    self.step();
  }

  pub fn push_iy(&mut self) {
    self.push(self.iy);
    self.step();
  }

  pub fn ld_ix_nn(&mut self) {
    let data = self.mem.r16(self.pc + 1);
    self.ix = data;
    self.step();
    self.step();
    self.step();
  }

  pub fn ld_iy_nn(&mut self) {
    let data = self.mem.r16(self.pc + 1);
    self.iy = data;
    self.step();
    self.step();
    self.step();
  }

  pub fn ld_de_a(&mut self) {
    let addr = self.de();
    self.mem.w8(addr, self.a as u8);
  }

  pub fn rlca(&mut self) {
    let r = (self.a as u8).rotate_left(1);
    self.flags_set_h(false);
    self.flags_set_n(false);
    self.flags_set_c((r & 0x01) == 0x01);
    self.set_a(r as i8);
  }

  pub fn ret(&mut self) {
    let data = self.mem.r16(self.sp);
    self.set_sp(self.sp.wrapping_add (2));
    self.set_pc(data);
  }

  pub fn ret_cc(&mut self, op: u8) {
    match op {
      0xc0 => {
        if !self.flags_get_z() {
           let data = self.mem.r16(self.sp);
           self.set_sp(self.sp + 2);
           self.set_pc(data);
        }
      },
      0xc8 => {
        // Ret Z
        if self.flags_get_z() {
           let data = self.mem.r16(self.sp);
           self.set_sp(self.sp + 2);
           self.set_pc(data);
        }
      },
      0xd0 => {
        // Ret NC
        if !self.flags_get_c() {
           let data = self.mem.r16(self.sp);
           self.set_sp(self.sp + 2);
           self.set_pc(data);
        }
      },
      0xd8 => {
        // Ret C
        if self.flags_get_c() {
           let data = self.mem.r16(self.sp);
           self.set_sp(self.sp + 2);
           self.set_pc(data);
        }
      },
      0xe0 => {
          // Ret PO
          if !self.flags_get_pe() {
           let data = self.mem.r16(self.sp);
           self.set_sp(self.sp + 2);
           self.set_pc(data);
          }
      },
      0xe8 => {
        // Ret PE
        if self.flags_get_pe() {
          let data = self.mem.r16(self.sp);
          self.set_sp(self.sp + 2);
          self.set_pc(data);
        }
      },
      0xf0 => {
        // Ret P
        if !self.flags_get_s() {
          let data = self.mem.r16(self.sp);
          self.set_sp(self.sp + 2);
          self.set_pc(data);
        }
      },
      0xf8 => {
        // Ret P
        if self.flags_get_s() {
          let data = self.mem.r16(self.sp);
          self.set_sp(self.sp + 2);
          self.set_pc(data);
        }
      },
      _ => {  panic!("unknown cp/m call {}!"); },
    }
  }

  pub fn sub_r(&mut self, op: u8) {
    let res = 0;
    let mut r = 0;
    match op {
      0x97 => {
        r = self.a as u8;
        self.set_a(0);
      },
      _ => {  panic!("unknown cp/m call {}!"); },
    }
    self.flags_set_s(res > 127);
    self.flags_set_z(res == 0);
    self.flags_set_n(true);
    self.flags_set_c(res > 255);
    self.flags_set_pe(res > 255);
    // self.flags_set_h((n & 0x0F) + (r & 0x0F) & 0x10 == 0x10);
  }

  pub fn add_a_n(&mut self) {
    let n = self.mem.r8(self.pc) as u8;
    let a = self.a as u8;
    let res = a as u16 + n as u16;
    self.set_a(res as i8);
    self.flags_set_s(res > 127);
    self.flags_set_z(n == 0);
    self.flags_set_n(false);
    self.flags_set_c(res > 255);
    self.flags_set_pe(res > 255);
    self.flags_set_h((n & 0x0F) + (a & 0x0F) & 0x10 == 0x10);

    self.step();
  }

  pub fn halt(&mut self) {

  }

  pub fn pop_qq(&mut self, op: u8) {
    let sel = (op & 0b0011_0000) >> 4 as u8;
    let data = self.mem.r16(self.sp);
    match sel {
      0b00 => {
          self.set_bc(data);
      },
      0b01 => {
          self.set_de(data);
      },
      0b10 => {
          self.set_hl(data);
      },
      0b11 => {
          self.set_af(data);
      }
      _ => {  panic!("unknown cp/m call {}!"); },
    }
    self.set_sp(self.sp + 2);
  }

  ///The contents of any register r' are loaded to any other register r.
  ///r, r' identifies any of the registers A, B, C, D, E, H, or L
  pub fn ld_r_r(&mut self, op: u8) {
    match op {
      0x6f => {
        let l = self.get_hl_l() as i8;
        self.set_a(l);
      },
      0x4f => {
        self.set_c(self.a);
      },
      0x54 => {
        let h = self.get_hl_h();
        self.set_d(h);
      },
      0x5d => {
        let l = self.get_hl_l() as i8;
        self.set_e(l);
      },
      0x78 => {
        self.set_a(self.b);
      }
      _ => {  panic!("unknown cp/m call {}!"); },
    }
  }

  pub fn inc_ss(&mut self, op: u8) {
    match op {
      0x23 => {
        let hl = self.hl as i16;
        self.set_hl((hl +1) as u16);
      },
      0x03 => {
        let bc = self.bc();
        self.set_bc(bc + 1);
      },
      0x13 => {
        let de = self.de();
        self.set_de(de + 1);
      },
      _ => {  panic!("unknown cp/m call {}!"); },
    }
  }

  pub fn ld_a_nn(&mut self) {
    let addr = self.mem.r16(self.pc());
    let data = self.mem.r8(addr);
    self.set_a(data);
    self.step();
    self.step();
  }

  pub fn ld_r_hl(&mut self, op: u8) {
    let data = self.mem.r8(self.hl);
    match op {
      0x4e => {
        self.set_c(data);
      },
      0x46 => {
        self.set_b(data);
      },
      0x7e => {
        self.set_a(data);
      },
      0x56 => {
        self.set_d(data);
      },
      0x5e => {
        self.set_e(data);
      },
      0x66 => {
        self.set_hl_h(data);
      },
      0x6e => {
        self.set_hl_l(data);
      },
      _ => {  panic!("unknown cp/m call {}!"); },
    }
  }

  /// opcode execution
  /***
   * the n integer is loaded to the memory address specified by the contents of the hl register pair.
   */
  pub fn ld_hl_n(&mut self) {
    let n = self.mem.r8(self.pc());
    let m = self.hl;
    self.mem.w8(m, n as u8);
    self.step();
  }

  pub fn cpl(&mut self) {
      let a = self.a;
      self.set_a(!a);
      self.flags_set_h(true);
      self.flags_set_n(true);
      self.step();
  }

  pub fn or_hl(&mut self) {
      let a = self.a;
      let _hl_ = self.mem.r8(self.hl);
      let result = a | _hl_;
      self.set_a(result);

      self.flags_set_s((result as i8) < 0);
      self.flags_set_z(result == 0);
      self.flags_set_n(false);
      self.flags_set_c(false);
      self.flags_set_h(false);
      let is_even = result & 0b000_0001 == 1;
      self.flags_set_pe(is_even); // fix !!!!
      self.step();
  }

  pub fn adc_r(&mut self) {
      let a = self.a;
      let n = self.mem.r8(self.pc());
      let c = self.flags_get_c();
      if c {
          self.set_a(a + n + 1);
          self.flags_set_s(a + n + 1 == 0);
          self.flags_set_z(a + n + 1 == 1);
          self.flags_set_pe((a + n + 1) as u8 > 127);
      } else {
          self.set_a(a + n);
          self.flags_set_s(a + n == 0);
          self.flags_set_z(a + n == 1);
          self.flags_set_pe((a + n) as u8 > 127);
      }
      // a = self.a;
      self.flags_set_n(false);
      self.flags_set_c(a == 0x7f);
      //h is set if carry from bit 3; otherwise, it is reset.
      self.flags_set_h((a & 0b0000_1111) == 0xf);
      self.step();
  }

  pub fn dec_r(&mut self, op: u8) {
      let sel = (op & 0b0111_0000) >> 4 as u8;
      let mut r = 0;
      match sel {
          0b000 => {
              r = self.b as i8;
              self.set_b(r -1);
          },
          0b001 => {
              r = self.c as i8;
              self.set_c(r -1);
          },
          0b010 => {
              r = self.d as i8;
              self.set_d(r -1);
          },
          0b011 => {
              r = self.e as i8;
              self.set_e(r -1);
          },
          0b100 => {
              r = self.get_hl_h() as i8;
              self.set_hl_h(r -1);
          },
          0b101 => {
              r = self.get_hl_l() as i8;
              self.set_hl_l(r -1);
          },
          0b111 => {
              r = self.a as i8;
              self.set_a(r -1);
          },
            _ => {
              panic!("unimplemented instruction");
          }
      }

      self.flags_set_s(r == 0);
      self.flags_set_z(r == 1);
      self.flags_set_n(true);
      self.flags_set_pe(r as u8 > 0x80);
      // self.step();
  }

  pub fn add_a_r( &mut self, op: u8) {
      let sel = (op & 0b0000_0111) as u8;
      let mut res: u16 = 0;
      let mut r: u8 = 0;
      let a = self.a as u8;

      match sel {
          0b111 => {
              r = self.a as u8;
              res = a as u16 + r as u16;
              self.set_a(res as i8);
          },
          0b000 => {
              r = self.b as u8;
              res = a as u16 + r as u16;
              self.set_a(res as i8);
          },
          0b001 => {
              r = self.c as u8;
              res = a as u16 + r as u16;
              self.set_a(res as u8 as i8);
          },
          0b010 => {
              r = self.d as u8;
              res = a as u16 + r as u16;
              self.set_a(res as i8);
          },
          0b011 => {
              r = self.e as u8;
              res = a as u16 + r as u16;
              self.set_a(res as i8);
          },
          0b100 => {
              r = self.get_hl_h() as u8;
              res = a as u16 + r as u16;
              self.set_a(res as i8);
          },
          0b101 => {
              r = self.get_hl_l() as u8;
              res = a as u16 + r as u16;
              self.set_a(res as i8);
          },
          _ => {
              panic!("unimplemented instruction");
          }
      }

    self.flags_set_s(res > 127);
    self.flags_set_z(r == 0);
    self.flags_set_n(false);
    self.flags_set_c(res > 255);
    self.flags_set_pe(res > 255);
    self.flags_set_h((r & 0x0F) + (a & 0x0F) & 0x10 == 0x10);

  }

  pub fn push_qq( &mut self, op: u8) {
      let sel = (op & 0b0011_0000) >> 4 as u8;
      match sel {
          0b11 => {
              let af = self.af();
              self.push(af);
          },
          0b00 => {
              let bc = self.bc();
              self.push(bc);
          },
          0b01 => {
              let de = self.de();
              self.push(de);
          },
          0b10 => {
              let hl = self.hl;
              self.push(hl);
          },
          _ => {
              panic!("unimplemented instruction");
          }
      }
  }

  pub fn call_cc_nn(&mut self, op: u8) {
    match op {
      0xcc => {
           if self.flags_get_z() {
            let addr = self.mem.r16(self.pc());
            let pc = self.pc();
            // Because this process is a 3-byte instruction,
            // the Program Counter was incremented by three before the push is executed.
            self.push(pc + 2);
            self.set_pc(addr);
          } else {
            self.step();
            self.step();
          }
      }
      0xc4 => {
           if !self.flags_get_z() {
            let addr = self.mem.r16(self.pc());
            let pc = self.pc();
            self.push(pc + 2);
            self.set_pc(addr);
          } else {
            self.step();
            self.step();
          }
      },
      0xd4 => {
           if !self.flags_get_c() {
            let addr = self.mem.r16(self.pc());
            let pc = self.pc();
            self.push(pc + 2);
            self.set_pc(addr);
          } else {
            self.step();
            self.step();
          }
      },
      0xdc => {
           if self.flags_get_c() {
            let addr = self.mem.r16(self.pc());
            let pc = self.pc();
            self.push(pc + 2);
            self.set_pc(addr);
          } else {
            self.step();
            self.step();
          }
      },
      0xf4 => {
          // f4: if sign positive (p), then push pc onto stack and put nn contents on pc
        if !self.flags_get_s() {
            let addr = self.mem.r16(self.pc());
            let pc = self.pc();
            self.push(pc + 2);
            self.set_pc(addr);
        } else {
          self.step();
          self.step();
        }
      },
      0xfc => {
          // f4: if sign positive (p), then push pc onto stack and put nn contents on pc
        if self.flags_get_s() {
            let addr = self.mem.r16(self.pc());
            let pc = self.pc();
            self.push(pc + 2);
            self.set_pc(addr);
        } else {
          self.step();
          self.step();
        }
      },
      0xe4 => {
        if !self.flags_get_pe() {
            let addr = self.mem.r16(self.pc());
            let pc = self.pc();
            self.push(pc + 2);
            self.set_pc(addr);
        } else {
          self.step();
          self.step();
        }
      }
      0xec => {
         if self.flags_get_pe() {
            let addr = self.mem.r16(self.pc());
            let pc = self.pc();
            self.push(pc);
            self.set_pc(addr);
        } else {
          self.step();
          self.step();
        }
      },
       _ => { panic!("unimplemented instruction");}
    }
  }

  pub fn sub_n(&mut self, op: u8) {
      let n = self.mem.r8(self.pc());
      let a = self.a;
      let res: i8 = a - n;
      self.set_a(res);
      self.flags_set_s(res < 0);
      self.flags_set_z(res == 0);
      self.flags_set_n(true);
      self.flags_set_pe(res as u8 > 0x80);

      let mut borrow = false;
      for mask in [0b000_0001, 0b0000_0010, 0b0000_0100, 0b0000_1000,
                    0b0001_0000, 0b0010_0000, 0b0100_0000, 0b1000_0000].iter() {

                    if (n as u8 & mask) > (a as u8 & mask) {
                      borrow = true;
                    }
              }
      self.flags_set_c(borrow);
      self.step();
  }

  pub fn call(&mut self) {
      let pc = self.pc();
      let addr = self.mem.r16(self.pc());
      self.push(pc);
      self.set_pc(addr);
  }

  pub fn add_hl_ss(&mut self, op: u8) {
    let hl = self.hl as i16;

    match op {
      0x09 => {
        let bc = self.bc() as i16;
        self.set_hl((hl + bc) as u16);

      },
      0x19 => {
        let de = self.de() as i16;
        self.set_hl((hl + de) as u16);
      },
      _ => {  panic!("unknown cp/m call {}!"); }
    }
    self.flags_set_n(false);
    self.flags_set_c((hl & 0b0100_0000_0000_0000) >> 14 == 1);
    self.flags_set_h((hl & 0b0000_1000_0000_0000) >> 10 == 1);
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
  pub fn jp_cc_nn(&mut self, op: u8) {
      let sel = (op & 0b00111000) >> 3 as u8;
      let addr = self.mem.r16(self.pc());

      match sel {
          0b000 => {
              if !self.flags_get_z() {
                  self.set_pc(addr)
              } else {


              }
          }
          0b101 => {
              if self.flags_get_pe() {
                  self.set_pc(addr);
              } else {


              }
          },
          0b011 => {
              if self.flags_get_c() {
                  self.set_pc(addr);
              } else {


              }
          },
          0b100 => {
              if !self.flags_get_pe() {
                  self.set_pc(addr);
              } else {


              }
          },
          _ => {
              panic!("unimplemented instruction");
          }
      }
  }

  pub fn ld_sp_hl(&mut self) {
      self.set_sp(self.hl);
  }

  /// if address 4545h contains 37h and address 4546h contains a1h,
  /// then upon the execution of an ld hl, (4545h) instruction,
  /// the hl register pair contains a137h.
  pub fn ld_hl_nn(&mut self) {
      let addr = self.pc();
      let nn = self.mem.r16(addr);
      let val = self.mem.r16(nn);
      self.set_hl(val);
      self.step();
      self.step();
  }

  pub fn ld_dd_nn(&mut self, op: u8) {
      let addr = self.pc();
      let nn = self.mem.r16(addr);

      match op {
          0x11 => {
              self.set_de(nn);
          },
          0x21 => {
              self.set_hl(nn);
          }
          0x22 => {
              self.set_hl(nn);
          },
          0x31 => {
              self.set_sp(nn);
          }
          _ => {  panic!("unknown cp/m call {}!"); }
      }
      self.step();
      self.step();
  }

  pub fn xor_r(&mut self, op: u8) {
      let sel = op & 0b0000_0111;
      let mut result: i8 = 0;
      match sel {
          0b101 => {
            let c = self.c;
            let a = self.a;
            result = c ^ a;
            self.set_a(result);
          },
          0b111 => {
            let a = self.a;
            result = a ^ a;
            self.set_a(result);
          },
          _ => {
            panic!("unimplemented instruction");
        }
      }
     
      // check flags
      self.flags_set_z(result == 0);
      self.flags_set_s(result < 0);
      self.flags_set_n(false);
      self.flags_set_c(false);
      self.flags_set_h(false);
  }

  pub fn or_r(&mut self, op: u8) {
    let sel = op & 0b0000_0111;
    let a = self.a;
    let mut result = 0;

    match sel {
      0b000 => {
        let b = self.b;
        result = b | a;
      },
      0b001 => {
        let c = self.c;
        result = c | a;
      },
      0b010 => {
        let d = self.d;
        result = d | a;
      },
      0b011 => {
        let e = self.e;
        result = e | a;
      },
      0b100 => {
        let h = self.get_hl_h();
        result = h | a;
      },
      0b101 => {
        let l = self.get_hl_l();
        result = l | a;
      },
      0b111 => {
        let a = self.a;
        result = a | a;
      },
       _ => {  panic!("unknown cp/m call {}!"); }
    }

    self.set_a(result);
    // check flags
    self.flags_set_z(result == 0);
    self.flags_set_s(result < 0);
    self.flags_set_n(false);
    self.flags_set_c(false);
    self.flags_set_h(false);
  }

  pub fn rst_p(&mut self, op: u8) {
      let pc = self.pc();
      self.push(pc);
      match op {
          0xff => {
              self.set_pc(0x0038);
          },
          0xc7 => {
              self.set_pc(0x0000);
          },
          0xdf => {
            self.set_pc(0x0018);
          },
          0xcf => {
            self.set_pc(0x0008);
          }
          _ => {
              println!("unimplemented instruction");
          }
      }
  }

  /*
  the 8-bit integer n is loaded to any register r, in which r identifies registers a, b, c, d, e, h, or l,
  assembled as follows in the object code:
  */

  pub fn ld_r_n(&mut self, op: u8) {
      let sel = (op & 0b00111110) >> 3 as u8;
      let addr = self.pc();
      let n = self.mem.r8(addr) as i8;
      match sel {
          0b111 => {
              self.set_a(n);
          },
          0b000 => {
              self.set_b(n);
          },
          0b001 => {
              self.set_c(n);
          },
          0b010 => {
              self.set_d(n);
          },
          0b011 => {
              self.set_e(n);
          },
          0b100 => {
              self.set_hl_h(n);
          },
          0b101 => {
              self.set_hl_l(n);
          },
          _ => {  panic!("unknown cp/m call {}!"); }
      }
      self.step();
  }

  /*
  ld   (hl), r
  the contents of register r are loaded to the memory location specified by the contents of the hl register pair.
  */
  pub fn ld_hl_r(&mut self, op: u8) {
    let r = op & 0b0000_0111;
    let addr = self.hl;
    match r {
      0b000_0111 => {
        self.mem.w8(addr, self.a as u8);
      },
      0b000_0000 => {
        self.mem.w8(addr, self.b as u8);
      },
      0b000_0001 => {
        self.mem.w8(addr, self.c as u8);
      },
      0b000_0010 => {
        self.mem.w8(addr, self.d as u8);
      },
      0b000_0011 => {
        self.mem.w8(addr, self.e as u8);
      },
      0b000_0100=> {
        let h = self.get_hl_h();
        self.mem.w8(addr,h as u8);
      },
      0b000_0101 => {
        let l = self.get_hl_l();
        self.mem.w8(addr,l as u8);
      },
      _ => {  panic!("unknown cp/m call {}!"); }
    }
  }

  pub fn ld_lb(&mut self) {
      self.set_hl_l(self.b);
  }

  pub fn ld_rr(&mut self, op: u8) {
      match op {
          0x6c => {
              /* ld  l,h */
              let h = self.get_hl_h();
              self.set_hl_l(h);
          },
          0x61 => {
              /* ld   h,c */
              let c = self.c;
              self.set_hl_h(c);
          },
          0x62 => {
              /* ld   h,d */
              self.set_hl_h(self.d);
          },
          0x63 => {
              /* ld   h,e */
              self.set_hl_h(self.e);
          }
          _ => {  panic!("unknown cp/m call {}!"); }
      }
  }

  pub fn inc_r(&mut self, op: u8) {
      let mut incr = 0;
      let mut r = 0;
      match op {
          0x24 => {
              /* inc  h */
              r = self.get_hl_h();
              incr = r + 1;
              self.set_hl_h(incr);
          }
          0x2c => {
              r = self.get_hl_l();
              if r == 127 {
                self.set_hl_l(0);
              } else {
                incr = r + 1;
                self.set_hl_l(incr);
              }
          },
          0x3c => {
              r = self.a;
              incr = r + 1;
              self.set_a(incr);

          },
          _ => {  panic!("unknown cp/m call {}!"); }

      }

      // update flags
      self.flags_set_z(incr == 0);
      self.flags_set_s((incr as i8) < 0);
      self.flags_set_n(false);
      self.flags_set_c(false);
      self.flags_set_h(false);
      self.flags_set_pe(r == 0x7f);
  }

  pub fn ld_bc_nn(&mut self) {

      let addr = self.pc();
      let data = self.mem.r16(addr);
      self.set_bc(data);

      self.step();
      self.step();
  }

  pub fn jr_nz(&mut self) {
      if !self.flags_get_z() {
          let addr = self.pc();
          let offset = self.mem.r8(addr);
          let displacement = (addr -1).wrapping_add(offset as u16);
          self.set_pc(displacement);
          self.step();
          self.step();
      } else {
        self.step();
      }
  }

  pub fn ld_hh(&mut self, op: u8) {
      let r = (op & 0b0011_1000) >> 3 as u8;
      let r1 = (op & 0b0000_0111) >> 3 as u8;
      match (r, r1) {
          (0b111, 0b000) => {
              self.set_a(self.b);
          },
          (0b111, 0b001) => {
              self.set_a(self.c);
          },
          (0b111, 0b010) => {
              self.set_a(self.d);
          },
          (0b111, 0b011) => {
              self.set_a(self.e);
          },
          (0b111, 0b100) => {
              let h = self.get_hl_h();
              self.set_a(h);
          },
          (0b111, 0b101) => {
              let l = self.get_hl_l();
              self.set_a(l);
          },
          (0b000, 0b111) => {
              self.set_b(self.a);
          },
          (0b000, 0b001) => {
              self.set_b(self.c);
          },
          (0b000, 0b010) => {
              self.set_b(self.d);
          },
          (0b000, 0b011) => {
              self.set_b(self.e);
          },
          (0b000, 0b100) => {
              let h = self.get_hl_h();
              self.set_b(h);
          },
          (0b000, 0b101) => {
              let l = self.get_hl_l();
              self.set_b(l);
          },
          (0b001, 0b111) => {
              self.set_c(self.a);
          },
          (0b001, 0b000) => {
              self.set_c(self.b);
          },
          (0b001, 0b010) => {
              self.set_c(self.d);
          },
          (0b001, 0b011) => {
              self.set_c(self.e);
          },
          (0b001, 0b100) => {
              let h = self.get_hl_h();
              self.set_c(h);
          },
          (0b001, 0b101) => {
              let l = self.get_hl_l();
              self.set_c(l);
          },
          (0b010, 0b111) => {
              self.set_d(self.a);
          },
          (0b010, 0b000) => {
              self.set_d(self.b);
          },
          (0b010, 0b001) => {
              self.set_d(self.c);
          },
          (0b010, 0b011) => {
              self.set_d(self.e)
          },
          (0b010, 0b100) => {
              let h = self.get_hl_h();
              self.set_d(h);
          },
          (0b010, 0b101) => {
              let l = self.get_hl_l();
              self.set_d(l);
          },
          (0b011, 0b111) => {
              self.set_e(self.a);
          },
          (0b011, 0b000) => {
              self.set_e(self.b);
          },
          (0b011, 0b001) => {
              self.set_e(self.c);
          },
          (0b011, 0b010) => {
              self.set_e(self.d);
          },
          (0b011, 0b011) => {
              self.set_e(self.e)
          },
          (0b011, 0b100) => {
              let h = self.get_hl_h();
              self.set_e(h);
          },
          (0b011, 0b101) => {
              let l = self.get_hl_l();
              self.set_e(l);
          },
          (0b100, 0b111) => {
              self.set_hl_h(self.a);
          },
          (0b100, 0b000) => {
              self.set_hl_h(self.b);
          },
          (0b100, 0b001) => {
              self.set_hl_h(self.c);
          },
          (0b100, 0b010) => {
              self.set_hl_h(self.d);
          },
          (0b100, 0b011) => {
              self.set_hl_h(self.e);
          },
          (0b100, 0b100) => {
              let h = self.get_hl_h();
              self.set_hl_h(h);
          },
          (0b100, 0b101) => {
              let l = self.get_hl_l();
              self.set_hl_l(l);
          },
          (0b101, 0b111) => {
              self.set_hl_l(self.a);
          },
          (0b101, 0b000) => {
              self.set_hl_l(self.b);
          },
          (0b101,0b001) => {
              self.set_hl_l(self.c);
          },
          (0b101,0b010) => {
              self.set_hl_l(self.d);
          },
          (0b101, 0b011) => {
              self.set_hl_l(self.e);
          },
          (0b101,0b100) => {
              let h = self.get_hl_h();
              self.set_hl_l(h);
          },
          (0b101, 0b101) => {
              let h = self.get_hl_h();
              self.set_hl_l(h);
          },
          _ => {  panic!("unknown cp/m call {}!"); }
      }

  }

  pub fn nop(&mut self) {

  }

  pub fn jmp(&mut self) {
      //this will make the pc point to the next two bytes of the instruction
      let addr = self.pc();
      let data = self.mem.r16(addr);
      self.set_pc(data);
  }

  pub fn djnz(&mut self) {
    let pc = self.pc();
    let b = self.b;
    let displacement = self.mem.r8(self.pc()) as u16;
    let r = b.wrapping_sub(1);
    if r == 0 {
      self.step();
    } else {
      self.set_pc(pc - 1);
      self.set_pc(pc.wrapping_add(displacement) as u16);
      self.step();
      // self.step();
    }
    self.set_b(r);
  }

  fn dec_ss(&mut self, op: u8) {
    match op {
      0x2b => {
        let hl = self.hl;
        let r = hl - 1;
        self.set_hl(r);
      },
      0x0b => {
        let bc = self.bc();
        let r = bc - 1;
        self.set_bc(r);
      },
      0x1b => {
        let de = self.de();
        let r = de - 1;
        self.set_de(r);
      },
       _ => {  panic!("unknown cp/m call {}!"); }
    }
  }
}