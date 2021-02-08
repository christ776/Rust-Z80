use Z80::z80::Z80;
use ::Z80::memory::Memory;
use ::Z80::registers::{ Register16Bit };


static ZEXDOC: &'static [u8] = include_bytes!("zexdoc.com");
static ZEXALL: &'static [u8] = include_bytes!("zexall.com");

#[test]
fn test_zexdoc() {
    let mut cpu = Z80::new(Memory::new_64k());
    cpu.mem.write(0x0100, ZEXDOC);
    cpu.r.sp = 0xF000;
    cpu.r.pc = 0x0100;

    loop {
        cpu.exec();
        match cpu.r.pc {
            0x0005 => { cpm_bdos(&mut cpu); },
            0x0000 => { break; },
            _ => { },
        }
    }
}

fn cpm_bdos(cpu: &mut Z80) {
    match cpu.r.c {
        2 => {
            // output a character
            print!("{}", cpu.r.e as char);
        },
        9 => {
            // output a string
            let mut addr = cpu.r.get_u16(Register16Bit::DE);
            let mut msg = String::new();
            loop {
                let c = cpu.mem.r8(addr) as char;
                addr = (addr + 1) & 0xFFFF;
                if c != '$' {
                    msg.push(c);
                }
                else {
                    break;
                }
            }
            print!("{}", msg);
        },
        _ => {
            panic!("Unknown CP/M call {}!", cpu.r.c);
        }
    }
    cpu.ret();
}