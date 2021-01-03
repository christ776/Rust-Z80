// extern crate memory;

// mod memory;

pub struct Memory {
  // work_ram: [u8; 65536]
  pub work_ram: Vec<u8>
}

impl Memory {

  pub fn new() -> Memory {
    Memory{
      work_ram: Vec::new()
    }
  }

  pub fn new_64k() -> Memory {
    Memory { 
      work_ram: vec![0; 65536]
    }
  }

  pub fn new_1k() -> Memory {
    Memory { 
      work_ram: vec![0; 1024]
    }
  }

  pub fn w8(&mut self, offset:u16, data:u8) {
    match offset {
      0x5000..=0x5007 => 
        println!("IO: accessed {} with {}", format!("{:#x}", offset), data),
      0x50c0..=0x50ff => 
        println!("Kicking the watchdog at {} with {}", format!("{:#x}", offset), data),
      0x5040..=0x505f => {    
          println!("Sound tests at {} with {}", format!("{:#x}", offset), data)
      },
      0x5060..=0x506f => {    
        println!("Sprite coordinates at {} with {}", format!("{:#x}", offset), data)
      },
      0x5070..=0x50bf => {    
        println!("??? {} with {}", format!("{:#x}", offset), data)
      },
      _ => self.work_ram[offset as usize] = data,
    }
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

  pub fn r8(&self, addr: u16) -> i8 {
    self.work_ram[addr as usize] as i8
  }
}