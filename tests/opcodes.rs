#[allow(unused_imports)]
#[cfg(test)]
mod test_opcodes {
use ::Z80::memory::PlainMemory;
use ::Z80::memory::Memory;
use ::Z80::registers::{ Register16Bit, Flags };
use Z80::z80::Z80;

  #[test]
  fn test_ld_ihl_n() {
    let mut cpu = Z80::new();
    let mut mem = PlainMemory::new_64k();
      cpu.r.sp = 0xFFFF;
      cpu.r.pc = 0x0000;
      let prog = [
          0x21, 0x00, 0x20,   // LD HL,0x2000
          0x36, 0x33,         // LD (HL),0x33
          0x21, 0x00, 0x10,   // LD HL,0x1000
          0x36, 0x65,         // LD (HL),0x65
      ];
      mem.write(0x0000, &prog);
      cpu.exec(&mut mem);
      assert!(0x2000 == cpu.r.get_u16(Register16Bit::HL));
      cpu.exec(&mut  mem);
      assert!(0x33 == mem.r8(0x2000));
      cpu.exec(&mut  mem);
      assert!(0x1000 == cpu.r.get_u16(Register16Bit::HL));
      cpu.exec(&mut  mem);
      assert!(0x65 == mem.r8(0x1000));
  }

  #[test]
  fn test_ld_ihl() {
    let mut cpu = Z80::new();
    let mut mem = PlainMemory::new_64k();
      let prog = [
          0x77,       // LD (HL),A
          0x46,       // LD B,(HL)
          0x4E,       // LD C,(HL)
          0x56,       // LD D,(HL)
          0x5E,       // LD E,(HL)
          0x66,       // LD H,(HL)
      ];
      mem.write(0x0100, &prog);

      cpu.r.a = 0x33;
      cpu.r.set_u16(Register16Bit::HL, 0x1000);
      cpu.r.pc = 0x0100;
      cpu.exec(&mut  mem);
      assert_eq!(0x33, mem.r8(0x1000));
      cpu.exec(&mut  mem);
      assert_eq!(0x33, cpu.r.b);
      cpu.exec(&mut  mem);
      assert_eq!(0x33, cpu.r.c);
      cpu.exec(&mut  mem);
      assert_eq!(0x33, cpu.r.d);
      cpu.exec(&mut  mem);
      assert_eq!(0x33, cpu.r.e);
      cpu.exec(&mut  mem);
      assert_eq!(0x33, cpu.r.h);
  }


  #[test]
  fn test_ld_ixiy_n() {
    let mut cpu = Z80::new();
    let mut mem = PlainMemory::new_64k();
      let prog = [
          0xDD, 0x21, 0x00, 0x20,     // LD IX,0x2000
          0xDD, 0x36, 0x02, 0x33,     // LD (IX+2),0x33
          0xDD, 0x36, 0xFE, 0x11,     // LD (IX-2),0x11
          0xFD, 0x21, 0x00, 0x10,     // LD IY,0x1000
          0xFD, 0x36, 0x01, 0x22,     // LD (IY+1),0x22
          0xFD, 0x36, 0xFF, 0x44,     // LD (IY-1),0x44
      ];
      mem.write(0x0000, &prog);
      cpu.exec(&mut  mem);
      assert_eq!(0x2000, cpu.r.get_u16(Register16Bit::IX)); 
      cpu.exec(&mut  mem);   
      assert_eq!(0x33, mem.r8(0x2002));
      cpu.exec(&mut  mem);
      assert_eq!(0x11, mem.r8(0x1FFE));
      cpu.exec(&mut  mem);
      assert_eq!(0x1000, cpu.r.get_u16(Register16Bit::IY));    
      cpu.exec(&mut  mem);
      assert_eq!(0x22, mem.r8(0x1001));
      cpu.exec(&mut  mem);
      assert_eq!(0x44, mem.r8(0x0FFF));
  }

  #[test]
  fn ld_inn_hlddixiy() {
    let mut cpu = Z80::new();
    let mut mem = PlainMemory::new_64k();
      let prog = [
          0x21, 0x01, 0x02,           // LD HL,0x0201
          0x22, 0x00, 0x10,           // LD (0x1000),HL
          0x01, 0x34, 0x12,           // LD BC,0x1234
          0xED, 0x43, 0x02, 0x10,     // LD (0x1002),BC
          0x11, 0x78, 0x56,           // LD DE,0x5678
          0xED, 0x53, 0x04, 0x10,     // LD (0x1004),DE
          0x21, 0xBC, 0x9A,           // LD HL,0x9ABC
          0xED, 0x63, 0x06, 0x10,     // LD (0x1006),HL undocumented 'long' version
          0x31, 0x68, 0x13,           // LD SP,0x1368
          0xED, 0x73, 0x08, 0x10,     // LD (0x1008),SP
          0xDD, 0x21, 0x21, 0x43,     // LD IX,0x4321
          0xDD, 0x22, 0x0A, 0x10,     // LD (0x100A),IX
          0xFD, 0x21, 0x65, 0x87,     // LD IY,0x8765
          0xFD, 0x22, 0x0C, 0x10,     // LD (0x100C),IY
      ];
      mem.write(0x0000, &prog);
      cpu.exec(&mut  mem);
      assert_eq!(0x0201, cpu.r.get_u16(Register16Bit::HL));
      cpu.exec(&mut  mem);
      assert_eq!(0x0201, mem.r16(0x1000));
      cpu.exec(&mut  mem);
      assert_eq!(0x1234, cpu.r.get_u16(Register16Bit::BC));       
      cpu.exec(&mut  mem);
      assert_eq!(0x1234, mem.r16(0x1002));
      cpu.exec(&mut  mem);
      assert_eq!(0x5678, cpu.r.get_u16(Register16Bit::DE));
      cpu.exec(&mut  mem);
      assert_eq!(0x5678, mem.r16(0x1004));
      cpu.exec(&mut  mem);
      assert_eq!(0x9ABC, cpu.r.get_u16(Register16Bit::HL));
      cpu.exec(&mut  mem);
      assert_eq!(0x9ABC, mem.r16(0x1006));
      cpu.exec(&mut  mem);
      assert_eq!(0x1368, cpu.r.get_u16(Register16Bit::SP));
      cpu.exec(&mut  mem);
      assert_eq!(0x1368, mem.r16(0x1008));
      cpu.exec(&mut  mem);
      assert_eq!(0x4321, cpu.r.get_u16(Register16Bit::IX));       
      cpu.exec(&mut  mem);
      assert_eq!(0x4321, mem.r16(0x100A));
      cpu.exec(&mut  mem);
      assert_eq!(0x8765, cpu.r.get_u16(Register16Bit::IY));       
      cpu.exec(&mut  mem);
      assert_eq!(0x8765, mem.r16(0x100C));
  }


  #[test]
  fn test_ld_r_ixiy() {
    let mut cpu = Z80::new();
    let mut mem = PlainMemory::new_64k();
      let data = [
          1, 2, 3, 4, 5, 6, 7, 8
      ];
      mem.write(0x1000, &data);

      let  prog = [
          0xDD, 0x21, 0x03, 0x10,     // LD IX,0x1003
          0xDD, 0x7E, 0x00,           // LD A,(IX+0)
          0xDD, 0x46, 0x01,           // LD B,(IX+1)
          0xDD, 0x4E, 0x02,           // LD C,(IX+2)
          0xDD, 0x56, 0xFF,           // LD D,(IX-1)
          0xDD, 0x5E, 0xFE,           // LD E,(IX-2)
          0xDD, 0x66, 0x03,           // LD H,(IX+3)
          0xDD, 0x6E, 0xFD,           // LD L,(IX-3)

          0xFD, 0x21, 0x04, 0x10,     // LD IY,0x1004
          0xFD, 0x7E, 0x00,           // LD A,(IY+0)
          0xFD, 0x46, 0x01,           // LD B,(IY+1)
          0xFD, 0x4E, 0x02,           // LD C,(IY+2)
          0xFD, 0x56, 0xFF,           // LD D,(IY-1)
          0xFD, 0x5E, 0xFE,           // LD E,(IY-2)
          0xFD, 0x66, 0x03,           // LD H,(IY+3)
          0xFD, 0x6E, 0xFD,           // LD L,(IY-3)
      ];
      mem.write(0x0000, &prog);
      cpu.exec(&mut  mem);
      assert_eq!(0x1003, cpu.r.get_u16(Register16Bit::IX));
      cpu.exec(&mut mem);
      assert_eq!(4, cpu.r.a);      
      cpu.exec(&mut  mem);
      assert_eq!(5, cpu.r.b);      
      cpu.exec(&mut mem);
      assert_eq!(6, cpu.r.c);
      cpu.exec(&mut mem);      
      assert_eq!(3, cpu.r.d);  cpu.exec(&mut  mem);
      assert_eq!(2, cpu.r.e);  cpu.exec(&mut  mem);
      assert_eq!(7, cpu.r.h);  cpu.exec(&mut  mem);    
      assert_eq!(1, cpu.r.l);  cpu.exec(&mut  mem);
      assert_eq!(0x1004, cpu.r.get_u16(Register16Bit::IY)); cpu.exec(&mut  mem);
      assert_eq!(5, cpu.r.a);  cpu.exec(&mut  mem);
      assert_eq!(6, cpu.r.b);  cpu.exec(&mut  mem);
      assert_eq!(7, cpu.r.c);  cpu.exec(&mut  mem);
      assert_eq!(4, cpu.r.d);  cpu.exec(&mut  mem);
      assert_eq!(3, cpu.r.e);  cpu.exec(&mut  mem);
      assert_eq!(8, cpu.r.h);  cpu.exec(&mut  mem);
      assert_eq!(2, cpu.r.l); 
  }

