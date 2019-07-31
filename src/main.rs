#![allow(non_snake_case)]

/// In Z80 all 16-bit immedidates are encoded in the little-endian order of bytes,
/// meaning the byte that contains the least significant bits (LSB)
/// comes first and is followed by the byte that contains the most significant bits (MSB) of the value.

pub mod z80;
pub mod memory;

pub use crate::z80::*;

fn main() {
    static ZEXDOC: &'static [u8] = include_bytes!("zexdoc.com");
    let mut cpu = z80::Z80::new();
    cpu.mem.write(0x0100, ZEXDOC);
    cpu.set_sp(0xFFFF);
    cpu.set_pc(0x0100);

    loop {
        let op = cpu.mem.r8(cpu.pc());
        println!("{}", format!("{:#X}", op));
        cpu.step();
        match op {
            0x00 => { nop(&mut cpu); },
            0x01 => { LD_BC_nn(&mut cpu) },
            0x09 => { ADD_HL_BC(&mut cpu) },
            0x20 => { jr_nz(&mut cpu) },
            0x11 | 0x22 | 0x21 => { LD_dd_nn(op, &mut cpu) },
            0x1D => { DEC_r(op, &mut cpu) },
            0x2A => { LD_HL_nn(&mut cpu) },
            0x2B => { DEC_ss(&mut cpu) },
            0x2C | 0x24 | 0x3C => { INC_r(op, &mut cpu) },
            0x2F => { CPL(&mut cpu) },
            0x06 | 0x3E | 0x2E => { LD_r_n(op, &mut cpu) },
            0xB6 => { OR_HL(&mut cpu) },
            0xCE => { ADC_r (&mut cpu) },
            0x51 | 0x5C | 0x64 | 0x65 | 0x6C
            | 0x61 | 0x62 | 0x63 | 0x68  => { LD_hh(op, &mut cpu) },

            // 0x6C | 0x61 | 0x62 | 0x63 => { LD_rr(op, &mut cpu) },
            0x70 => { LD_HL_b(&mut cpu) },
            0x73 => { LD_HL_e(&mut cpu) },
            0x83 => { ADD_A_r(op, &mut cpu) },
            0x97 => { SUB_s(op, &mut cpu) },
            0xA9 => { XOR_r(op, &mut cpu) },
            0xB4 => { OR_r(op, &mut cpu) },
            0xC3 => { JMP(&mut cpu); },
            0xCD => { CALL(&mut cpu) },
            0xEA | 0xE2 | 0xDA | 0xC2 => { JP_cc_nn(op, &mut cpu) },
            0xF4 => { CALL_cc_nn(&mut cpu) },
            0xF5 | 0xC5 | 0xD5 => { PUSH_qq(op, &mut cpu) },
            0xFF | 0xC7 => { RST_p(op, &mut cpu) },
            0xF9 => { LD_SP_HL(&mut cpu) },
            _ => {  panic!("Unknown CP/M call {}!"); },
        }
         match cpu.pc() {
            0x0005 => { cpm_bdos(&mut cpu); },
            0x0000 => { break; },
            _ => { },
        }
    }
}

