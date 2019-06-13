pub struct Memory {
  work_ram: [u8; 16384]
}

impl Memory {

  pub fn new() -> Memory {
    Memory{
      work_ram: [0; 16384]
    }
  }

  pub fn w8f(&mut self, offset:u16, data:u8) {
    self.work_ram[offset as usize] = data;
  }

  pub fn write(&mut self, addr:u16, data: &[u8]) {
      let mut offset = 0;
      for b in data {
          self.w8f(addr + offset, *b);
          offset += 1;
      }
  }

  pub fn read(&self, addr: u16) -> u8 {
     self.work_ram[addr as usize]
  }
}