  #[test]
  fn test_rrc_rlc_rr_rl_ihlixiy() {
    let mut cpu = Z80::new();
    let mut mem = PlainMemory::new_64k();
      let data = [ 0x01, 0xFF, 0x11 ];
      mem.write(0x1000, &data);
      let prog = [
          0x21, 0x00, 0x10,           // LD HL,0x1000
          0xDD, 0x21, 0x00, 0x10,     // LD IX,0x1001
          0xFD, 0x21, 0x03, 0x10,     // LD IY,0x1003
          0xCB, 0x0E,                 // RRC (HL)
          0x7E,                       // LD A,(HL)
          0xCB, 0x06,                 // RLC (HL)
          0x7E,                       // LD A,(HL)
          0xDD, 0xCB, 0x01, 0x0E,     // RRC (IX+1)
          0xDD, 0x7E, 0x01,           // LD A,(IX+1)
          0xDD, 0xCB, 0x01, 0x06,     // RLC (IX+1)
          0xDD, 0x7E, 0x01,           // LD A,(IX+1)
          0xFD, 0xCB, 0xFF, 0x0E,     // RRC (IY-1)
          0xFD, 0x7E, 0xFF,           // LD A,(IY-1)
          0xFD, 0xCB, 0xFF, 0x06,     // RLC (IY-1)
          0xFD, 0x7E, 0xFF,           // LD A,(IY-1)
          0xCB, 0x1E,                 // RR (HL)
          0x7E,                       // LD A,(HL)
          0xCB, 0x16,                 // RL (HL)
          0x7E,                       // LD A,(HL)
          0xDD, 0xCB, 0x01, 0x1E,     // RR (IX+1)
          0xDD, 0x7E, 0x01,           // LD A,(IX+1)
          0xDD, 0xCB, 0x01, 0x16,     // RL (IX+1)
          0xDD, 0x7E, 0x01,           // LD A,(IX+1)
          0xFD, 0xCB, 0xFF, 0x16,     // RL (IY-1)
          0xFD, 0x7E, 0xFF,           // LD A,(IY-1)
          0xFD, 0xCB, 0xFF, 0x1E,     // RR (IY-1)
          0xFD, 0x7E, 0xFF,           // LD A,(IY-1)
      ];
      mem.write(0x0000, &prog);

      // skip loads
      for _ in 0..3 {
        cpu.exec(&mut mem);  
      }
      cpu.exec(&mut mem);
      assert_eq!(0x80, mem.r8(0x1000)); assert!(cpu.r.f.contains(Flags::SIGN | Flags::CARRY));
      cpu.exec(&mut mem);
      assert_eq!(0x80, cpu.r.a);
      cpu.exec(&mut mem);
      assert_eq!(0x01, mem.r8(0x1000)); assert!(cpu.r.f.contains(Flags::CARRY));
      cpu.exec(&mut mem);
      assert_eq!(0x01, cpu.r.a);
      cpu.exec(&mut mem);
      assert_eq!(0xFF, mem.r8(0x1001)); assert!(cpu.r.f.contains(Flags::SIGN | Flags::PARITY | Flags::CARRY));
      cpu.exec(&mut mem);
      assert_eq!(0xFF, cpu.r.a);
      cpu.exec(&mut mem);
      assert_eq!(0xFF, mem.r8(0x1001)); assert!(cpu.r.f.contains(Flags::SIGN | Flags::PARITY | Flags::CARRY));
      cpu.exec(&mut mem);
      assert_eq!(0xFF, cpu.r.a);
      cpu.exec(&mut mem);
      assert_eq!(0x88, mem.r8(0x1002)); assert!(cpu.r.f.contains(Flags::SIGN | Flags::PARITY | Flags::CARRY));
      cpu.exec(&mut mem);
      assert_eq!(0x88, cpu.r.a);
      cpu.exec(&mut mem);
      assert_eq!(0x11, mem.r8(0x1002)); assert!(cpu.r.f.contains(Flags::PARITY | Flags::CARRY)); 
      cpu.exec(&mut mem);
      assert_eq!(0x11, cpu.r.a);
      cpu.exec(&mut mem);
      assert_eq!(0x80, mem.r8(0x1000)); assert!(cpu.r.f.contains(Flags::SIGN | Flags::CARRY));
      cpu.exec(&mut mem);
      assert_eq!(0x80, cpu.r.a);
      cpu.exec(&mut mem);
      assert_eq!(0x01, mem.r8(0x1000)); assert!(cpu.r.f.contains(Flags::CARRY));
      cpu.exec(&mut mem);
      assert_eq!(0x01, cpu.r.a);
      cpu.exec(&mut mem);
      assert_eq!(0xFF, mem.r8(0x1001)); assert!(cpu.r.f.contains(Flags::SIGN | Flags::PARITY | Flags::CARRY));
      cpu.exec(&mut mem);
      assert_eq!(0xFF, cpu.r.a);
      cpu.exec(&mut mem);
      assert_eq!(0xFF, mem.r8(0x1001)); assert!(cpu.r.f.contains(Flags::SIGN | Flags::PARITY | Flags::CARRY));
      cpu.exec(&mut mem);
      assert_eq!(0xFF, cpu.r.a);
      cpu.exec(&mut mem);
      assert_eq!(0x23, mem.r8(0x1002)); assert_eq!(cpu.r.f, Flags::empty());
      cpu.exec(&mut mem);
      assert_eq!(0x23, cpu.r.a);
      cpu.exec(&mut mem);
      assert_eq!(0x11, mem.r8(0x1002)); assert!(cpu.r.f.contains(Flags::PARITY | Flags::CARRY));
      cpu.exec(&mut mem);
      assert_eq!(0x11, cpu.r.a);
  }

  #[test]
  fn test_ld_ixiy_r() {
    let mut cpu = Z80::new();
    let mut mem = PlainMemory::new_64k();
      let prog = [
          0xDD, 0x21, 0x03, 0x10,     // LD IX,0x1003
          0x3E, 0x12,                 // LD A,0x12
          0xDD, 0x77, 0x00,           // LD (IX+0),A
          0x06, 0x13,                 // LD B,0x13
          0xDD, 0x70, 0x01,           // LD (IX+1),B
          0x0E, 0x14,                 // LD C,0x14
          0xDD, 0x71, 0x02,           // LD (IX+2),C
          0x16, 0x15,                 // LD D,0x15
          0xDD, 0x72, 0xFF,           // LD (IX-1),D
          0x1E, 0x16,                 // LD E,0x16
          0xDD, 0x73, 0xFE,           // LD (IX-2),E
          0x26, 0x17,                 // LD H,0x17
          0xDD, 0x74, 0x03,           // LD (IX+3),H
          0x2E, 0x18,                 // LD L,0x18
          0xDD, 0x75, 0xFD,           // LD (IX-3),L
          0xFD, 0x21, 0x03, 0x10,     // LD IY,0x1003
          0x3E, 0x12,                 // LD A,0x12
          0xFD, 0x77, 0x00,           // LD (IY+0),A
          0x06, 0x13,                 // LD B,0x13
          0xFD, 0x70, 0x01,           // LD (IY+1),B
          0x0E, 0x14,                 // LD C,0x14
          0xFD, 0x71, 0x02,           // LD (IY+2),C
          0x16, 0x15,                 // LD D,0x15
          0xFD, 0x72, 0xFF,           // LD (IY-1),D
          0x1E, 0x16,                 // LD E,0x16
          0xFD, 0x73, 0xFE,           // LD (IY-2),E
          0x26, 0x17,                 // LD H,0x17
          0xFD, 0x74, 0x03,           // LD (IY+3),H
          0x2E, 0x18,                 // LD L,0x18
          0xFD, 0x75, 0xFD,           // LD (IY-3),L
      ];
      mem.write(0x0000, &prog);
      cpu.exec(&mut  mem);
      assert_eq!(0x1003, cpu.r.get_u16(Register16Bit::IX)); cpu.exec(&mut  mem);
      assert_eq!(0x12, cpu.r.a);         cpu.exec(&mut  mem);
      assert_eq!(0x12, mem.r8(0x1003));  cpu.exec(&mut  mem);
      assert_eq!(0x13, cpu.r.b);         cpu.exec(&mut  mem);
      assert_eq!(0x13, mem.r8(0x1004));  cpu.exec(&mut  mem);
      assert_eq!(0x14, cpu.r.c);         cpu.exec(&mut  mem);
      assert_eq!(0x14, mem.r8(0x1005));  cpu.exec(&mut  mem);
      assert_eq!(0x15, cpu.r.d);         cpu.exec(&mut  mem);
      assert_eq!(0x15, mem.r8(0x1002));  cpu.exec(&mut  mem);
      assert_eq!(0x16, cpu.r.e);         cpu.exec(&mut  mem);
      assert_eq!(0x16, mem.r8(0x1001));  cpu.exec(&mut  mem);
      assert_eq!(0x17, cpu.r.h);         cpu.exec(&mut  mem);
      assert_eq!(0x17, mem.r8(0x1006));  cpu.exec(&mut  mem);
      assert_eq!(0x18, cpu.r.l);         cpu.exec(&mut  mem);
      assert_eq!(0x18, mem.r8(0x1000));  cpu.exec(&mut  mem);
      assert_eq!(0x1003, cpu.r.get_u16(Register16Bit::IY)); cpu.exec(&mut  mem);
      assert_eq!(0x12, cpu.r.a);         cpu.exec(&mut  mem);
      assert_eq!(0x12, mem.r8(0x1003));  cpu.exec(&mut  mem);
      assert_eq!(0x13, cpu.r.b);        cpu.exec(&mut  mem);
      assert_eq!(0x13, mem.r8(0x1004)); cpu.exec(&mut  mem);
      assert_eq!(0x14, cpu.r.c);        cpu.exec(&mut  mem);
      assert_eq!(0x14, mem.r8(0x1005)); cpu.exec(&mut  mem);
      assert_eq!(0x15, cpu.r.d);        cpu.exec(&mut  mem);
      assert_eq!(0x15, mem.r8(0x1002)); cpu.exec(&mut  mem);
      assert_eq!(0x16, cpu.r.e);        cpu.exec(&mut  mem);
      assert_eq!(0x16, mem.r8(0x1001)); cpu.exec(&mut  mem);
      assert_eq!(0x17, cpu.r.h);        cpu.exec(&mut  mem);
      assert_eq!(0x17, mem.r8(0x1006)); cpu.exec(&mut  mem);
      assert_eq!(0x18, cpu.r.l);        cpu.exec(&mut  mem);
      assert_eq!(0x18, mem.r8(0x1000)); 
  }

  #[test]
  fn test_ld_sp_hlixiy() {
    let mut cpu = Z80::new();
    let mut mem = PlainMemory::new_64k();
      let prog = [
          0x21, 0x34, 0x12,           // LD HL,0x1234
          0xDD, 0x21, 0x78, 0x56,     // LD IX,0x5678
          0xFD, 0x21, 0xBC, 0x9A,     // LD IY,0x9ABC
          0xF9,                       // LD SP,HL
          0xDD, 0xF9,                 // LD SP,IX
          0xFD, 0xF9,                 // LD SP,IY
      ];
      mem.write(0x0000, &prog);
      cpu.exec(&mut mem);
      assert_eq!(0x1234, cpu.r.get_u16(Register16Bit::HL));
      cpu.exec(&mut mem);
      assert_eq!(0x5678, cpu.r.get_u16(Register16Bit::IX));
      cpu.exec(&mut mem);
      assert_eq!(0x9ABC, cpu.r.get_u16(Register16Bit::IY));
      cpu.exec(&mut mem);
      assert_eq!(0x1234, cpu.r.get_u16(Register16Bit::SP));
      cpu.exec(&mut mem);
      assert_eq!(0x5678, cpu.r.get_u16(Register16Bit::SP));
      cpu.exec(&mut mem);
      assert_eq!(0x9ABC, cpu.r.get_u16(Register16Bit::SP));
  }

