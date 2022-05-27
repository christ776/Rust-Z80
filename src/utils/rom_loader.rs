use std::{collections::HashMap, ffi::OsStr, fs::File, io::{Error, Read}, path::Path};

use zip::read::ZipArchive;

pub struct  RomLoader {}

impl RomLoader {
    pub fn load_rom_mut(rom_name: &String, mem: &mut Vec<u8>) {
        match std::fs::read(rom_name) {
            Ok(bytes) => { 
                let buffer: Vec<u8> = bytes;            
                mem.extend(buffer);
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

    pub fn file_reader(filename: &str, rom_contents: Vec<&str>) -> Result<HashMap<String, Vec<u8>>, Error> {
        let path = Path::new(filename);
        let mut rom_map = HashMap::new();
        let file = match File::open(&path) {
            Ok(file) => file,
            Err(why) => return Err(why),
        };
    
        if path.extension() == Some(OsStr::new("zip")) {
            let mut archive_contents = ZipArchive::new(file)?; 
            for item in rom_contents {
                let mut archive_file = archive_contents.by_name(item)?;
                    // Read the contents of the file into a vec.
                    let mut data = Vec::new();
                    archive_file.read_to_end(&mut data)?;
                    rom_map.insert(item.to_string(), data);
                };
        };
        Ok(rom_map)
    }

    pub fn load_rom_item(item_name: &str, rom_contents: &HashMap<String, Vec<u8>>, mem: &mut Vec<u8>, at_position: usize) {
        if let Some(data) = rom_contents.get(&item_name.to_string()) {
            mem.splice(at_position..at_position + data.len(),
             data.iter().cloned());
        } else {
            println!("Error!!")
        }
    }
}