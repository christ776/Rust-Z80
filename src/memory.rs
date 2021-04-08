
use crate::WIDTH;
// use crate::gfx_decoder::Decoder;
use crate::gfx_decoder::TileDecoder;

pub trait Memory {
  fn w16(&mut self, addr:u16, data: u16);
  fn w8(&mut self, address:u16, data:u8);
  fn write(&mut self, addr:u16, data: &[u8]);
  fn r16(&self, addr: u16) -> u16;
  fn r8(&self, addr: u16) -> u8;
  fn new() -> Self where Self: Sized;
  fn new_64k() -> Self where Self: Sized;
}


pub struct BoardMemory {
  pub work_ram: Vec<u8>,
  // pub pixel_buffer: Vec<u32>,
  pub tile_rom: Vec<u8>, 
  pub sprite_rom: Vec<u8>, 
  pub decoder: TileDecoder,
}

pub struct PlainMemory {
  ram: Vec<u8>
}

impl Memory for PlainMemory {

  fn new() -> Self {
    Self {
      ram: Vec::new(),
    }
  }

  fn new_64k() -> Self {
    Self { 
      ram: vec![0; 65536],
    }
  }

  fn r16(&self, addr: u16) -> u16 {
    match addr {
      0x5555 => 0,
      _ => {
        let l:u16 = self.ram[addr as usize].into();
        let h: u16 = self.ram[(addr +1) as usize].into();
        h << 8 | l
      }
    }
  }

  fn w8(&mut self, address:u16, data:u8) {
    self.ram[address as usize] = data;
  }

  fn write(&mut self, addr:u16, data: &[u8]) {
    let mut offset = 0;
    for b in data {
        self.w8(addr + offset, *b);
        offset += 1;
    }
  }

  fn w16(&mut self, addr:u16, data: u16) {
    let l = (data & 0x00FF) as u8;
    let h = (data >> 8) as u8;
    self.w8(addr, l);
    self.w8(addr + 1, h);
  }

  fn r8(&self, addr: u16) -> u8 {
     self.ram[addr as usize]
  }
}

impl Memory for BoardMemory {

    fn new() -> BoardMemory {
      BoardMemory{
        work_ram: Vec::new(),
        tile_rom: Vec::new(),
        sprite_rom: Vec::new(),
        decoder: TileDecoder::new(WIDTH),
      }
    }

    fn new_64k() -> BoardMemory {
      BoardMemory { 
        work_ram: vec![0; 65536],
        tile_rom: vec![0],
        sprite_rom: vec![0],
        decoder: TileDecoder::new(WIDTH)
      }
    }

    fn w8(&mut self, address:u16, data:u8) {
      let address= address & 0x7fff;
      match address {
        0x0000..=0x3fff => {
          println!("Attempted to write to ROM!");
          // panic!("Write Violation");
        },     
        // },
        // 0x5040..=0x505f => {    
        //     println!("Sound tests at {} with {}", format!("{:#x}", address), data)
        // },
        // 0x5070..=0x50bf => {    
        //   println!("??? {} with {}", format!("{:#x}", address), data)
        // },
        // 0x50c0..=0x50ff => {    
        //   // println!("Watchdog reset")
        // },
        _ => self.work_ram[address  as usize] = data,
      }
    }

    fn write(&mut self, addr:u16, data: &[u8]) {
      let mut offset = 0;
      for b in data {
          self.w8(addr + offset, *b);
          offset += 1;
      }
    }

    fn w16(&mut self, addr:u16, data: u16) {
      let l = (data & 0x00FF) as u8;
      let h = (data >> 8) as u8;
      self.w8(addr, l);
      self.w8(addr + 1, h);
    }

    fn r16(&self, addr: u16) -> u16 {
        let l:u16 = self.work_ram[addr as usize].into();
        let h: u16 = self.work_ram[(addr +1) as usize].into();
        h << 8 | l
    }

    fn r8(&self, addr: u16) -> u8 {
      let address= addr & 0x7fff;
      match addr {
        0x5000 => { // Read IN0: Joystick and coin slot
          0b1111_1111
        },
        0x5040 => {
          0b1111_1111 // IN1
        },
        0x5080..=0x50bf => {
          0xc9 //Dip-Switch byte
        }
        0x4400..=0x47ff => {
          0x7f
        },
        // 0x5100..=0xffff => {
        //   0
        // }
        _ => self.work_ram[address as usize]
      }
  }
}