fn cpm_bdos(cpu: &mut Z80) {
        // get the function call id from register C
    match cpu.c {
        2 => {
            // output character in register E
            print!("{}", cpu.e as char);
        },
        9 => {
            // output a string at register DE until '$'
            let mut addr = cpu.DE();
            loop {
                let c = cpu.mem.r8(addr) as char;
                addr = (addr + 1) & 0xFFFF;
                if c != '$' {
                    print!("{}", c);
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
}

fn DEC_ss(cpu: &mut Z80) {
    let HL = cpu.HL;
    let r = HL - 1;
    cpu.set_HL(r);
    cpu.step();
}

fn CPL(cpu: &mut Z80) {
    let A = cpu.a;
    cpu.set_A(!A);
    cpu.Flags_setH(true);
    cpu.Flags_setN(true);
    cpu.step();
}

fn OR_HL(cpu: &mut Z80) {
    let A = cpu.a;
    let _HL_ = cpu.mem.r8(cpu.HL);
    let result = A | _HL_;
    cpu.set_A(result);

    cpu.Flags_setS((result as i8) < 0);
    cpu.Flags_setZ(result == 0);
    cpu.Flags_setN(false);
    cpu.Flags_setC(false);
    cpu.Flags_setH(false);
    let is_even = result & 0b000_0001 == 1;
    cpu.Flags_setPE(is_even); // FIX !!!!
    cpu.step();
}

fn ADC_r(cpu: &mut Z80) {
    let A = cpu.a;
    let n = cpu.mem.r8(cpu.pc());
    let C = cpu.Flags_getC();
    if C {
        cpu.set_A(A + n + 1);
        cpu.Flags_setS(A + n + 1 == 0);
        cpu.Flags_setZ(A + n + 1 == 1);
        cpu.Flags_setPE(A + n + 1 > 127);
    } else {
        cpu.set_A(A + n);
        cpu.Flags_setS(A + n == 0);
        cpu.Flags_setZ(A + n == 1);
        cpu.Flags_setPE(A + n > 127);
    }
    // A = cpu.a;
    cpu.Flags_setN(false);
    cpu.Flags_setC(A == 0x7F);
    //H is set if carry from bit 3; otherwise, it is reset.
    cpu.Flags_setH((A & 0b0000_1111) == 0xF);
    cpu.step();
}

fn DEC_r(op: u8, cpu: &mut Z80) {
    let sel = (op & 0b0111_0000) >> 4 as u8;
    let mut r = 0;
    match sel {
        0b000 => {
            r = cpu.b as i8;
            cpu.set_B((r -1) as u8);
        },
        0b001 => {
            r = cpu.c as i8;
            cpu.set_C((r -1) as u8);
        },
        0b010 => {
            r = cpu.d as i8;
            cpu.set_D((r -1) as u8);
        },
        0b011 => {
            r = cpu.e as i8;
            cpu.set_E((r -1) as u8);
        },
        0b100 => {
            r = cpu.get_HL_H() as i8;
            cpu.set_HL_H((r -1) as u8);
        },
        0b101 => {
            r = cpu.get_HL_L() as i8;
            cpu.set_HL_L((r -1) as u8);
        },
        0b111 => {
            r = cpu.a as i8;
            cpu.set_A((r -1) as u8);
        },
          _ => {
            panic!("Unimplemented instruction");
        }
    }

    cpu.Flags_setS(r == 0);
    cpu.Flags_setZ(r == 1);
    cpu.Flags_setN(true);
    cpu.Flags_setPE(r as u8 == 0x80);
    cpu.step();
}

fn ADD_A_r(op: u8, cpu: &mut Z80) {
    let sel = (op & 0b0000_0111) as u8;
    match sel {
        0b111 => {
            let A = cpu.a;
            cpu.set_A(A + A);
        },
        0b000 => {
            let B = cpu.b;
            let A = cpu.a;
            cpu.set_A(A + B);
        },
        0b001 => {
            let C = cpu.c;
            let A = cpu.a;
            cpu.set_A(A + C);
        },
        0b010 => {
            let D = cpu.d;
            let A = cpu.a;
            cpu.set_A(A + D);
        },
        0b011 => {
            let E = cpu.e;
            let A = cpu.a;
            cpu.set_A(A + E);
        },
        0b100 => {
            let H = cpu.get_HL_H();
            let A = cpu.a;
            cpu.set_A(A + H);
        },
        0b101 => {
            let L = cpu.get_HL_L();
            let A = cpu.a;
            cpu.set_A(A + L);
        },
         _ => {
            panic!("Unimplemented instruction");
        }
    }
    cpu.step();
}

fn PUSH_qq(op: u8, cpu: &mut Z80) {
    let sel = (op & 0b0011_0000) >> 4 as u8;
    match sel {
        0b11 => {
            let AF = cpu.AF();
            cpu.push(AF);
        },
        0b00 => {
            let BC = cpu.BC();
            cpu.push(BC);
        },
        0b01 => {
            let DE = cpu.DE();
            cpu.push(DE);
        },
        0b10 => {
            let HL = cpu.HL;
            cpu.push(HL);
        },
        _ => {
            panic!("Unimplemented instruction");
        }
    }
    cpu.step();
}

fn CALL_cc_nn(cpu: &mut Z80) {
    // F4: if Sign Positive (P), then push PC onto stack and put nn contents on PC
    if !cpu.Flags_getS() {
        let addr = cpu.mem.r16(cpu.pc());
        let pc = cpu.pc();
        cpu.push(pc);
        cpu.set_pc(addr);
    } else {
        cpu.step();
        cpu.step();
    }
}

fn SUB_s(op: u8, cpu: &mut Z80) {
    let n = cpu.mem.r8(cpu.pc()) as i8;
    let r = cpu.a as i8 - n;
    cpu.set_A(r as u8);
    cpu.Flags_setS(r < 0);
    cpu.Flags_setZ(r == 0);
    cpu.Flags_setN(true);
    cpu.Flags_setPE(r as u8 == 0x80);
    cpu.step();
}

fn CALL(cpu: &mut Z80) {
    let pc = cpu.pc();
    let addr = cpu.mem.r16(cpu.pc());
    cpu.push(pc -1);
    cpu.set_pc(addr);
}

fn ADD_HL_BC(cpu: &mut Z80) {
    let BC = cpu.BC() as i16;
    let HL = cpu.HL as i16;
    cpu.set_HL((HL + BC) as u16);
    cpu.Flags_setN(false);
    cpu.Flags_setC((HL & 0b0100_0000_0000_0000) >> 14 == 1);
    cpu.Flags_setH((HL & 0b0000_1000_0000_0000) >> 10 == 1);
    cpu.step();
}

/// cc   Condition Relevant  Flag
/// 000  Non-Zero (NZ)         Z
/// 001   Zero (Z)             Z
/// 010   No Carry (NC)        C
/// 011   Carry (C)            C
/// 100   Parity Odd (PO)     P/V
/// 101   Parity Even (PE)    P/V
/// 110   Sign Positive (P)    S
/// 111   Sign Negative (M)    S
fn JP_cc_nn(op: u8, cpu: &mut Z80) {
    let sel = (op & 0b00111000) >> 3 as u8;
    let addr = cpu.mem.r16(cpu.pc());

    match sel {
        0b000 => {
            if !cpu.Flags_getZ() {
                cpu.set_pc(addr)
            } else {
                cpu.step();
                cpu.step();
            }
        }
        0b101 => {
            if cpu.Flags_getPE() {
                cpu.set_pc(addr);
            } else {
                cpu.step();
                cpu.step();
            }
        },
        0b011 => {
            if cpu.Flags_getC() {
                cpu.set_pc(addr);
            } else {
                cpu.step();
                cpu.step();
            }
        },
        0b100 => {
            if !cpu.Flags_getPE() {
                cpu.set_pc(addr);
            } else {
                cpu.step();
                cpu.step();
            }
        },
        _ => {
            panic!("Unimplemented instruction");
        }
    }
}

fn LD_SP_HL(cpu: &mut Z80) {
    cpu.set_sp(cpu.HL);
    cpu.step();
}

/// If address 4545h contains 37h and address 4546h contains A1h,
/// then upon the execution of an LD HL, (4545h) instruction,
/// the HL register pair contains A137h.
fn LD_HL_nn(cpu: &mut Z80) {
    let addr = cpu.pc();
    let n = cpu.mem.r8(addr);
    let nn = cpu.mem.r8(addr + 1);
    cpu.set_HL_L(n);
    cpu.set_HL_H(nn);
    cpu.step();
}

fn LD_dd_nn(op: u8, cpu: &mut Z80) {
    let addr = cpu.pc();
    let nn = cpu.mem.r16(addr);

    match op {
        0x11 => {
            cpu.set_DE(nn);
        },
        0x21 => {
            cpu.set_HL(nn);
        }
        0x22 => {
            cpu.set_HL(nn);
        },
        _ => {  panic!("Unknown CP/M call {}!"); }
    }
    cpu.step();
}

fn XOR_r(op: u8, cpu: &mut Z80) {
    let C = cpu.c;
    let A = cpu.a;
    let result = (C ^ A) as i8;
    cpu.set_A(result as u8);
    // Check flags
    cpu.Flags_setZ(result == 0);
    cpu.Flags_setS(result < 0);
    cpu.Flags_setN(false);
    cpu.Flags_setC(false);
    cpu.Flags_setH(false);
    cpu.step();
}

fn OR_r(op: u8, cpu: &mut Z80) {
    let H = (cpu.HL & 0x0F) as u8;
    let A = cpu.a;
    let result = (H | A) as i8;
    cpu.set_A(result as u8);
    // Check flags
    cpu.Flags_setZ(result == 0);
    cpu.Flags_setS(result < 0);
    cpu.Flags_setN(false);
    cpu.Flags_setC(false);
    cpu.Flags_setH(false);
    cpu.step();
}

fn RST_p(op: u8, cpu: &mut Z80) {
    let pc = cpu.pc();
    cpu.push(pc);
    match op {
        0xFF => {
            cpu.set_pc(0x0038);
        },
        0xC7 => {
            cpu.set_pc(0x0000);
        },
        _ => {
            println!("Unimplemented instruction");
        }
    }
}

/*
The 8-bit integer n is loaded to any register r, in which r identifies registers A, B, C, D, E, H, or L,
assembled as follows in the object code:
*/

fn LD_r_n(op: u8, cpu: &mut Z80) {
    let sel = (op & 0b00111110) >> 3 as u8;
    let addr = cpu.pc();
    let n = cpu.mem.r8(addr);
    match sel {
        0b111 => {
            cpu.set_A(n);
        },
        0b000 => {
            cpu.set_B(n);
        },
        0b001 => {
            cpu.set_C(n);
         },
        0b010 => {
            cpu.set_D(n);
         },
        0b011 => {
            cpu.set_E(n);
        },
        0b100 => {
            cpu.set_HL_H(n);
        },
        0b101 => {
            cpu.set_HL_L(n);
        },
        _ => {  panic!("Unknown CP/M call {}!"); }
    }
}

/*
LD   (HL),E
The contents of register r are loaded to the memory location specified by the contents of the HL register pair.
*/
fn LD_HL_e(cpu: &mut Z80) {
    let addr = cpu.HL;
    cpu.mem.w8(addr, cpu.e);
}

/*
LD   (HL),B
The contents of register r are loaded to the memory location specified by the contents of the HL register pair.
*/
fn LD_HL_b(cpu: &mut Z80) {
    let addr = cpu.HL;
    cpu.mem.w8(addr, cpu.b);
}

fn LD_lb(cpu: &mut Z80) {
    cpu.set_HL_L(cpu.b);
}

fn LD_rr(op: u8, cpu: &mut Z80) {
    match op {
        0x6C => {
            /* LD  L,H */
            let H = (cpu.HL & 0xF0) as u8;
            cpu.set_HL_L(H);
        },
        0x61 => {
            /* LD   H,C */
            let c = cpu.c;
            cpu.set_HL_H(c);
        },
        0x62 => {
            /* LD   H,D */
            cpu.set_HL_H(cpu.d);
        },
        0x63 => {
            /* LD   H,E */
            cpu.set_HL_H(cpu.e);
        }
         _ => {  panic!("Unknown CP/M call {}!"); }
    }
}

fn INC_r(op: u8, cpu: &mut Z80) {
    let mut incr = 0;
    let mut r = 0;
    match op {
        0x24 => {
            /* INC  H */
            r = cpu.get_HL_H();
            incr = r + 1;
            cpu.set_HL_H(incr);
        }
        0x2C => {
            r = (cpu.HL & 0x0F) as u8;
            incr = r + 1;
            cpu.set_HL_L(incr);
        },
        0x3C => {
            r = cpu.a;
            incr = r + 1;
            cpu.set_A(incr);

        },
        _ => {  panic!("Unknown CP/M call {}!"); }

    }

    // Update flags
    cpu.Flags_setZ(incr == 0);
    cpu.Flags_setS((incr as i8) < 0);
    cpu.Flags_setN(false);
    cpu.Flags_setC(false);
    cpu.Flags_setH(false);
    cpu.Flags_setPE(r == 0x7F);
}

fn LD_BC_nn(cpu: &mut Z80) {

    let addr = cpu.pc();
    let lowByte = cpu.mem.r8(addr);
    let highByte = cpu.mem.r8(addr + 1);
    cpu.set_C(lowByte);
    cpu.set_B(highByte);
    cpu.step();
    cpu.step();
}

fn jr_nz(cpu: &mut Z80) {
    if !cpu.Flags_getZ() {
        let addr = cpu.pc();
        let offset = cpu.mem.r8(addr);
        let displacement = addr.wrapping_add(offset as u16);
        cpu.set_pc(displacement);
    } else {
        cpu.step();
    }

}

fn LD_hh(op: u8, cpu: &mut Z80) {
    let r = (op & 0b0011_1000) >> 3 as u8;
    let r1 = (op & 0b0000_0111) >> 3 as u8;
    match (r, r1) {
        (0b111, 0b000) => {
            cpu.set_A(cpu.b);
        },
        (0b111, 0b001) => {
            cpu.set_A(cpu.c);
        },
        (0b111, 0b010) => {
            cpu.set_A(cpu.d);
        },
        (0b111, 0b011) => {
            cpu.set_A(cpu.e);
        },
        (0b111, 0b100) => {
            let H = cpu.get_HL_H();
            cpu.set_A(H);
        },
        (0b111, 0b101) => {
            let L = cpu.get_HL_L();
            cpu.set_A(L);
        },
        (0b000, 0b111) => {
            cpu.set_B(cpu.a);
        },
        (0b000, 0b001) => {
            cpu.set_B(cpu.c);
        },
        (0b000, 0b010) => {
            cpu.set_B(cpu.d);
        },
        (0b000, 0b011) => {
            cpu.set_B(cpu.e);
        },
        (0b000, 0b100) => {
            let H = cpu.get_HL_H();
            cpu.set_B(H);
        },
        (0b000, 0b101) => {
            let L = cpu.get_HL_L();
            cpu.set_B(L);
        },
        (0b001, 0b111) => {
            cpu.set_C(cpu.a);
        },
        (0b001, 0b000) => {
            cpu.set_C(cpu.b);
        },
        (0b001, 0b010) => {
            cpu.set_C(cpu.d);
        },
        (0b001, 0b011) => {
            cpu.set_C(cpu.e);
        },
        (0b001, 0b100) => {
            let H = cpu.get_HL_H();
            cpu.set_C(H);
        },
        (0b001, 0b101) => {
            let L = cpu.get_HL_L();
            cpu.set_C(L);
        },
        (0b010, 0b111) => {
            cpu.set_D(cpu.a);
        },
        (0b010, 0b000) => {
            cpu.set_D(cpu.b);
        },
        (0b010, 0b001) => {
            cpu.set_D(cpu.c);
        },
        (0b010, 0b011) => {
            cpu.set_D(cpu.e)
        },
        (0b010, 0b100) => {
            let H = cpu.get_HL_H();
            cpu.set_D(H);
        },
        (0b010, 0b101) => {
            let L = cpu.get_HL_L();
            cpu.set_D(L);
        },
        (0b011, 0b111) => {
            cpu.set_E(cpu.a);
        },
        (0b011, 0b000) => {
            cpu.set_E(cpu.b);
        },
        (0b011, 0b001) => {
            cpu.set_E(cpu.c);
        },
        (0b011, 0b010) => {
            cpu.set_E(cpu.d);
        },
        (0b011, 0b011) => {
            cpu.set_E(cpu.e)
        },
        (0b011, 0b100) => {
            let H = cpu.get_HL_H();
            cpu.set_E(H);
        },
        (0b011, 0b101) => {
            let L = cpu.get_HL_L();
            cpu.set_E(L);
        },
        (0b100, 0b111) => {
            cpu.set_HL_H(cpu.a);
        },
        (0b100, 0b000) => {
            cpu.set_HL_H(cpu.b);
        },
        (0b100, 0b001) => {
            cpu.set_HL_H(cpu.c);
        },
        (0b100, 0b010) => {
            cpu.set_HL_H(cpu.d);
        },
        (0b100, 0b011) => {
            cpu.set_HL_H(cpu.e);
        },
        (0b100, 0b100) => {
            let H = cpu.get_HL_H();
            cpu.set_HL_H(H);
        },
        (0b100, 0b101) => {
            let L = cpu.get_HL_L();
            cpu.set_HL_L(L);
        },
        (0b101, 0b111) => {
            cpu.set_HL_L(cpu.a);
        },
        (0b101, 0b000) => {
            cpu.set_HL_L(cpu.b);
        },
        (0b101,0b001) => {
            cpu.set_HL_L(cpu.c);
        },
        (0b101,0b010) => {
            cpu.set_HL_L(cpu.d);
        },
        (0b101, 0b011) => {
            cpu.set_HL_L(cpu.e);
        },
        (0b101,0b100) => {
            let H = cpu.get_HL_H();
            cpu.set_HL_L(H);
        },
        (0b101, 0b101) => {
            let H = cpu.get_HL_H();
            cpu.set_HL_L(H);
        },
        _ => {  panic!("Unknown CP/M call {}!"); }
    }
    cpu.step();
}

// fn LD_hl(cpu: &mut Z80) {
//     let L = (cpu.HL & 0xF) as u8;
//     cpu.set_HL_H(L);
// }

fn nop(cpu: &mut Z80) {
    cpu.step();
}

fn JMP(cpu: &mut Z80) {
    //This will make the PC point to the next two bytes of the instruction
    let addr = cpu.pc();
    let data = cpu.mem.r16(addr);
    cpu.set_pc(data);
}