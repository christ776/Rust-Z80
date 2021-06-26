pub struct  RomLoader {}

impl RomLoader {
    pub fn load_rom_mut(rom_name: &String, mem: &mut Vec<u8>) {
        match std::fs::read(rom_name) {
            Ok(bytes) => { 
                let mut buffer: Vec<u8> = bytes;            
                mem.append(&mut buffer);
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::PermissionDenied {
                    eprintln!("please run again with appropriate permissions.");
                    return;
                }
                panic!("{}", e);
            }
        }
    }
    
    pub fn as_u32_be(array: &[u8; 4]) -> u32 {
        ((array[0] as u32) << 24) +
        ((array[1] as u32) << 16) +
        ((array[2] as u32) <<  8) +
        ((array[3] as u32) <<  0)
    }
}