  #[test]
    fn ld_hlddixiy_inn() {
        let mut cpu = Z80::new();
        let mut mem = PlainMemory::new_64k();
        let data = [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08
        ];
        mem.write(0x1000, &data);
        let prog = [
            0x2A, 0x00, 0x10,           // LD HL,(0x1000)
            0xED, 0x4B, 0x01, 0x10,     // LD BC,(0x1001)
            0xED, 0x5B, 0x02, 0x10,     // LD DE,(0x1002)
            0xED, 0x6B, 0x03, 0x10,     // LD HL,(0x1003) undocumented 'long' version
            0xED, 0x7B, 0x04, 0x10,     // LD SP,(0x1004)
            0xDD, 0x2A, 0x05, 0x10,     // LD IX,(0x1004)
            0xFD, 0x2A, 0x06, 0x10,     // LD IY,(0x1005)
        ];
        mem.write(0x0000, &prog);
        cpu.exec(&mut mem);
        assert_eq!(0x0201, cpu.r.get_u16(Register16Bit::HL));
        cpu.exec(&mut mem);
        assert_eq!(0x0302, cpu.r.get_u16(Register16Bit::BC));
        cpu.exec(&mut mem);
        assert_eq!(0x0403, cpu.r.get_u16(Register16Bit::DE));
        cpu.exec(&mut mem);
        assert_eq!(0x0504, cpu.r.get_u16(Register16Bit::HL));
        cpu.exec(&mut mem);
        assert_eq!(0x0605, cpu.r.get_u16(Register16Bit::SP));
        cpu.exec(&mut mem);
        assert_eq!(0x0706, cpu.r.get_u16(Register16Bit::IX));
        cpu.exec(&mut mem);
        assert_eq!(0x0807, cpu.r.get_u16(Register16Bit::IY));
    }

    #[test]
    fn test_add_r() {
        let mut cpu = Z80::new();
        let mut mem = PlainMemory::new_64k();
        let prog = [
            0x3E, 0x0F,     // LD A,0x0F
            0x87,           // ADD A,A
            0x06, 0xE0,     // LD B,0xE0
            0x80,           // ADD A,B
            0x3E, 0x81,     // LD A,0x81
            0x0E, 0x80,     // LD C,0x80
            0x81,           // ADD A,C
            0x16, 0xFF,     // LD D,0xFF
            0x82,           // ADD A,D
            0x1E, 0x40,     // LD E,0x40
            0x83,           // ADD A,E
            0x26, 0x80,     // LD H,0x80
            0x84,           // ADD A,H
            0x2E, 0x33,     // LD L,0x33
            0x85,           // ADD A,L
            0xC6, 0x44,     // ADD A,0x44
        ];
        mem.write(0x0000, &prog);
        cpu.exec(&mut  mem);
        assert_eq!(0x0F, cpu.r.a); assert_eq!(cpu.r.f, Flags::empty());
        cpu.exec(&mut  mem);
        assert_eq!(0x1E, cpu.r.a); assert!(cpu.r.f.contains(Flags::HALFCARRY));
        cpu.exec(&mut  mem);
        assert_eq!(0xE0, cpu.r.b);
        cpu.exec(&mut  mem);
        assert_eq!(0xFE, cpu.r.a); assert!(cpu.r.f.contains(Flags::SIGN));
        cpu.exec(&mut  mem);
        assert_eq!(0x81, cpu.r.a);
        cpu.exec(&mut  mem);
        assert_eq!(0x80, cpu.r.c);
        cpu.exec(&mut  mem);
        assert_eq!(0x01, cpu.r.a); assert!(cpu.r.f.contains(Flags::CARRY)| cpu.r.f.contains(Flags::PARITY));
        cpu.exec(&mut  mem);
        assert_eq!(0xFF, cpu.r.d);
        cpu.exec(&mut  mem);
        assert_eq!(0x00, cpu.r.a); assert!(cpu.r.f.contains(Flags::ZERO) | cpu.r.f.contains(Flags::HALFCARRY)| cpu.r.f.contains(Flags::CARRY));
        cpu.exec(&mut  mem);
        assert_eq!(0x40, cpu.r.e);
        cpu.exec(&mut  mem);
        assert_eq!(0x40, cpu.r.a); assert_eq!(cpu.r.f, Flags::empty());
        cpu.exec(&mut  mem);
        assert_eq!(0x80, cpu.r.h);
        cpu.exec(&mut  mem);
        assert_eq!(0xC0, cpu.r.a); assert!(cpu.r.f.contains(Flags::SIGN));
        cpu.exec(&mut  mem);
        assert_eq!(0x33, cpu.r.l);
        cpu.exec(&mut  mem);
        assert_eq!(0xF3, cpu.r.a); assert!(cpu.r.f.contains(Flags::SIGN));
        cpu.exec(&mut  mem);
        assert_eq!(0x37, cpu.r.a); assert!(cpu.r.f.contains(Flags::CARRY));
    }

    #[test]
    fn test_add_r_2() {
        let mut cpu = Z80::new();
    let mut mem = PlainMemory::new_64k();
        let prog = [
            0x3E, 0x0F,     // LD A,0x0F
            0x87,           // ADD A,A
            0x06, 0xE0,     // LD B,0xE0
            0x80,           // ADD A,B
            0x3E, 0x81,     // LD A,0x81
            0x0E, 0x80,     // LD C,0x80
            0x81,           // ADD A,C
            0x16, 0xFF,     // LD D,0xFF
            0x82,           // ADD A,D
            0x1E, 0x40,     // LD E,0x40
            0x83,           // ADD A,E
            0x26, 0x80,     // LD H,0x80
            0x84,           // ADD A,H
            0x2E, 0x33,     // LD L,0x33
            0x85,           // ADD A,L
            0xC6, 0x44,     // ADD A,0x44
        ];
        mem.write(0x0000, &prog);
        cpu.exec(&mut  mem);
        assert_eq!(0x0F, cpu.r.a); assert_eq!(cpu.r.f, Flags::empty());
        cpu.exec(&mut  mem);
        assert_eq!(0x1E, cpu.r.a); assert!(cpu.r.f.contains(Flags::HALFCARRY));
        cpu.exec(&mut  mem);
        assert_eq!(0xE0, cpu.r.b);
        cpu.exec(&mut  mem);
        assert_eq!(0xFE, cpu.r.a); assert!(cpu.r.f.contains(Flags::SIGN));
        cpu.exec(&mut  mem);
        assert_eq!(0x81, cpu.r.a);
        cpu.exec(&mut  mem);
        assert_eq!(0x80, cpu.r.c);
        cpu.exec(&mut  mem);
        assert_eq!(0x01, cpu.r.a); assert!(cpu.r.f.contains(Flags::CARRY)| cpu.r.f.contains(Flags::PARITY));
        cpu.exec(&mut  mem);
        assert_eq!(0xFF, cpu.r.d);
        cpu.exec(&mut  mem);
        assert_eq!(0x00, cpu.r.a); assert!(cpu.r.f.contains(Flags::ZERO) | cpu.r.f.contains(Flags::CARRY)| cpu.r.f.contains(Flags::HALFCARRY));
        cpu.exec(&mut  mem);
        assert_eq!(0x40, cpu.r.e);
        cpu.exec(&mut  mem);
        assert_eq!(0x40, cpu.r.a); assert_eq!(cpu.r.f, Flags::empty());
        cpu.exec(&mut  mem);
        assert_eq!(0x80, cpu.r.h);
        cpu.exec(&mut  mem);
        assert_eq!(0xC0, cpu.r.a); assert!(cpu.r.f.contains(Flags::SIGN));
        cpu.exec(&mut  mem);
        assert_eq!(0x33, cpu.r.l);
        cpu.exec(&mut  mem);
        assert_eq!(0xF3, cpu.r.a); assert!(cpu.r.f.contains(Flags::SIGN));
        cpu.exec(&mut  mem);
        assert_eq!(0x37, cpu.r.a); assert!(cpu.r.f.contains(Flags::CARRY));
    }

    #[test]
    fn test_call_cc_ret_cc() {
        let mut cpu = Z80::new();
        let mut mem = PlainMemory::new_64k();
        let prog = [
			0x97,               //      SUB A
			0xC4, 0x29, 0x02,   //      CALL NZ,l0
			0xCC, 0x29, 0x02,   //      CALL Z,l0
			0xC6, 0x01,         //      ADD A,0x01
			0xCC, 0x2B, 0x02,   //      CALL Z,l1
			0xC4, 0x2B, 0x02,   //      CALL NZ,l1
			0x07,               //      RLCA
			0xEC, 0x2D, 0x02,   //      CALL PE,l2
			0xE4, 0x2D, 0x02,   //      CALL PO,l2
			0xD6, 0x03,         //      SUB 0x03
			0xF4, 0x2F, 0x02,   //      CALL P,l3
			0xFC, 0x2F, 0x02,   //      CALL M,l3
			0xD4, 0x31, 0x02,   //      CALL NC,l4
			0xDC, 0x31, 0x02,   //      CALL C,l4
			0xC9,               //      RET
			0xC0,               // l0:  RET NZ
			0xC8,               //      RET Z
			0xC8,               // l1:  RET Z
			0xC0,               //      RET NZ
			0xE8,               // l2:  RET PE
			0xE0,               //      RET PO
			0xF0,               // l3:  RET P
			0xF8,               //      RET M
			0xD0,               // l4:  RET NC
			0xD8,               //      RET C<Paste>
        ];
		mem.write(0x0204, &prog);
		cpu.r.pc = 0x0204;
		cpu.r.sp = 0x0100;

        cpu.exec(&mut  mem);
        assert_eq!(0x00, cpu.r.a);
        cpu.exec(&mut  mem);
        assert_eq!(0x0208, cpu.r.pc);
        cpu.exec(&mut  mem);
        assert_eq!(0x0229, cpu.r.pc);
        cpu.exec(&mut  mem);
        assert_eq!(0x022A, cpu.r.pc);
        cpu.exec(&mut  mem);
        assert_eq!(0x020B, cpu.r.pc);
        cpu.exec(&mut  mem);
        assert_eq!(0x01, cpu.r.a);
        cpu.exec(&mut  mem);
        assert_eq!(0x0210, cpu.r.pc);
        cpu.exec(&mut  mem);
        assert_eq!(0x022B, cpu.r.pc);
        cpu.exec(&mut  mem);
        assert_eq!(0x022C, cpu.r.pc);
        cpu.exec(&mut  mem);
        assert_eq!(0x0213, cpu.r.pc);
        cpu.exec(&mut  mem);
        assert_eq!(0x02, cpu.r.a);
        cpu.exec(&mut  mem);
        assert_eq!(0x0217, cpu.r.pc);
        cpu.exec(&mut  mem);
        assert_eq!(0x022D, cpu.r.pc);
        cpu.exec(&mut  mem);
        assert_eq!(0x022E, cpu.r.pc);
        cpu.exec(&mut  mem);
        assert_eq!(0x021A, cpu.r.pc);
        cpu.exec(&mut  mem);
        assert_eq!(0xFF, cpu.r.a);
        cpu.exec(&mut  mem);
        assert_eq!(0x021F, cpu.r.pc);
        cpu.exec(&mut  mem);
        assert_eq!(0x022F, cpu.r.pc);
        cpu.exec(&mut  mem);
        assert_eq!(0x0230, cpu.r.pc);
        cpu.exec(&mut  mem);
        assert_eq!(0x0222, cpu.r.pc);
        cpu.exec(&mut  mem);
        assert_eq!(0x0225, cpu.r.pc);
        cpu.exec(&mut  mem);
        assert_eq!(0x0231, cpu.r.pc);
        cpu.exec(&mut  mem);
        assert_eq!(0x0232, cpu.r.pc);
        cpu.exec(&mut  mem);
        assert_eq!(0x0228, cpu.r.pc);
    }

