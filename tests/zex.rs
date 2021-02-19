mod zex {

    use Z80::z80::Z80;
    use ::Z80::memory::Memory;
    use ::Z80::registers::{ Register16Bit };

    static ZEXDOC: &'static [u8] = include_bytes!("resources/zexdoc.com");
    static ZEXALL: &'static [u8] = include_bytes!("resources/zexall.com");

    fn run_zex(prog: &'static [u8]) {
        let mut cpu = Z80::new(Memory::new_64k());
        cpu.mem.write(0x0100, prog);
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

    #[test]
    #[ignore]
    fn test_zex() {
        println!("Running ZEXDOC >>>");
        run_zex(ZEXDOC);
        // println!("Running ZEXALL >>>");
        // run_zex(ZEXALL);
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
                    addr += 1;
                    if c == '$' {
                        break;
                    }
                    msg.push(c);
                }
                print!("{}", msg);
            },
            _ => panic!("Unknown CP/M call {}!", cpu.r.c)
            
        }
        cpu.ret();
    }
}
