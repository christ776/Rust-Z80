pub struct Z80 {
  pub a: i8,
  pub b: i8,
  pub mem: crate::memory::Memory
}

impl Z80 {
  pub fn new() -> Z80 {
    Z80{ a:0, b:0, mem: crate::memory::Memory::new() }
  }

  pub fn step(&self) {

  }

  pub fn pc(&self) -> u16 {
    return 0;
  }

  pub fn c(&self) -> u16 {
    return 0;
  }

   pub fn e(&self) -> u16 {
    return 0;
  } 

   pub fn de(&self) -> u16 {
    return 0;
  }
}