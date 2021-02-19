mod zex {

    use Z80::z80::Z80;
    use ::Z80::memory::Memory;
    use ::Z80::registers::{ Register16Bit };

    static PROG: &'static [u8] = include_bytes!("resources/8080EX1.COM");

    #[test]
    fn test_ex8080() {
        let mut tests_passed = 0;
        let mut cpu = Z80::new(Memory::new_64k());
        cpu.mem.write(0x0100, PROG);
        cpu.r.sp = 0xF000;
        cpu.r.pc = 0x0100;
        cpu.mem.work_ram[5] = 0xc9;
    
        loop {
            cpu.exec();
            match cpu.r.pc {
                0x0005 => { cpm_bdos(&mut cpu, &mut tests_passed); },
                0x0000 => { break; },
                _ => { },
            }
        }
        assert_eq!(25, tests_passed);
    }

    fn cpm_bdos(cpu: &mut Z80, tests_passed: &mut u8) {
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
                if msg.contains("OK") {
                    *tests_passed += 1;
                }
               
                print!("{}", msg);
            },
            _ => panic!("Unknown CP/M call {}!", cpu.r.c)
            
        }
        cpu.ret();
    }
}