    #[test]
    fn test_call_ret() {
        let mut cpu = Z80::new();
        let mut mem = PlainMemory::new_64k();
        cpu.r.sp = 0xFFFF;
        let prog = [
            0xCD, 0x0A, 0x02,   // CALL l0
            0xCD, 0x0A, 0x02,   // CALL l0
            0xC9,               // l0: RET
        ];
        mem.write(0x0204, &prog);
        cpu.r.pc = 0x0204;
        cpu.exec(&mut  mem);
        assert_eq!(0x020A, cpu.r.pc);
        assert_eq!(0xFFFD, cpu.r.sp);
        assert_eq!(0x0207, mem.r16(0xFFFD));
        cpu.exec(&mut  mem);
        assert_eq!(0x0207, cpu.r.pc);
        assert_eq!(0xFFFF, cpu.r.sp);
        cpu.exec(&mut  mem);
        assert_eq!(0x020A, cpu.r.pc);
        assert_eq!(0xFFFD, cpu.r.sp);
        cpu.exec(&mut  mem);
        assert_eq!(0x020A, mem.r16(0xFFFD));
        assert_eq!(0x020A, cpu.r.pc);
        assert_eq!(0xFFFF, cpu.r.sp);
    }

    #[test]
    fn test_jp_cc_nn() {
        let mut cpu = Z80::new();
        let mut mem = PlainMemory::new_64k();
        let prog = [
            0x97,               //          SUB A
            0xC2, 0x0C, 0x02,   //          JP NZ,label0
            0xCA, 0x0C, 0x02,   //          JP Z,label0
            0x00,               //          NOP
            0xC6, 0x01,         // label0:  ADD A,0x01
            0xCA, 0x15, 0x02,   //          JP Z,label1
            0xC2, 0x15, 0x02,   //          JP NZ,label1
            0x00,               //          NOP
            0x07,               // label1:  RLCA
            0xEA, 0x1D, 0x02,   //          JP PE,label2
            0xE2, 0x1D, 0x02,   //          JP PO,label2
            0x00,               //          NOP
            0xC6, 0xFD,         // label2:  ADD A,0xFD
            0xF2, 0x26, 0x02,   //          JP P,label3
            0xFA, 0x26, 0x02,   //          JP M,label3
            0x00,               //          NOP
            0xD2, 0x2D, 0x02,   // label3:  JP NC,label4
            0xDA, 0x2D, 0x02,   //          JP C,label4
            0x00,               //          NOP
            0x00,               //          NOP
        ];
        mem.write(0x0204, &prog);
        cpu.r.pc = 0x0204;
        cpu.exec(&mut mem);
        assert_eq!(0x00, cpu.r.a); assert!(cpu.r.f.contains(Flags::ZERO | Flags::NEGATIVE));
        cpu.exec(&mut mem);
        assert_eq!(0x0208, cpu.r.pc); cpu.exec(&mut mem);
        assert_eq!(0x020C, cpu.r.pc); cpu.exec(&mut mem);
        assert_eq!(0x01, cpu.r.a); assert!(cpu.r.f.is_empty());
        cpu.exec(&mut mem);
        assert_eq!(0x0211, cpu.r.pc);
        cpu.exec(&mut mem);
        assert_eq!(0x0215, cpu.r.pc);
        cpu.exec(&mut mem);
        assert_eq!(0x02, cpu.r.a); assert!(cpu.r.f.is_empty());
        cpu.exec(&mut mem);
        assert_eq!(0x0219, cpu.r.pc);
        cpu.exec(&mut mem);
        assert_eq!(0x021D, cpu.r.pc);
        cpu.exec(&mut mem);
        assert_eq!(0xFF, cpu.r.a); assert!(cpu.r.f.contains(Flags::SIGN));
        cpu.exec(&mut mem);
        assert_eq!(0x0222, cpu.r.pc);
        cpu.exec(&mut  mem);
        assert_eq!(0x0226, cpu.r.pc);
        cpu.exec(&mut  mem);
        assert_eq!(0x022D, cpu.r.pc);
    }

    #[test]
    fn test_push_pop() {
        let mut cpu = Z80::new();
        let mut mem = PlainMemory::new_64k();
        let prog = [
            0x01, 0x34, 0x12,       // LD BC,0x1234
            0x11, 0x78, 0x56,       // LD DE,0x5678
            0x21, 0xBC, 0x9A,       // LD HL,0x9ABC
            0x3E, 0xEF,             // LD A,0xEF
            0xDD, 0x21, 0x45, 0x23, // LD IX,0x2345
            0xFD, 0x21, 0x89, 0x67, // LD IY,0x6789
            0x31, 0x00, 0x01,       // LD SP,0x0100
            0xF5,                   // PUSH AF
            0xC5,                   // PUSH BC
            0xD5,                   // PUSH DE
            0xE5,                   // PUSH HL
            0xDD, 0xE5,             // PUSH IX
            0xFD, 0xE5,             // PUSH IY
            0xF1,                   // POP AF
            0xC1,                   // POP BC
            0xD1,                   // POP DE
            0xE1,                   // POP HL
            0xDD, 0xE1,             // POP IX
            0xFD, 0xE1,             // POP IY
        ];
        mem.write(0x0000, &prog);

        cpu.exec(&mut  mem);
        assert_eq!(0x1234, cpu.r.get_u16(Register16Bit::BC));
        cpu.exec(&mut  mem);
        assert_eq!(0x5678, cpu.r.get_u16(Register16Bit::DE));
        cpu.exec(&mut  mem);
        assert_eq!(0x9ABC, cpu.r.get_u16(Register16Bit::HL));
        cpu.exec(&mut  mem);
        assert_eq!(0xEF00, cpu.r.get_u16(Register16Bit::AF));
        cpu.exec(&mut  mem);
        assert_eq!(0x2345, cpu.r.get_u16(Register16Bit::IX));
        cpu.exec(&mut  mem);
        assert_eq!(0x6789, cpu.r.get_u16(Register16Bit::IY));
        cpu.exec(&mut  mem);
        assert_eq!(0x0100, cpu.r.sp);
        cpu.exec(&mut  mem);
        assert_eq!(0xEF00, mem.r16(0x00FE)); assert_eq!(0x00FE, cpu.r.sp);
        cpu.exec(&mut  mem);
        assert_eq!(0x1234, mem.r16(0x00FC)); assert_eq!(0x00FC, cpu.r.sp);
        cpu.exec(&mut  mem);
        assert_eq!(0x5678, mem.r16(0x00FA)); assert_eq!(0x00FA, cpu.r.sp);
        cpu.exec(&mut  mem);
        assert_eq!(0x9ABC, mem.r16(0x00F8)); assert_eq!(0x00F8, cpu.r.sp);
        cpu.exec(&mut  mem);
        assert_eq!(0x2345, mem.r16(0x00F6)); assert_eq!(0x00F6, cpu.r.sp);
        cpu.exec(&mut  mem);
        assert_eq!(0x6789, mem.r16(0x00F4)); assert_eq!(0x00F4, cpu.r.sp);
        cpu.exec(&mut  mem);
        assert_eq!(0x6789, cpu.r.get_u16(Register16Bit::AF)); assert_eq!(0x00F6, cpu.r.sp);
        cpu.exec(&mut  mem);
        assert_eq!(0x2345, cpu.r.get_u16(Register16Bit::BC)); assert_eq!(0x00F8, cpu.r.sp);
        cpu.exec(&mut  mem);
        assert_eq!(0x9ABC, cpu.r.get_u16(Register16Bit::DE)); assert_eq!(0x00FA, cpu.r.sp);
        cpu.exec(&mut  mem);
        assert_eq!(0x5678, cpu.r.get_u16(Register16Bit::HL)); assert_eq!(0x00FC, cpu.r.sp);
        cpu.exec(&mut  mem);
        assert_eq!(0x1234, cpu.r.get_u16(Register16Bit::IX)); assert_eq!(0x00FE, cpu.r.sp);
        cpu.exec(&mut  mem);
        assert_eq!(0xEF00, cpu.r.get_u16(Register16Bit::IY)); assert_eq!(0x0100, cpu.r.sp);
    }

    #[test]
    fn test_cp_r() {
        let mut cpu = Z80::new();
    let mut mem = PlainMemory::new_64k();
        let prog = [
            0x3E, 0x04,     // LD A,0x04
            0x06, 0x05,     // LD B,0x05
            0x0E, 0x03,     // LD C,0x03
            0x16, 0xff,     // LD D,0xff
            0x1E, 0xaa,     // LD E,0xaa
            0x26, 0x80,     // LD H,0x80
            0x2E, 0x7f,     // LD L,0x7f
            0xBF,           // CP A
            0xB8,           // CP B
            0xB9,           // CP C
            0xBA,           // CP D
            0xBB,           // CP E
            0xBC,           // CP H
            0xBD,           // CP L
            0xFE, 0x04,     // CP 0x04
        ];
        mem.write(0x0000, &prog);
        cpu.exec(&mut  mem);
        assert_eq!(0x04, cpu.r.a);
        cpu.exec(&mut  mem);
        assert_eq!(0x05, cpu.r.b);
        cpu.exec(&mut  mem);
        assert_eq!(0x03, cpu.r.c);
        cpu.exec(&mut  mem);
        assert_eq!(0xff, cpu.r.d);
        cpu.exec(&mut  mem);
        assert_eq!(0xaa, cpu.r.e);
        cpu.exec(&mut  mem);
        assert_eq!(0x80, cpu.r.h);
        cpu.exec(&mut  mem);
        assert_eq!(0x7f, cpu.r.l);
        cpu.exec(&mut  mem);
        assert_eq!(0x04, cpu.r.a); assert!(cpu.r.f.contains(Flags::ZERO | Flags::NEGATIVE));
        cpu.exec(&mut  mem);
        assert_eq!(0x04, cpu.r.a); assert!(cpu.r.f.contains(Flags::CARRY | Flags::NEGATIVE | Flags::SIGN));
        cpu.exec(&mut  mem);
        assert_eq!(0x04, cpu.r.a); assert!(cpu.r.f.contains(Flags::NEGATIVE));
        cpu.exec(&mut  mem);
        assert_eq!(0x04, cpu.r.a); assert!(cpu.r.f.contains(Flags::CARRY | Flags::NEGATIVE));
        cpu.exec(&mut  mem);
        assert_eq!(0x04, cpu.r.a); assert!(cpu.r.f.contains(Flags::CARRY | Flags::NEGATIVE));
        cpu.exec(&mut  mem);
        assert_eq!(0x04, cpu.r.a); assert!(cpu.r.f.contains(Flags::CARRY | Flags::NEGATIVE | Flags::SIGN | Flags::PARITY));
        cpu.exec(&mut  mem);
        assert_eq!(0x04, cpu.r.a); assert!(cpu.r.f.contains(Flags::CARRY | Flags::NEGATIVE | Flags::SIGN));
        cpu.exec(&mut  mem);
        assert_eq!(0x04, cpu.r.a); assert!(cpu.r.f.contains(Flags::ZERO | Flags::NEGATIVE));
    }

