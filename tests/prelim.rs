mod prelim {

    use Z80::z80::Z80;
    use ::Z80::memory::PlainMemory;
    use ::Z80::memory::Memory;
    use ::Z80::registers::{ Register16Bit };

    static PRELIM: &'static [u8] = include_bytes!("resources/prelim.com");

    #[test]
    fn test_prelim() {
        let mut tests_passed = 0;
        let mut cpu = Z80::new();
        let mut mem = PlainMemory::new_64k();
        mem.write(0x0100, PRELIM);
        cpu.r.sp = 0xF000;
        cpu.r.pc = 0x0100;
        /*
        System call 5

        .org $5
            out ($0), a
            ret
        */
        let code = [0xD3, 0x00, 0xC9];
        for i in 0..code.len() {
             mem.w8(5 + i as u16, code[i]);
        }
    
        loop {
            cpu.exec(&mut mem);
            if cpu.r.pc == 0x0000 {
                println!("");
                break;
            }
            match cpu.r.pc {
                0x0005 => { cpm_bdos(&mut cpu, &mut tests_passed, &mem); },
                0x0000 => { break; },
                _ => { },
            }
        }

        // assert_eq!(25, tests_passed);
    }

    fn cpm_bdos(cpu: &mut Z80, tests_passed: &mut u8, mem: &PlainMemory) {
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
                    let c = mem.r8(addr) as char;
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
            _ => panic!("Unknown BDOS call {}! at PC: {}", cpu.r.c, format!("{:#x}", cpu.r.pc)) 
        }
    }
}
