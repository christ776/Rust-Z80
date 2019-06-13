pub struct Z80 {
  pub a: u8,
  pub b: u8,
  pub c: u8,
  pub pc: u16,
  pub mem: crate::memory::Memory
}

impl Z80 {
  pub fn new() -> Z80 {
    Z80{ a:0, 
          b:0, 
          c:0,
          pc: 0, 
          mem: crate::memory::Memory::new() 
        }
  }

  pub fn step(&mut self) {
    self.pc += 1;
  }

  pub fn pc(&self) -> u16 {
    self.pc
  }

  pub fn c(&self) -> u8 {
    self.c
  }

   pub fn e(&self) -> u16 {
     0
  } 

   pub fn de(&self) -> u16 {
     0
  }
}