    #[test]
    fn test_jr_cc() {
        let mut cpu = Z80::new();
        let mut mem = PlainMemory::new_64k();
        let prog = [
            0x97,           //      SUB A
            0x20, 0x03,     //      JR NZ l0
            0x28, 0x01,     //      JR Z, l0
            0x00,           //      NOP
            0xC6, 0x01,     // l0:  ADD A,0x01
            0x28, 0x03,     //      JR Z, l1
            0x20, 0x01,     //      HR NZ, l1
            0x00,           //      NOP
            0xD6, 0x03,     // l1:  SUB 0x03
            0x30, 0x03,     //      JR NC, l2
            0x38, 0x01,     //      JR C, l2
            0x00,           //      NOP
            0x00,           //      NOP
        ];
        mem.write(0x204, &prog);
        cpu.r.pc = 0x0204;
        cpu.exec(&mut  mem);
        assert_eq!(0x00, cpu.r.a);
        cpu.exec(&mut  mem);
        assert_eq!(0x0207, cpu.r.pc);
        cpu.exec(&mut  mem);
        assert_eq!(0x020A, cpu.r.pc);
        cpu.exec(&mut  mem);
        assert_eq!(0x01, cpu.r.a);
        cpu.exec(&mut  mem);
        assert_eq!(0x020E, cpu.r.pc);
        cpu.exec(&mut  mem);
        assert_eq!(0x0211, cpu.r.pc);
        cpu.exec(&mut  mem);
        assert_eq!(0xFE, cpu.r.a);
        cpu.exec(&mut  mem);
        assert_eq!(0x0215, cpu.r.pc);
        cpu.exec(&mut  mem);
        assert_eq!(0x0218, cpu.r.pc);
    }

    #[test]
    fn test_jp_jr() {
        let mut cpu = Z80::new();
        let mut mem = PlainMemory::new_64k();
        let prog = [
            0x21, 0x16, 0x02,           //      LD HL,l3
            0xDD, 0x21, 0x19, 0x02,     //      LD IX,l4
            0xFD, 0x21, 0x21, 0x02,     //      LD IY,l5
            0xC3, 0x14, 0x02,           //      JP l0
            0x18, 0x04,                 // l1:  JR l2
            0x18, 0xFC,                 // l0:  JR l1
            0xDD, 0xE9,                 // l3:  JP (IX)
            0xE9,                       // l2:  JP (HL)
            0xFD, 0xE9,                 // l4:  JP (IY)
            0x18, 0x06,                 // l6:  JR l7
            0x00, 0x00, 0x00, 0x00,     //      4x NOP
            0x18, 0xF8,                 // l5:  JR l6
            0x00                        // l7:  NOP
        ];
        mem.write(0x0204, &prog);
        cpu.r.pc = 0x0204;
        cpu.exec(&mut mem);
        assert_eq!(0x0216, cpu.r.get_u16(Register16Bit::HL));
        cpu.exec(&mut mem);
        assert_eq!(0x0219, cpu.r.get_u16(Register16Bit::IX));
        cpu.exec(&mut mem);
        assert_eq!(0x0221, cpu.r.get_u16(Register16Bit::IY));
        cpu.exec(&mut mem);
        assert_eq!(0x0214, cpu.r.pc);
        cpu.exec(&mut mem);
        assert_eq!(0x0212, cpu.r.pc);
        cpu.exec(&mut mem);
        assert_eq!(0x0218, cpu.r.pc);
        cpu.exec(&mut mem);
        assert_eq!(0x0216, cpu.r.pc);
        cpu.exec(&mut mem);
        assert_eq!(0x0219, cpu.r.pc);
        cpu.exec(&mut mem);
        assert_eq!(0x0221, cpu.r.pc);
        cpu.exec(&mut mem);
        assert_eq!(0x021B, cpu.r.pc);
        cpu.exec(&mut mem);
        assert_eq!(0x0223, cpu.r.pc);
    }

    #[test]
    fn test_djnz() {
        let mut cpu = Z80::new();
        let mut mem = PlainMemory::new_64k();
        let prog = [
            0x06, 0x03,     // LD BC,0x03
            0x97,           // SUB A
            0x3C,           // loop: INC A
            0x10, 0xFD,     // DJNZ loop
            0x00,           // NOP
        ];
        mem.write(0x0204, &prog);
        cpu.r.pc = 0x0204;
        cpu.exec(&mut  mem);
        assert_eq!(0x03, cpu.r.b);
        cpu.exec(&mut  mem);
        assert_eq!(0x00, cpu.r.a);
        cpu.exec(&mut  mem);
        assert_eq!(0x01, cpu.r.a);
        cpu.exec(&mut  mem);
        assert_eq!(0x02, cpu.r.b); assert_eq!(0x0207, cpu.r.pc);
        cpu.exec(&mut  mem);
        assert_eq!(0x02, cpu.r.a);
        cpu.exec(&mut  mem);
        assert_eq!(0x01, cpu.r.b); assert_eq!(0x0207, cpu.r.pc);
        cpu.exec(&mut  mem);
        assert_eq!(0x03, cpu.r.a);
        cpu.exec(&mut  mem);
        assert_eq!(0x00, cpu.r.b); assert_eq!(0x020A, cpu.r.pc);
    }

    #[test]
    fn test_inc_dec_r() {
        let mut cpu = Z80::new();
        let mut mem = PlainMemory::new_64k();
        let prog = [
            0x3e, 0x00,         // LD A,0x00
            0x06, 0xFF,         // LD B,0xFF
            0x0e, 0x0F,         // LD C,0x0F
            0x16, 0x0E,         // LD D,0x0E
            0x1E, 0x7F,         // LD E,0x7F
            0x26, 0x3E,         // LD H,0x3E
            0x2E, 0x23,         // LD L,0x23
            0x3C,               // INC A
            0x3D,               // DEC A
            0x04,               // INC B
            0x05,               // DEC B
            0x0C,               // INC C
            0x0D,               // DEC C
            0x14,               // INC D
            0x15,               // DEC D
            0xFE, 0x01,         // CP 0x01  // set carry flag (should be preserved)
            0x1C,               // INC E
            0x1D,               // DEC E
            0x24,               // INC H
            0x25,               // DEC H
            0x2C,               // INC L
            0x2D,               // DEC L
        ];
        mem.write(0x0000, &prog);

        for _ in 0..7 {
            cpu.exec(&mut mem);
        }

        cpu.exec(&mut  mem);
        assert_eq!(0x01, cpu.r.a); assert!(cpu.r.f.is_empty());
        cpu.exec(&mut  mem);
        assert_eq!(0x00, cpu.r.a); assert!(cpu.r.f.contains(Flags::ZERO) && cpu.r.f.contains(Flags::NEGATIVE));
        cpu.exec(&mut  mem);
        assert_eq!(0x00, cpu.r.b); assert!(cpu.r.f.contains(Flags::ZERO) & cpu.r.f.contains(Flags::HALFCARRY));
        cpu.exec(&mut  mem);
        assert_eq!(0xFF, cpu.r.b); assert!(cpu.r.f.contains(Flags::NEGATIVE) & cpu.r.f.contains(Flags::HALFCARRY) & cpu.r.f.contains(Flags::SIGN));
        cpu.exec(&mut  mem);
        assert_eq!(0x10, cpu.r.c); assert!(cpu.r.f.contains(Flags::HALFCARRY));
        cpu.exec(&mut  mem);
        assert_eq!(0x0F, cpu.r.c); assert!(cpu.r.f.contains(Flags::NEGATIVE) & cpu.r.f.contains(Flags::HALFCARRY));
        cpu.exec(&mut  mem);
        assert_eq!(0x0F, cpu.r.d); assert!(cpu.r.f.is_empty());
        cpu.exec(&mut  mem);
        assert_eq!(0x0E, cpu.r.d); assert!(cpu.r.f.contains(Flags::NEGATIVE));
        cpu.exec(&mut  mem);
        assert_eq!(0x00, cpu.r.a); 
        assert!(cpu.r.f.contains(Flags::SIGN | Flags::CARRY | Flags::NEGATIVE));
        cpu.exec(&mut  mem);
        assert_eq!(0x80, cpu.r.e); assert!(cpu.r.f.contains(Flags::SIGN | Flags::CARRY));
        cpu.exec(&mut  mem);
        assert_eq!(0x7F, cpu.r.e); assert!(cpu.r.f.contains(Flags::NEGATIVE | Flags::CARRY | Flags::HALFCARRY));
        cpu.exec(&mut  mem);
        assert_eq!(0x3F, cpu.r.h); assert!(cpu.r.f.contains(Flags::CARRY));
        cpu.exec(&mut  mem);
        assert_eq!(0x3E, cpu.r.h); assert!(cpu.r.f.contains(Flags::NEGATIVE | Flags::CARRY));
        cpu.exec(&mut  mem);
        assert_eq!(0x24, cpu.r.l); assert!(cpu.r.f.contains(Flags::CARRY));
        cpu.exec(&mut  mem);
        assert_eq!(0x23, cpu.r.l); assert!(cpu.r.f.contains(Flags::NEGATIVE | Flags::CARRY));        
    }

    #[test]
    fn test_inc_dec_ihlixiy() {
        let mut cpu = Z80::new();
        let mut mem = PlainMemory::new_64k();
        let data = [ 0x00, 0x3F, 0x7F ];
        mem.write(0x1000, &data);
        let prog = [
            0x21, 0x00, 0x10,           // LD HL,0x1000
            0xDD, 0x21, 0x00, 0x10,     // LD IX,0x1000
            0xFD, 0x21, 0x03, 0x10,     // LD IY,0x1003
            0x35,                       // DEC (HL)
            0x34,                       // INC (HL)
            0xDD, 0x34, 0x01,           // INC (IX+1)
            0xDD, 0x35, 0x01,           // DEC (IX+1)
            0xFD, 0x34, 0xFF,           // INC (IY-1)
            0xFD, 0x35, 0xFF,           // DEC (IY-1)
        ];
        mem.write(0x0000, &prog);

        // skip loads
        for _ in 0..4 {
            cpu.exec(&mut mem);
        }
        assert_eq!(0xFF, mem.r8(0x1000)); assert!(cpu.r.f.contains(Flags::SIGN | Flags::HALFCARRY | Flags::NEGATIVE));
        cpu.exec(&mut  mem);
        assert_eq!(0x00, mem.r8(0x1000)); assert!(cpu.r.f.contains(Flags::ZERO | Flags::HALFCARRY));
        cpu.exec(&mut  mem);
        assert_eq!(0x40, mem.r8(0x1001)); assert!(cpu.r.f.contains(Flags::HALFCARRY));
        cpu.exec(&mut  mem);
        assert_eq!(0x3F, mem.r8(0x1001)); assert!(cpu.r.f.contains(Flags::NEGATIVE | Flags::HALFCARRY));
        cpu.exec(&mut  mem);
        assert_eq!(0x80, mem.r8(0x1002)); assert!(cpu.r.f.contains(Flags::SIGN | Flags::HALFCARRY));
        cpu.exec(&mut  mem);
        assert_eq!(0x7F, mem.r8(0x1002)); assert!(cpu.r.f.contains(Flags::HALFCARRY | Flags::NEGATIVE));
    }

