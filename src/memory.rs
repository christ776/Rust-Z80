use crate::gfx_decoder::TileDecoder;

pub struct Memory {
  pub work_ram: Vec<u8>,
  pub tile_rom: Vec<u8>,
  pub pixel_buffer: Vec<u32>
}

impl Memory {

    pub fn new() -> Memory {
      Memory{
        work_ram: Vec::new(),
        tile_rom: Vec::new(),
        pixel_buffer: Vec::new()
      }
    }

    pub fn new_64k() -> Memory {
      Memory { 
        work_ram: vec![0; 65536],
        tile_rom: vec![0; 65536],
        pixel_buffer: vec![0]
      }
    }

    pub fn new_1k() -> Memory {
      Memory { 
        work_ram: vec![0; 1024],
        tile_rom: vec![0; 1024],
        pixel_buffer: vec![0]
      }
    }

    pub fn w8(&mut self, address:u16, data:u8) {
      match address {
        0x4000..=0x43de => {
          // println!("Video RAM: accessed {} with {}", format!("{:#x}", address), data);
          let offset = address - 0x4000;
          TileDecoder::decode_tile(offset as usize, &self.tile_rom, &mut self.pixel_buffer);
        },
        // 0x5000 => {
        //   println!("IO: accessed {} with {}", format!("{:#x}", address), data);
        //   self.work_ram[address as usize] = data;
        // },
        // 0x50c0..=0x50ff => 
        //   println!("Kicking the watchdog at {} with {}", format!("{:#x}", address), data),
        0x5040..=0x505f => {    
            println!("Sound tests at {} with {}", format!("{:#x}", address), data)
        },
        0x5060..=0x506f => {    
          println!("Sprite coordinates at {} with {}", format!("{:#x}", address), data);
          //let offset = address - 0x4000;
          //TileDecoder::decode_sprite(offset as usize, &self.tile_rom[4096], &mut self.pixel_buffer)
        },
        0x5070..=0x50bf => {    
          println!("??? {} with {}", format!("{:#x}", address), data)
        },
        0x50c0..=0x50ff => {    
          // println!("Watchdog reset")
        },
        _ => self.work_ram[address as usize] = data,
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
      match addr {
        0x5040..=0x507F => {
          0x10
        }, 
        _ => self.work_ram[addr as usize] as i8
      }
  }
}