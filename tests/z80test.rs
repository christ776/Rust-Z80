use Z80::z80::Z80;
use ::Z80::memory::PlainMemory;
use ::Z80::memory::Memory;

// From https://github.com/raxoft/z80test
// Not passing

//static CODE: &'static [u8] = include_bytes!("res/z80doc.out");
//static CODE: &'static [u8] = include_bytes!("res/z80ccf.out");
//static CODE: &'static [u8] = include_bytes!("res/z80docflags.out");
//static CODE: &'static [u8] = include_bytes!("res/z80flags.out");
//static CODE: &'static [u8] = include_bytes!("res/z80memptr.out");
static CODE: &'static [u8] = include_bytes!("resources/z80full.out");

const START: u16 = 0x8000;

#[test]
#[ignore]
fn z80test() {
    let mut cpu = Z80::new();
    let mut mem = PlainMemory::new_64k();

    // Load program
    mem.write(START, CODE);

    // Do nothing on 0x1601 and RST 0x10
    mem.w8(0x1601, 0xc9); // RET
    mem.w8(0x0010, 0xc9); // RET

    // Patch to run a single test
    let run_single_test = false;
    let single_test = 148;
    // if run_single_test {
    //     machine.poke16(0x802b, single_test); // ld bc, 0 to ld bc, test
    //     let mut test_start = machine.peek16(0x802e);
    //     println!("Test table {:x}", test_start);
    //     test_start += single_test*2;
    //     println!("Test table {:x}", test_start);
    //     machine.poke16(0x802e, test_start); // Move start
    //     machine.poke16(test_start + 2 , 0); // NUL terminate test
    // }

    cpu.r.pc = START;
    let mut msg = String::new();
    loop {
        cpu.exec(&mut mem);

        if cpu.r.pc == 0x0000 {
            println!("");
            break;
        }

        if cpu.r.pc == 0x0010 {
            let mut ch = cpu.r.a as char;
            if ch == '\r' {
                ch = '\n'
            } else if ch as u8 == 23 {
                ch = ' '
            } else if ch as u8 == 26 {
                ch = ' '
            } 
            //print!("{}[{}]", ch, ch as u8);
            print!("{}", ch);
            msg.push(ch);
        }
    }

    assert_eq!(true, msg.contains("CPU TESTS OK"));
}