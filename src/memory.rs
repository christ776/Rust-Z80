pub struct Memory {
  work_ram: [i8; 16384]
}

impl Memory {

  pub fn new() -> Memory {
    Memory{
      work_ram: [0; 16384]
    }
  }

  pub fn write(&self) {

  }

  pub fn read(&self, addr: i16) -> i8 {
    return 0;
  }
}