    #[test]
    fn test_add_ihlixiy() {
        let mut cpu = Z80::new();
        let mut mem = PlainMemory::new_64k();
        let data = [ 0x41, 0x61, 0x81 ];
        mem.write(0x1000, &data);

        let prog = [
            0x21, 0x00, 0x10,       // LD HL,0x1000
            0xDD, 0x21, 0x00, 0x10, // LD IX,0x1000
            0xFD, 0x21, 0x03, 0x10, // LD IY,0x1003
            0x3E, 0x00,             // LD A,0x00
            0x86,                   // ADD A,(HL)
            0xDD, 0x86, 0x01,       // ADD A,(IX+1)
            0xFD, 0x86, 0xFF,       // ADD A,(IY-1)
        ];
        mem.write(0x0000, &prog);
        cpu.exec(&mut mem);
        assert_eq!(0x1000, cpu.r.get_u16(Register16Bit::HL));
        cpu.exec(&mut mem);
        assert_eq!(0x1000, cpu.r.get_u16(Register16Bit::IX));
        cpu.exec(&mut mem);
        assert_eq!(0x1003, cpu.r.get_u16(Register16Bit::IY));
        cpu.exec(&mut mem);
        assert_eq!(0x00, cpu.r.a);
        cpu.exec(&mut mem);
        assert_eq!(0x41, cpu.r.a); assert_eq!(cpu.r.f, Flags::empty());
        cpu.exec(&mut mem);
        assert_eq!(0xA2, cpu.r.a); assert!(cpu.r.f.contains(Flags::SIGN | Flags::PARITY));
        cpu.exec(&mut mem);
        assert_eq!(0x23, cpu.r.a); assert!(cpu.r.f.contains(Flags::PARITY | Flags::CARRY));
    }

    #[test]
    fn test_inc_dec_ssixiy() {
        let mut cpu = Z80::new();
        let mut mem = PlainMemory::new_64k();
        let prog = [
            0x01, 0x00, 0x00,       // LD BC,0x0000
            0x11, 0xFF, 0xFF,       // LD DE,0xffff
            0x21, 0xFF, 0x00,       // LD HL,0x00ff
            0x31, 0x11, 0x11,       // LD SP,0x1111
            0xDD, 0x21, 0xFF, 0x0F, // LD IX,0x0fff
            0xFD, 0x21, 0x34, 0x12, // LD IY,0x1234
            0x0B,                   // DEC BC
            0x03,                   // INC BC
            0x13,                   // INC DE
            0x1B,                   // DEC DE
            0x23,                   // INC HL
            0x2B,                   // DEC HL
            0x33,                   // INC SP
            0x3B,                   // DEC SP
            0xDD, 0x23,             // INC IX
            0xDD, 0x2B,             // DEC IX
            0xFD, 0x23,             // INC IY
            0xFD, 0x2B,             // DEC IY
        ];
        mem.write(0x0000, &prog);

        for _ in 0..7 {
            cpu.exec(&mut mem);
        }
        assert_eq!(0xFFFF, cpu.r.get_u16(Register16Bit::BC));
        cpu.exec(&mut mem);
        assert_eq!(0x0000, cpu.r.get_u16(Register16Bit::BC));
        cpu.exec(&mut mem);
        assert_eq!(0x0000, cpu.r.get_u16(Register16Bit::DE));
        cpu.exec(&mut mem);
        assert_eq!(0xFFFF, cpu.r.get_u16(Register16Bit::DE));
        cpu.exec(&mut mem);
        assert_eq!(0x0100, cpu.r.get_u16(Register16Bit::HL));
        cpu.exec(&mut mem);
        assert_eq!(0x00FF, cpu.r.get_u16(Register16Bit::HL));
        cpu.exec(&mut mem);
        assert_eq!(0x1112, cpu.r.get_u16(Register16Bit::SP));
        cpu.exec(&mut mem);
        assert_eq!(0x1111, cpu.r.get_u16(Register16Bit::SP));
        cpu.exec(&mut mem);
        assert_eq!(0x1000, cpu.r.get_u16(Register16Bit::IX));
        cpu.exec(&mut mem);
        assert_eq!(0x0FFF, cpu.r.get_u16(Register16Bit::IX));
        cpu.exec(&mut mem);
        assert_eq!(0x1235, cpu.r.get_u16(Register16Bit::IY));
        cpu.exec(&mut mem);
        assert_eq!(0x1234, cpu.r.get_u16(Register16Bit::IY));
    }

    #[test]
    fn test_adc_r() {
        let mut cpu = Z80::new();
    let mut mem = PlainMemory::new_64k();
        let prog = [
            0x3E, 0x00,         // LD A,0x00
            0x06, 0x41,         // LD B,0x41
            0x0E, 0x61,         // LD C,0x61
            0x16, 0x81,         // LD D,0x81
            0x1E, 0x41,         // LD E,0x41
            0x26, 0x61,         // LD H,0x61
            0x2E, 0x81,         // LD L,0x81
            0x8F,               // ADC A,A
            0x88,               // ADC A,B
            0x89,               // ADC A,C
            0x8A,               // ADC A,D
            0x8B,               // ADC A,E
            0x8C,               // ADC A,H
            0x8D,               // ADC A,L
            0xCE, 0x01,         // ADC A,0x01
        ];
        mem.write(0x0000, &prog);
        cpu.exec(&mut  mem);
        assert_eq!(0x00, cpu.r.a);
        cpu.exec(&mut  mem);
        assert_eq!(0x41, cpu.r.b);
        cpu.exec(&mut  mem);
        assert_eq!(0x61, cpu.r.c);
        cpu.exec(&mut  mem);
        assert_eq!(0x81, cpu.r.d);
        cpu.exec(&mut  mem);
        assert_eq!(0x41, cpu.r.e);
        cpu.exec(&mut  mem);
        assert_eq!(0x61, cpu.r.h);
        cpu.exec(&mut  mem);
        assert_eq!(0x81, cpu.r.l);
        cpu.exec(&mut  mem);
        assert_eq!(0x00, cpu.r.a); assert!(cpu.r.f.contains(Flags::ZERO));
        cpu.exec(&mut  mem);
        assert_eq!(0x41, cpu.r.a); assert_eq!(cpu.r.f, Flags::empty());
        cpu.exec(&mut  mem);
        assert_eq!(0xA2, cpu.r.a); assert!(cpu.r.f.contains(Flags::SIGN) && cpu.r.f.contains(Flags::PARITY));
        cpu.exec(&mut  mem);
        assert_eq!(0x23, cpu.r.a); assert!(cpu.r.f.contains(Flags::CARRY));
        cpu.exec(&mut  mem);
        assert_eq!(0x65, cpu.r.a); assert_eq!(cpu.r.f, Flags::empty());
        cpu.exec(&mut  mem);
        assert_eq!(0xC6, cpu.r.a); assert!(cpu.r.f.contains(Flags::SIGN) && cpu.r.f.contains(Flags::PARITY));
        cpu.exec(&mut  mem);
        assert_eq!(0x47, cpu.r.a); assert!(cpu.r.f.contains(Flags::CARRY));
        cpu.exec(&mut  mem);
        assert_eq!(0x49, cpu.r.a); assert_eq!(cpu.r.f, Flags::empty());
    }

    #[test]
    fn test_sbc_r() {
        let mut cpu = Z80::new();
        let mut mem = PlainMemory::new_64k();
        let prog = [
            0x3E, 0x04,     // LD A,0x04
            0x06, 0x01,     // LD B,0x01
            0x0E, 0xF8,     // LD C,0xF8
            0x16, 0x0F,     // LD D,0x0F
            0x1E, 0x79,     // LD E,0x79
            0x26, 0xC0,     // LD H,0xC0
            0x2E, 0xBF,     // LD L,0xBF
            0x97,           // SUB A,A
            0x98,           // SBC A,B
            0x99,           // SBC A,C
            0x9A,           // SBC A,D
            0x9B,           // SBC A,E
            0x9C,           // SBC A,H
            0x9D,           // SBC A,L
            0xDE, 0x01,     // SBC A,0x01
            0xDE, 0xFE,     // SBC A,0xFE
        ];
        mem.write(0x0000, &prog);

        for _ in 0..7 {
            cpu.exec(&mut  mem);
        }
        cpu.exec(&mut  mem);
        assert_eq!(0x00, cpu.r.a); assert!(cpu.r.f.contains(Flags::ZERO | Flags::NEGATIVE));
        cpu.exec(&mut  mem);
        assert_eq!(0xFF, cpu.r.a); assert!(cpu.r.f.contains(Flags::SIGN | Flags::NEGATIVE | Flags::CARRY));
        cpu.exec(&mut  mem);
        assert_eq!(0x06, cpu.r.a); assert!(cpu.r.f.contains(Flags::NEGATIVE));
        cpu.exec(&mut  mem);
        assert_eq!(0xF7, cpu.r.a); assert!(cpu.r.f.contains(Flags::SIGN | Flags::NEGATIVE | Flags::CARRY | Flags:: HALFCARRY));
        cpu.exec(&mut  mem);
        assert_eq!(0x7D, cpu.r.a); assert!(cpu.r.f.contains(Flags::PARITY | Flags::NEGATIVE | Flags:: HALFCARRY));
        cpu.exec(&mut  mem);
        assert_eq!(0xBD, cpu.r.a); assert!(cpu.r.f.contains(Flags::SIGN | Flags::NEGATIVE | Flags::CARRY));
        cpu.exec(&mut  mem);
        assert_eq!(0xFD, cpu.r.a); assert!(cpu.r.f.contains(Flags::SIGN | Flags::NEGATIVE | Flags::CARRY));
        cpu.exec(&mut  mem);
        assert_eq!(0xFB, cpu.r.a); assert!(cpu.r.f.contains(Flags::SIGN | Flags::NEGATIVE));
        cpu.exec(&mut  mem);
        assert_eq!(0xFD, cpu.r.a); assert!(cpu.r.f.contains(Flags::SIGN | Flags::NEGATIVE | Flags::CARRY));        
    }

