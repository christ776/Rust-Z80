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
  pub tile_rom: Vec<u8>, 
  pub sprite_rom: Vec<u8>, 
  pub memory_mapped_area: Vec<u8>,
  pub in0: u8,
  pub in1: u8,
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

    fn w16(&mut self, addr:u16, data: u16) {
      let l = (data & 0x00FF) as u8;
      let h = (data >> 8) as u8;
      self.w8(addr, l);
      self.w8(addr + 1, h);
    }

    fn w8(&mut self, address:u16, data:u8) {
      let address= address & 0x7fff;
      match address {
        0x0000..=0x3fff => {
          println!("Attempted to write to ROM!");
          panic!("Write Violation");
        }, 
        0x5001 => {
          println!("Sound enabled");
        },
        0x5002 => {
          // When the Ms. Pac-Man auxiliary board is connected on Pac-Man hardware in the Z80 slot,
          // writing a 1 to this address will enable the additional functionality provided by the
          // board, which is special behavior when reading memory locations.
          println!("Enable Aux board");
        },
        0x5003 => {
          println!("Flip screen");
        },
        0x5004 => {
          // println!("1 player start lamp")
        },
        0x5005 => {
          // println!("2 player start lamp")
        },
        0x5006 => {
          // println!("Coin lockout")
        },
        0x5007 => {
          // println!("Coin counter")
        },
        0x5040..=0x504f => {
          //Sound voice 1
        },
        0x5050..=0x505f => {
          //Voice 1 frequency
        }
        0x5060..=0x506f => {
          //Write Sprite coordinates
          // self.decoder.update_sprite_coordinates((address - 0x5060).into(), data);
          self.memory_mapped_area[(address - 0x5060) as usize] = data;
        }
        // },
        // 0x5040..=0x505f => {    
        //     println!("Sound tests at {} with {}", format!("{:#x}", address), data)
        // },
        0x5070..=0x50bf => {    
          println!("??? {} with {}", format!("{:#x}", address), data)
        },
        0x50c0..=0x50ff => {    
          // Watchdog reset (each byte has the same function)
          // This would write values that the watchdog would look for to determine if the game
          // code had locked up or not. Since I'm not implementing the watchdog hardware I don't
          // need to implement this.
        },
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

    fn r16(&self, addr: u16) -> u16 {
        let l:u16 = self.work_ram[addr as usize].into();
        let h: u16 = self.work_ram[(addr +1) as usize].into();
        h << 8 | l
    }

    fn r8(&self, addr: u16) -> u8 {
      match addr {
        0x5000..=0x503f => { // Read IN0: Joystick and coin slot
          self.in0
        },
        0x5040..=0x507f => {
          self.in1 // IN1: Read IN1: Joystick and coin slot
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
        _ => self.work_ram[addr as usize]
      }
  }

    fn new() -> BoardMemory {
      BoardMemory{
        work_ram: Vec::new(),
        tile_rom: Vec::new(),
        sprite_rom: Vec::new(),
        memory_mapped_area: vec![0; 16],
        in0: 0b1111_1111,
        in1: 0b1111_1111,
      }
    }

    fn new_64k() -> BoardMemory {
      BoardMemory { 
        work_ram: vec![0; 65536],
        tile_rom: vec![],
        sprite_rom: vec![],
        memory_mapped_area: vec![],
        in0: 0b1111_1111,
        in1: 0b1111_1111,
      }
    }
}
