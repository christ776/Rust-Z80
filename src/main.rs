#![allow(non_snake_case)]

/// In Z80 all 16-bit immedidates are encoded in the little-endian order of bytes,
/// meaning the byte that contains the least significant bits (LSB)
/// comes first and is followed by the byte that contains the most significant bits (MSB) of the value.

pub mod z80;
pub mod memory;

use std::time::Duration;
use std::time::Instant;
pub use crate::z80::*;

const CPU_FREQ:u64 = 3_072_000;

fn main () {

    //alloc memory
    let mut mem = Memory::new();

    // Load ROM contents 
    load_rom_mut(&String::from("./pacman/pacman.6e"), &mut mem.work_ram);
    load_rom_mut(&String::from("./pacman/pacman.6f"), &mut mem.work_ram);
    load_rom_mut(&String::from("./pacman/pacman.6h"), &mut mem.work_ram);
    load_rom_mut(&String::from("./pacman/pacman.6j"), &mut mem.work_ram);
    // Working RAM ... it's a bit of a hack for now
    let mut video_ram:Vec<u8> = vec![0; 2048];
    &mem.work_ram.append(&mut video_ram);
    let mut working_ram:Vec<u8> = vec![0; 4196];
    &mem.work_ram.append(&mut working_ram);
    println!("Memory size is {}", format!("{:#x}", mem.work_ram.len()));
    let mut cpu = z80::Z80::new(mem);

    // For throttling to 60 FPS
    let mut timer = Instant::now();
    let mut fps_timer = Instant::now();
    let mut fps = 0;
    let cycles_per_frame = CPU_FREQ / 60;
    let mut available_cycles = cycles_per_frame;

    loop {
        let cycles = cpu.exec();
        if available_cycles < cycles as u64 {
            // if we're running faster than 60Hz, kill time
            ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
        } else {
            available_cycles -= cycles as u64;
        }
       
    }
}

fn load_rom_mut(rom_name: &String, mem: &mut Vec<u8>) {
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



fn cpm_bdos(cpu: &mut Z80) {
        // get the function call id from register C
    match cpu.c {
        2 => {
            // output character in register E
            print!("{}", cpu.e as u8 as char);
        },
        9 => {
            // output a string at register DE until '$'
            let mut addr = cpu.de();
            loop {
                let c = cpu.mem.r8(addr) as u8;
                if c != 0x24 {
                    addr = addr + 1;
                    print!("{}", c as char);
                }
                else {
                    break;
                }
            }
        },
        _ => {
            panic!("Unknown CP/M call {}!", cpu.c);
        }
    }
    cpu.ret();
}