    // #[test]
    // fn test_add_adc_sbc_16_2() {
    //     let mut cpu = Z80::new();
    //     let mut mem = PlainMemory::new_64k();
    //     let prog = [
    //         0x2E, 0x81,             // LD L,0x81
    //         0x3E, 0xC6,             // LD A,0xC6
    //         0x8D,                   // ADC A,L
    //         0x21, 0xff, 0x7f,       // LD HL,0x7fff
    //         0x01, 0x00, 0x80,       // LD BC,0x8000
    //         0x11, 0x00, 0x80,       // LD DE,0x8000
    //         0xED, 0x52,             // SBC HL,DE
    //         0x21, 0x00, 0x00,       // LD HL,0x0000
    //         0xED, 0x52,             // SBC HL,DE
    //     ];
    //     mem.write(0x0000, &prog);
    //     cpu.exec(&mut  mem);
    //     cpu.exec(&mut  mem);
    //     cpu.exec(&mut  mem);
    //     cpu.exec(&mut  mem);
    //     assert_eq!(0x7fff, cpu.r.get_u16(Register16Bit::HL));
    //     cpu.exec(&mut  mem);
    //     assert_eq!(0x8000, cpu.r.get_u16(Register16Bit::BC));
    //     cpu.exec(&mut  mem);
    //     assert_eq!(0x8000, cpu.r.get_u16(Register16Bit::DE));
    //     cpu.exec(&mut  mem);
    //     assert_eq!(0x7fff, cpu.r.get_u16(Register16Bit::HL)); assert_eq!(cpu.r.f, Flags::empty());
    // }

    #[test]
    fn test_add_adc_sbc_16() {
        let mut cpu = Z80::new();
        let mut mem = PlainMemory::new_64k();
        let prog = [
            0x21, 0xFC, 0x00,       // LD HL,0x00FC
            0x01, 0x08, 0x00,       // LD BC,0x0008
            0x11, 0xFF, 0xFF,       // LD DE,0xFFFF
            0x09,                   // ADD HL,BC
            0x19,                   // ADD HL,DE
            0xED, 0x4A,             // ADC HL,BC
            0x29,                   // ADD HL,HL
            0x19,                   // ADD HL,DE
            0xED, 0x42,             // SBC HL,BC
            0xDD, 0x21, 0xFC, 0x00, // LD IX,0x00FC
            0x31, 0x00, 0x10,       // LD SP,0x1000
            0xDD, 0x09,             // ADD IX, BC
            0xDD, 0x19,             // ADD IX, DE
            0xDD, 0x29,             // ADD IX, IX
            0xDD, 0x39,             // ADD IX, SP
            0xFD, 0x21, 0xFF, 0xFF, // LD IY,0xFFFF
            0xFD, 0x09,             // ADD IY,BC
            0xFD, 0x19,             // ADD IY,DE
            0xFD, 0x29,             // ADD IY,IY
            0xFD, 0x39,             // ADD IY,SP
        ];
        mem.write(0x0000, &prog);
        cpu.exec(&mut  mem);
        assert_eq!(0x00FC, cpu.r.get_u16(Register16Bit::HL));
        cpu.exec(&mut  mem);
        assert_eq!(0x0008, cpu.r.get_u16(Register16Bit::BC));
        cpu.exec(&mut  mem);
        assert_eq!(0xFFFF, cpu.r.get_u16(Register16Bit::DE));
        cpu.exec(&mut  mem);
        assert_eq!(0x0104, cpu.r.get_u16(Register16Bit::HL)); assert_eq!(cpu.r.f, Flags::empty());
        cpu.exec(&mut  mem);
        assert_eq!(0x0103, cpu.r.get_u16(Register16Bit::HL)); assert!(cpu.r.f.contains(Flags::HALFCARRY | Flags::CARRY));
        cpu.exec(&mut  mem);
        assert_eq!(0x010C, cpu.r.get_u16(Register16Bit::HL)); assert_eq!(cpu.r.f, Flags::empty());
        cpu.exec(&mut  mem);
        assert_eq!(0x0218, cpu.r.get_u16(Register16Bit::HL)); assert_eq!(cpu.r.f, Flags::empty());
        cpu.exec(&mut  mem);
        assert_eq!(0x0217, cpu.r.get_u16(Register16Bit::HL)); assert!(cpu.r.f.contains(Flags::HALFCARRY | Flags::CARRY));
        cpu.exec(&mut  mem);
        assert_eq!(0x020E, cpu.r.get_u16(Register16Bit::HL)); assert!(cpu.r.f.contains(Flags::NEGATIVE));
        cpu.exec(&mut  mem);
        assert_eq!(0x00FC, cpu.r.get_u16(Register16Bit::IX));
        cpu.exec(&mut  mem);
        assert_eq!(0x1000, cpu.r.get_u16(Register16Bit::SP));
        cpu.exec(&mut  mem);
        assert_eq!(0x0104, cpu.r.get_u16(Register16Bit::IX)); assert_eq!(cpu.r.f, Flags::empty());
        cpu.exec(&mut  mem);
        assert_eq!(0x0103, cpu.r.get_u16(Register16Bit::IX)); assert!(cpu.r.f.contains(Flags::HALFCARRY | Flags::CARRY));
        cpu.exec(&mut  mem);
        assert_eq!(0x0206, cpu.r.get_u16(Register16Bit::IX)); assert_eq!(cpu.r.f, Flags::empty());
        cpu.exec(&mut  mem);
        assert_eq!(0x1206, cpu.r.get_u16(Register16Bit::IX)); assert_eq!(cpu.r.f, Flags::empty());
        cpu.exec(&mut  mem);
        assert_eq!(0xFFFF, cpu.r.get_u16(Register16Bit::IY));
        cpu.exec(&mut  mem);
        assert_eq!(0x0007, cpu.r.get_u16(Register16Bit::IY)); assert!(cpu.r.f.contains(Flags::HALFCARRY | Flags::CARRY));
        cpu.exec(&mut  mem);
        assert_eq!(0x0006, cpu.r.get_u16(Register16Bit::IY)); assert!(cpu.r.f.contains(Flags::HALFCARRY | Flags::CARRY));
        cpu.exec(&mut  mem);
        assert_eq!(0x000C, cpu.r.get_u16(Register16Bit::IY)); assert_eq!(cpu.r.f, Flags::empty());
        cpu.exec(&mut  mem);
        assert_eq!(0x100C, cpu.r.get_u16(Register16Bit::IY)); assert_eq!(cpu.r.f, Flags::empty());
    }

    #[test]
    fn test_ldir() {
        let mut cpu = Z80::new();
        let mut mem = PlainMemory::new_64k();
        let data = [ 0x01, 0x02, 0x03 ];
        mem.write(0x1000, &data);
        let prog = [
            0x21, 0x00, 0x10,       // LD HL,0x1000
            0x11, 0x00, 0x20,       // LD DE,0x2000
            0x01, 0x03, 0x00,       // LD BC,0x0003
            0xED, 0xB0,             // LDIR
            0x3E, 0x33,             // LD A,0x33
        ];
        mem.write(0x0000, &prog);

        // skip loads
        for _ in 0..3 {
            cpu.exec(&mut mem);
        }
        cpu.exec(&mut mem);
        assert_eq!(0x01, mem.r8(0x2000));
        assert_eq!(0x02, mem.r8(0x2001));
        // assert!(cpu.r.f.contains(Flags::PARITY));
        assert_eq!(0x1003, cpu.r.get_u16(Register16Bit::HL));
        assert_eq!(0x2003, cpu.r.get_u16(Register16Bit::DE));
        assert_eq!(0x0000, cpu.r.get_u16(Register16Bit::BC));
        assert_eq!(0x03, mem.r8(0x2002));
        assert_eq!(cpu.r.f, Flags::empty());
        cpu.exec(&mut mem);
        assert_eq!(0x33, cpu.r.a);
    }

    #[test]
    fn test_ldi() {
        let mut cpu = Z80::new();
        let mut mem = PlainMemory::new_64k();
        let data = [ 0x01, 0x02, 0x03 ];
        mem.write(0x1000, &data);
        let prog = [
            0x21, 0x00, 0x10,       // LD HL,0x1000
            0x11, 0x00, 0x20,       // LD DE,0x2000
            0x01, 0x03, 0x00,       // LD BC,0x0003
            0xED, 0xA0,             // LDI
            0xED, 0xA0,             // LDI
            0xED, 0xA0,             // LDI
        ];
        mem.write(0x0000, &prog);

        // skip loads
         for _ in 0..3 {
            cpu.exec(&mut mem);
        }
        cpu.exec(&mut mem);
        assert_eq!(0x1001, cpu.r.get_u16(Register16Bit::HL));
        assert_eq!(0x2001, cpu.r.get_u16(Register16Bit::DE));
        assert_eq!(0x0002, cpu.r.get_u16(Register16Bit::BC));
        assert_eq!(0x01, mem.r8(0x2000));
        assert!(cpu.r.f.contains(Flags::PARITY));
        cpu.exec(&mut mem);
        assert_eq!(0x1002, cpu.r.get_u16(Register16Bit::HL));
        assert_eq!(0x2002, cpu.r.get_u16(Register16Bit::DE));
        assert_eq!(0x0001, cpu.r.get_u16(Register16Bit::BC));
        assert_eq!(0x02, mem.r8(0x2001));
        assert!(cpu.r.f.contains(Flags::PARITY));
        cpu.exec(&mut mem);
        assert_eq!(0x1003, cpu.r.get_u16(Register16Bit::HL));
        assert_eq!(0x2003, cpu.r.get_u16(Register16Bit::DE));
        assert_eq!(0x0000, cpu.r.get_u16(Register16Bit::BC));
        assert_eq!(0x03, mem.r8(0x2002));
        assert_eq!(cpu.r.f, Flags::empty());
    }

