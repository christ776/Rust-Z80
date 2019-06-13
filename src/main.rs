#![allow(non_snake_case)]

pub mod z80;
pub mod memory;

pub use crate::z80::*;

fn main() {
    static ZEXDOC: &'static [u8] = include_bytes!("zexdoc.com");
    let mut cpu = z80::Z80::new();
    cpu.mem.write(0x0100, ZEXDOC);
    loop {
        cpu.step();
        match cpu.pc() {
            0x0005 => { cpm_bdos(&mut cpu); },
            0x0000 => { break; },
            _ => { },
        }
    }
}

fn cpm_bdos(cpu: &Z80) {
        // get the function call id from register C
    match cpu.c() {
        2 => {
            // output character in register E
            print!("{}", cpu.e() as u8 as char);
        },
        9 => {
            // output a string at register DE until '$'
            let mut addr = cpu.de();
            loop {
                let c = cpu.mem.read(addr) as char;
                addr = (addr + 1) & 0xFF;
                if c != '$' {
                    print!("{}", c);
                }
                else {
                    break;
                }
            }
        },
        _ => {
            panic!("Unknown CP/M call {}!", cpu.c());
        }
    }
}
