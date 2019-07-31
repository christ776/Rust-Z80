// extern crate memory;

// mod memory;

pub struct Memory {
  work_ram: [u8; 65536]
}

impl Memory {

  pub fn new() -> Memory {
    Memory{
      work_ram: [0; 65536]
    }
  }

  pub fn w8(&mut self, offset:u16, data:u8) {
    self.work_ram[offset as usize] = data;
  }

  pub fn write(&mut self, addr:u16, data: &[u8]) {
    let mut offset = 0;
    for b in data {
        self.w8(addr + offset, *b);
        offset += 1;
    }
  }

  pub fn w16(&mut self, addr:u16, data: u16) {
    let l = (data & 0x00FF) as u8;
    let h = (data >> 8) as u8;
    self.w8(addr, l);
    self.w8(addr + 1, h);
  }

  pub fn r16(&mut self, addr: u16) -> u16 {
    let l:u16 = self.work_ram[addr as usize].into();
    let h: u16 = self.work_ram[(addr +1) as usize].into();
    h << 8 | l
  }

  pub fn r8(&self, addr: u16) -> u8 {
     self.work_ram[addr as usize]
  }
}