    #[test]
    fn test_sub_r() {
        let mut cpu = Z80::new();
        let mut mem = PlainMemory::new_64k();
        let prog = [
            0x3E, 0x04,     // LD A,0x04
            0x06, 0x01,     // LD B,0x01
            0x0E, 0xF8,     // LD C,0xF8
            0x16, 0x0F,     // LD D,0x0F
            0x1E, 0x79,     // LD E,0x79
            0x26, 0xC0,     // LD H,0xC0
            0x2E, 0xBF,     // LD L,0xBF
            0x97,           // SUB A,A
            0x90,           // SUB A,B
            0x91,           // SUB A,C
            0x92,           // SUB A,D
            0x93,           // SUB A,E
            0x94,           // SUB A,H
            0x95,           // SUB A,L
            0xD6, 0x01,     // SUB A,0x01
            0xD6, 0xFE,     // SUB A,0xFE
        ];
        mem.write(0x0000, &prog);
        cpu.exec(&mut mem);
        assert_eq!(0x04, cpu.r.a); 
        cpu.exec(&mut mem);
        assert_eq!(0x01, cpu.r.b);
        cpu.exec(&mut mem);
        assert_eq!(0xF8, cpu.r.c);
        cpu.exec(&mut mem);
        assert_eq!(0x0F, cpu.r.d);
        cpu.exec(&mut mem);
        assert_eq!(0x79, cpu.r.e);
        cpu.exec(&mut mem);
        assert_eq!(0xC0, cpu.r.h);
        cpu.exec(&mut mem);
        assert_eq!(0xBF, cpu.r.l);
        cpu.exec(&mut mem);
        assert_eq!(0x00, cpu.r.a); assert!(cpu.r.f.contains(Flags::NEGATIVE | Flags::ZERO));
        cpu.exec(&mut mem);
        assert_eq!(0xFF, cpu.r.a); assert!(cpu.r.f.contains(Flags::SIGN | Flags::HALFCARRY | Flags::NEGATIVE | Flags::CARRY));
        cpu.exec(&mut mem);
        assert_eq!(0x07, cpu.r.a); assert!(cpu.r.f.contains(Flags::NEGATIVE));
        cpu.exec(&mut mem);
        assert_eq!(0xF8, cpu.r.a); assert!(cpu.r.f.contains(Flags::SIGN | Flags::HALFCARRY | Flags::NEGATIVE | Flags::CARRY));
        cpu.exec(&mut mem);
        assert_eq!(0x7F, cpu.r.a); assert!(cpu.r.f.contains(Flags::PARITY| Flags::HALFCARRY | Flags::NEGATIVE));
        cpu.exec(&mut mem);
        assert_eq!(0xBF, cpu.r.a); assert!(cpu.r.f.contains(Flags::SIGN | Flags::PARITY | Flags::NEGATIVE | Flags::CARRY));
        cpu.exec(&mut mem);
        assert_eq!(0x00, cpu.r.a); assert!(cpu.r.f.contains(Flags::NEGATIVE | Flags::ZERO));
        cpu.exec(&mut mem);
        assert_eq!(0xFF, cpu.r.a); assert!(cpu.r.f.contains(Flags::SIGN | Flags::HALFCARRY | Flags::NEGATIVE | Flags::CARRY));
        cpu.exec(&mut mem);
        assert_eq!(0x01, cpu.r.a); assert!(cpu.r.f.contains(Flags::NEGATIVE));        
    }

    #[test]
    fn sub8() {
        let mut cpu = Z80::new();
        cpu.r.a = 0x04;
        cpu.sub_n(0x04);
        assert_eq!(0x00, cpu.r.a);
        assert!(cpu.r.f.contains(Flags::NEGATIVE | Flags::ZERO));
        cpu.sub_n(0x01);
        assert_eq!(0xFF, cpu.r.a);
        assert!(cpu.r.f.contains(Flags::SIGN | Flags::HALFCARRY | Flags::NEGATIVE | Flags::CARRY));
        cpu.sub_n(0xF8);
        assert_eq!(0x07, cpu.r.a);
        assert!(cpu.r.f.contains(Flags::NEGATIVE));
        cpu.sub_n(0x0F);
        assert_eq!(0xF8, cpu.r.a);
        assert!(cpu.r.f.contains(Flags::SIGN | Flags::HALFCARRY | Flags::NEGATIVE | Flags::CARRY));
    }

    #[test]
    fn test_sla_r() {
        let mut cpu = Z80::new();
        let mut mem = PlainMemory::new_64k();
        let prog = [
            0x3E, 0x01,         // LD A,0x01
            0x06, 0x80,         // LD B,0x80
            0x0E, 0xAA,         // LD C,0xAA
            0x16, 0xFE,         // LD D,0xFE
            0x1E, 0x7F,         // LD E,0x7F
            0x26, 0x11,         // LD H,0x11
            0x2E, 0x00,         // LD L,0x00
            0xCB, 0x27,         // SLA A
            0xCB, 0x20,         // SLA B
            0xCB, 0x21,         // SLA C
            0xCB, 0x22,         // SLA D
            0xCB, 0x23,         // SLA E
            0xCB, 0x24,         // SLA H
            0xCB, 0x25,         // SLA L
        ];
        mem.write(0x0000, &prog);

        // skip loads
        for _ in 0..7 {
            cpu.exec(&mut mem);
        }
        cpu.exec(&mut mem);
        assert_eq!(0x02, cpu.r.a); assert_eq!(cpu.r.f, Flags::empty());
        cpu.exec(&mut mem);
        assert_eq!(0x00, cpu.r.b); assert!(cpu.r.f.contains(Flags::ZERO| Flags::PARITY | Flags::CARRY));
        cpu.exec(&mut mem);
        assert_eq!(0x54, cpu.r.c); assert!(cpu.r.f.contains(Flags::CARRY));
        cpu.exec(&mut mem);
        assert_eq!(0xFC, cpu.r.d); assert!(cpu.r.f.contains(Flags::SIGN| Flags::PARITY | Flags::CARRY));;
        cpu.exec(&mut mem);
        assert_eq!(0xFE, cpu.r.e); assert!(cpu.r.f.contains(Flags::SIGN));
        cpu.exec(&mut mem);
        assert_eq!(0x22, cpu.r.h); assert!(cpu.r.f.contains(Flags::PARITY));
        cpu.exec(&mut mem);
        assert_eq!(0x00, cpu.r.l); assert!(cpu.r.f.contains(Flags::ZERO | Flags::PARITY));
    }

    #[test]
    fn test_rlca_rla_rrca_rra() {
        let mut cpu = Z80::new();
        let mut mem = PlainMemory::new_64k();
        let prog = [
            0x3E, 0xA0,     // LD A,0xA0
            0x07,           // RLCA
            0x07,           // RLCA
            0x0F,           // RRCA
            0x0F,           // RRCA
            0x17,           // RLA
            0x17,           // RLA
            0x1F,           // RRA
            0x1F,           // RRA
        ];
        mem.write(0x0000, &prog);
        // cpu.r.set_f(0xff);
        cpu.r.f = Flags::from_bits_truncate(0xff);
        cpu.exec(&mut mem);
        assert_eq!(0xA0, cpu.r.a);
        cpu.exec(&mut mem);
        assert_eq!(0x41, cpu.r.a); 
        cpu.exec(&mut mem);
        assert_eq!(0x82, cpu.r.a); 
        cpu.exec(&mut mem);
        assert_eq!(0x41, cpu.r.a); 
        cpu.exec(&mut mem);
        assert_eq!(0xA0, cpu.r.a); 
        cpu.exec(&mut mem);
        assert_eq!(0x41, cpu.r.a); 
        cpu.exec(&mut mem);
        assert_eq!(0x83, cpu.r.a); 
        cpu.exec(&mut mem);
        assert_eq!(0x41, cpu.r.a); 
        cpu.exec(&mut mem);
        assert_eq!(0xA0, cpu.r.a);      
    }

    #[test]
    fn test_daa() {
        let mut cpu = Z80::new();
        let mut mem = PlainMemory::new_64k();
        let prog = [
            0x3E, 0x15,     // LD A,0x15
            0x06, 0x27,     // LD B,0x27
            0x80,           // ADD A,B
            0x27,           // DAA
            0x90,           // SUB B
            0x27,           // DAA
            0x3E, 0x90,     // LD A,0x90
            0x06, 0x15,     // LD B,0x15
            0x80,           // ADD A,B
            0x27,           // DAA
            0x90,           // SUB B
            0x27,           // DAA
        ];
        mem.write(0x0000, &prog);
        cpu.exec(&mut mem);
        assert_eq!(0x15, cpu.r.a);
        cpu.exec(&mut mem);
        assert_eq!(0x27, cpu.r.b);
        cpu.exec(&mut mem);
        assert_eq!(0x3C, cpu.r.a); assert_eq!(cpu.r.f, Flags::empty());
        cpu.exec(&mut mem);
        assert_eq!(0x42, cpu.r.a); assert!(cpu.r.f.contains(Flags::HALFCARRY | Flags::PARITY));
        cpu.exec(&mut mem);
        assert_eq!(0x1B, cpu.r.a); assert!(cpu.r.f.contains(Flags::HALFCARRY | Flags::NEGATIVE));
        cpu.exec(&mut mem);
        assert_eq!(0x15, cpu.r.a); assert!(cpu.r.f.contains(Flags::NEGATIVE));
        cpu.exec(&mut mem);
        assert_eq!(0x90, cpu.r.a); assert!(cpu.r.f.contains(Flags::NEGATIVE));
        cpu.exec(&mut mem);
        assert_eq!(0x15, cpu.r.b); assert!(cpu.r.f.contains(Flags::NEGATIVE));
        cpu.exec(&mut mem);
        assert_eq!(0xA5, cpu.r.a); assert!(cpu.r.f.contains(Flags::SIGN));
        cpu.exec(&mut mem);
        assert_eq!(0x05, cpu.r.a); assert!(cpu.r.f.contains(Flags::PARITY | Flags::CARRY));
        cpu.exec(&mut mem);
        assert_eq!(0xF0, cpu.r.a); assert!(cpu.r.f.contains(Flags::SIGN | Flags::NEGATIVE | Flags::CARRY));
        cpu.exec(&mut mem);
        assert_eq!(0x90, cpu.r.a); assert!(cpu.r.f.contains(Flags::SIGN | Flags::NEGATIVE | Flags::CARRY | Flags:: PARITY));
    }

    #[test]
    fn test_cpir() {
        let mut cpu = Z80::new();
        let mut mem = PlainMemory::new_64k();
        let data = [ 0x01, 0x02, 0x03, 0x04 ];
        mem.write(0x1000, &data);
        let prog = [
            0x21, 0x00, 0x10,       // ld hl,0x1000
            0x01, 0x04, 0x00,       // ld bc,0x0004
            0x3e, 0x03,             // ld a,0x03
            0xed, 0xb1,             // cpir
            0xed, 0xb1,             // cpir
        ];
        mem.write(0x0000, &prog);

        // skip loads
        for _ in 0..3 {
            cpu.exec(&mut mem);
        }
        cpu.exec(&mut mem);
        // assert_eq!(0x1001, cpu.r.get_u16(Register16Bit::HL));
        // assert_eq!(0x0003, cpu.r.get_u16(Register16Bit::BC));
        // assert!(cpu.r.f.contains(Flags::PARITY | Flags::NEGATIVE));
        // let f = cpu.reg.f() | CF;
        let current_flags = cpu.r.f.bits();
        cpu.r.f = Flags::from_bits_truncate(current_flags | 0x01);
        // assert_eq!(0x1002, cpu.r.get_u16(Register16Bit::HL));
        // assert_eq!(0x0002, cpu.r.get_u16(Register16Bit::BC));
        // assert!(cpu.r.f.contains(Flags::PARITY | Flags::NEGATIVE | Flags::CARRY));
        // cpu.exec(&mut mem);
        assert_eq!(0x1003, cpu.r.get_u16(Register16Bit::HL));
        assert_eq!(0x0001, cpu.r.get_u16(Register16Bit::BC));
        assert!(cpu.r.f.contains(Flags::ZERO | Flags::PARITY | Flags::NEGATIVE | Flags::CARRY));
        cpu.exec(&mut mem);
        assert_eq!(0x1004, cpu.r.get_u16(Register16Bit::HL));
        assert_eq!(0x0000, cpu.r.get_u16(Register16Bit::BC));
        assert!(cpu.r.f.contains(Flags::SIGN | Flags::HALFCARRY | Flags::NEGATIVE | Flags::CARRY))
    }

}
