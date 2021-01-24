use crate::gfx_decoder::Decoder;
use crate::gfx_decoder::TileDecoder;

pub struct Memory {
  pub work_ram: Vec<u8>,
  pub tile_rom: Vec<u8>,
  pub pixel_buffer: Vec<u32>,
  decoder:TileDecoder,
  pub sprite_rom: Vec<u8>,
  pub video_ram: Vec<u8>,
}

impl Memory {

    pub fn new(tile_decoder: TileDecoder) -> Memory {
      Memory{
        work_ram: Vec::new(),
        tile_rom: Vec::new(),
        decoder: tile_decoder,
        sprite_rom: Vec::new(),
        pixel_buffer: vec![0; 64512],
        video_ram: vec![0;2048],
      }
    }

    pub fn new_64k(tile_decoder: TileDecoder) -> Memory {
      Memory { 
        work_ram: vec![0; 65536],
        tile_rom: vec![0; 65536],
        video_ram: vec![0; 1024],
        pixel_buffer: vec![0],
        sprite_rom: vec![0; 1024],
        decoder: tile_decoder
      }
    }

    pub fn new_1k(tile_decoder: TileDecoder) -> Memory {
      Memory { 
        work_ram: vec![0; 1024],
        tile_rom: vec![0; 1024],
        video_ram: vec![0; 1024],
        pixel_buffer: vec![0],
        sprite_rom: vec![0; 1024],
        decoder: tile_decoder
      }
    }

    pub fn w8(&mut self, address:u16, data:u8) {
      match address {
        0x4000..=0x43de => {
          let offset = address - 0x4000;
          // println!("Video RAM: accessed {} with {}", format!("{:#x}", offset), format!("{:#x}", data));
          self.video_ram[offset as usize] = data;
          self.decoder.decode_tile(offset as usize, &self.tile_rom, data as usize, &mut self.pixel_buffer);
        },
        0x4400..=0x47ff => {
          let offset = address - 0x4000;
          self.video_ram[offset as usize] = data;
          // println!("Palette RAM: accessed {} with {}", format!("{:#x}", address), data);
        },
        // 0x5000..=0x5007=> {
        //   // println!("IO: accessed {} with {}", format!("{:#x}", address), data);
        // },
        // 0x50c0..=0x50ff => 
        //   println!("Kicking the watchdog at {} with {}", format!("{:#x}", address), data),
        0x5040..=0x505f => {    
            println!("Sound tests at {} with {}", format!("{:#x}", address), data)
        },
        0x5060..=0x506f => {    
          match self.work_ram.get(0x4ff0 ..0x4fff) {
            Some(sprite_positions) => self.decoder.decode_sprite(address as usize, sprite_positions, &self.sprite_rom, data as usize, &mut self.pixel_buffer),
            None => println!("Error decoding Sprite positions?")
          }
        },
        0x5070..=0x50bf => {    
          println!("??? {} with {}", format!("{:#x}", address), data)
        },
        0x50c0..=0x50ff => {    
          // println!("Watchdog reset")
        },
        _ => self.work_ram[(address & 0x7FFF) as usize] = data,
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

    pub fn r16(&self, addr: u16) -> u16 {
      match addr {
        0x5555 => 0,
        _ => {
          let address = addr & 0x7FFF;
          let l:u16 = self.work_ram[address as usize].into();
          let h: u16 = self.work_ram[(address +1) as usize].into();
          h << 8 | l
        }
      }
    }

    pub fn r8(&self, addr: u16) -> i8 {
      let address = (addr & 0x7FFF) as u16;
      match addr {
        0x5000 => { // Read IN0: Joystick and coin slot
          0b0000_0000 
        },
        0x5040 => {
          0b0000_0000 // IN1
        },
        0x5080 => {
          0b0001_0011 //Dip-Switch byte
        }
        0x5041..=0x507F => {
          0b0000_0000 
        }, 
        0x4400..=0x47ff => {
          println!("Reading Palette RAM");
          0x7f
        },
        _ => self.work_ram[address as usize] as i8
      }
  }
}