#![allow(non_snake_case)]

/// In Z80 all 16-bit immedidates are encoded in the little-endian order of bytes,
/// meaning the byte that contains the least significant bits (LSB)
/// comes first and is followed by the byte that contains the most significant bits (MSB) of the value.

pub mod z80;
pub mod memory;

pub use crate::z80::*;

fn main() {
    match std::fs::read("./pacman/pacman.6e") {
        Ok(bytes) => { 
            let buffer: Vec<u8> = bytes;
            let mut cpu = z80::Z80::new();
            let temp: &[u8] = &buffer;
            cpu.mem.write(0x0000, temp);

            loop {
                cpu.exec();
                match cpu.pc() {
                    0x0000 => { break; },
                    _ => { },
                }
            }
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