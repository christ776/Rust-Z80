#[allow(unused_imports)]

mod test_opcodes {
use Z80::gfx_decoder::{ TileDecoder, Decoder };
use Z80::memory::Memory;
use Z80::z80;

  #[test]
  fn test_ld_ihl_n() {
      let mut cpu = z80::Z80::new(Memory::new_64k(TileDecoder::new(1)));
      cpu.set_sp(0xFFFF);
      cpu.set_pc(0x0000);
      let prog = [
          0x21, 0x00, 0x20,   // LD HL,0x2000
          0x36, 0x33,         // LD (HL),0x33
          0x21, 0x00, 0x10,   // LD HL,0x1000
          0x36, 0x65,         // LD (HL),0x65
      ];
      cpu.mem.write(0x0000, &prog);
      cpu.exec();
      assert!(0x2000 == cpu.hl);
      cpu.exec();
      assert!(0x33 == cpu.mem.r8(0x2000));
      cpu.exec();
      assert!(0x1000 == cpu.hl);
      cpu.exec();
      assert!(0x65 == cpu.mem.r8(0x1000));
  }

  #[test]
  fn test_ld_ihl() {
      let mut cpu = z80::Z80::new(Memory::new_64k(TileDecoder::new(1)));
      let prog = [
          0x77,       // LD (HL),A
          0x46,       // LD B,(HL)
          0x4E,       // LD C,(HL)
          0x56,       // LD D,(HL)
          0x5E,       // LD E,(HL)
          0x66,       // LD H,(HL)
      ];
      cpu.mem.write(0x0100, &prog);

      cpu.a = 0x33;
      cpu.hl = 0x1000;
      cpu.set_pc(0x0100);
      cpu.exec();
      assert_eq!(0x33, cpu.mem.r8(0x1000));
      cpu.exec();
      assert_eq!(0x33, cpu.b);
      cpu.exec();
      assert_eq!(0x33, cpu.c);
      cpu.exec();
      assert_eq!(0x33, cpu.d);
      cpu.exec();
      assert_eq!(0x33, cpu.e);
      cpu.exec();
      assert_eq!(0x33, cpu.get_hl_h());
  }

      #[test]
    fn test_add_r() {
        let mut cpu = z80::Z80::new(Memory::new_64k(TileDecoder::new(1)));
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
        cpu.mem.write(0x0000, &prog);
        cpu.exec();
        assert_eq!(0x0F, cpu.a); assert_eq!(cpu.flags, 0);
        cpu.exec();
        assert_eq!(0x1E, cpu.a); assert!(cpu.flags_get_h());
        cpu.exec();
        assert_eq!(0xE0, cpu.b as u8);
        cpu.exec();
        assert_eq!(0xFE, cpu.a as u8); assert!(cpu.flags_get_s());
        cpu.exec();
        assert_eq!(0x81, cpu.a as u8);
        cpu.exec();
        assert_eq!(0x80, cpu.c as u8);
        cpu.exec();
        assert_eq!(0x01, cpu.a); assert!(cpu.flags_get_c() | cpu.flags_get_pe());
        cpu.exec();
        assert_eq!(0xFF, cpu.d as u8);
        cpu.exec();
        assert_eq!(0x00, cpu.a); assert!(cpu.flags_get_z() | cpu.flags_get_h()| cpu.flags_get_c());
        cpu.exec();
        assert_eq!(0x40, cpu.e);
        cpu.exec();
        assert_eq!(0x40, cpu.a); assert_eq!(cpu.flags, 0);
        cpu.exec();
        assert_eq!(0x80, cpu.get_hl_h()as u8);
        cpu.exec();
        assert_eq!(0xC0, cpu.a as u8); assert!(cpu.flags_get_s());
        cpu.exec();
        assert_eq!(0x33, cpu.get_hl_l() as u8);
        cpu.exec();
        assert_eq!(0xF3, cpu.a as u8); assert!(cpu.flags_get_s());
        cpu.exec();
        assert_eq!(0x37, cpu.a as u8); assert!(cpu.flags_get_c());
    }

    #[test]
    fn test_add_r_2() {
        let mut cpu = z80::Z80::new(Memory::new_64k(TileDecoder::new(1)));
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
        cpu.mem.write(0x0000, &prog);
        cpu.exec();
        assert_eq!(0x0F, cpu.a); assert_eq!(cpu.flags, 0);
        cpu.exec();
        assert_eq!(0x1E, cpu.a); assert!(cpu.flags_get_h());
        cpu.exec();
        assert_eq!(0xE0, cpu.b as u8);
        cpu.exec();
        assert_eq!(0xFE, cpu.a as u8); assert!(cpu.flags_get_s());
        cpu.exec();
        assert_eq!(0x81, cpu.a as u8);
        cpu.exec();
        assert_eq!(0x80, cpu.c as u8);
        cpu.exec();
        assert_eq!(0x01, cpu.a); assert!(cpu.flags_get_c() | cpu.flags_get_pe());
        cpu.exec();
        assert_eq!(0xFF, cpu.d as u8);
        cpu.exec();
        assert_eq!(0x00, cpu.a); assert!(cpu.flags_get_z() | cpu.flags_get_c() | cpu.flags_get_h());
        cpu.exec();
        assert_eq!(0x40, cpu.e);
        cpu.exec();
        assert_eq!(0x40, cpu.a); assert_eq!(cpu.flags, 0);
        cpu.exec();
        assert_eq!(0x80, cpu.get_hl_h() as u8);
        cpu.exec();
        assert_eq!(0xC0, cpu.a as u8); assert!(cpu.flags_get_s());
        cpu.exec();
        assert_eq!(0x33, cpu.get_hl_l() as u8);
        cpu.exec();
        assert_eq!(0xF3, cpu.a as u8); assert!(cpu.flags_get_s());
        cpu.exec();
        assert_eq!(0x37, cpu.a as u8); assert!(cpu.flags_get_c());
    }

    #[test]
    fn test_call_cc_ret_cc() {
        let mut cpu = z80::Z80::new(Memory::new_64k(TileDecoder::new(1)));
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
		cpu.mem.write(0x0204, &prog);
		cpu.set_pc(0x0204);
		cpu.set_sp(0x0100);

        cpu.exec();
        assert_eq!(0x00, cpu.a);
        cpu.exec();
        assert_eq!(0x0208, cpu.pc);
        cpu.exec();
        assert_eq!(0x0229, cpu.pc);
        cpu.exec();
        assert_eq!(0x022A, cpu.pc);
        cpu.exec();
        assert_eq!(0x020B, cpu.pc);
        cpu.exec();
        assert_eq!(0x01, cpu.a);
        cpu.exec();
        assert_eq!(0x0210, cpu.pc);
        cpu.exec();
        assert_eq!(0x022B, cpu.pc);
        cpu.exec();
        assert_eq!(0x022C, cpu.pc);
        cpu.exec();
        assert_eq!(0x0213, cpu.pc);
        cpu.exec();
        assert_eq!(0x02, cpu.a);
        cpu.exec();
        assert_eq!(0x0217, cpu.pc);
        cpu.exec();
        assert_eq!(0x022D, cpu.pc);
        cpu.exec();
        assert_eq!(0x022E, cpu.pc);
        cpu.exec();
        assert_eq!(0x021A, cpu.pc);
        cpu.exec();
        assert_eq!(0xFF, cpu.a as u8);
        cpu.exec();
        assert_eq!(0x021F, cpu.pc);
        cpu.exec();
        assert_eq!(0x022F, cpu.pc);
        cpu.exec();
        assert_eq!(0x0230, cpu.pc);
        cpu.exec();
        assert_eq!(0x0222, cpu.pc);
        cpu.exec();
        assert_eq!(0x0225, cpu.pc);
        cpu.exec();
        assert_eq!(0x0231, cpu.pc);
        cpu.exec();
        assert_eq!(0x0232, cpu.pc);
        cpu.exec();
        assert_eq!(0x0228, cpu.pc);
    }

    #[test]
    fn test_call_ret() {
        let mut cpu = z80::Z80::new(Memory::new_64k(TileDecoder::new(1)));
        cpu.set_sp(0xFFFF);
        let prog = [
            0xCD, 0x0A, 0x02,   // CALL l0
            0xCD, 0x0A, 0x02,   // CALL l0
            0xC9,               // l0: RET
        ];
        cpu.mem.write(0x0204, &prog);
        cpu.set_pc(0x0204);
        cpu.exec();
        assert_eq!(0x020A, cpu.pc);
        // cpu.exec();
        assert_eq!(0xFFFD, cpu.sp);
        println!("Memory contents at 0xFFFD {}", format!("{:#x}", cpu.mem.r16(0xFFFD)));
        assert_eq!(0x0207, cpu.mem.r16(0xFFFD));
        cpu.exec();
        assert_eq!(0x0207, cpu.pc);
        assert_eq!(0xFFFF, cpu.sp);
        cpu.exec();
        assert_eq!(0x020A, cpu.pc);
        assert_eq!(0xFFFD, cpu.sp);
        cpu.exec();
        assert_eq!(0x020A, cpu.mem.r16(0xFFFD));
        assert_eq!(0x020A, cpu.pc);
        assert_eq!(0xFFFF, cpu.sp);
    }

 #[test]
    fn test_push_pop() {
        let mut cpu = z80::Z80::new(Memory::new_64k(TileDecoder::new(1)));
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
        cpu.mem.write(0x0000, &prog);

        cpu.exec();
        assert_eq!(0x1234, cpu.bc());
        cpu.exec();
        assert_eq!(0x5678, cpu.de());
        cpu.exec();
        assert_eq!(0x9ABC, cpu.hl);
        cpu.exec();
        assert_eq!(0xEF00, cpu.af());
        cpu.exec();
        assert_eq!(0x2345, cpu.ix);
        cpu.exec();
        assert_eq!(0x6789, cpu.iy);
        cpu.exec();
        assert_eq!(0x0100, cpu.sp);
        cpu.exec();
        assert_eq!(0xEF00, cpu.mem.r16(0x00FE)); assert_eq!(0x00FE, cpu.sp);
        cpu.exec();
        assert_eq!(0x1234, cpu.mem.r16(0x00FC)); assert_eq!(0x00FC, cpu.sp);
        cpu.exec();
        assert_eq!(0x5678, cpu.mem.r16(0x00FA)); assert_eq!(0x00FA, cpu.sp);
        cpu.exec();
        assert_eq!(0x9ABC, cpu.mem.r16(0x00F8)); assert_eq!(0x00F8, cpu.sp);
        cpu.exec();
        assert_eq!(0x2345, cpu.mem.r16(0x00F6)); assert_eq!(0x00F6, cpu.sp);
        cpu.exec();
        assert_eq!(0x6789, cpu.mem.r16(0x00F4)); assert_eq!(0x00F4, cpu.sp);
        cpu.exec();
        assert_eq!(0x6789, cpu.af()); assert_eq!(0x00F6, cpu.sp);
        cpu.exec();
        assert_eq!(0x2345, cpu.bc()); assert_eq!(0x00F8, cpu.sp);
        cpu.exec();
        assert_eq!(0x9ABC, cpu.de()); assert_eq!(0x00FA, cpu.sp);
        cpu.exec();
        assert_eq!(0x5678, cpu.hl); assert_eq!(0x00FC, cpu.sp);
        cpu.exec();
        assert_eq!(0x1234, cpu.ix); assert_eq!(0x00FE, cpu.sp);
        cpu.exec();
        assert_eq!(0xEF00, cpu.iy); assert_eq!(0x0100, cpu.sp);
    }

    #[test]
    fn test_cp_r() {
        let mut cpu = z80::Z80::new(Memory::new_64k(TileDecoder::new(1)));
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
        cpu.mem.write(0x0000, &prog);
        cpu.exec();
        assert_eq!(0x04, cpu.a);
        cpu.exec();
        assert_eq!(0x05, cpu.b);
        cpu.exec();
        assert_eq!(0x03, cpu.c);
        cpu.exec();
        assert_eq!(0xff, cpu.d as u8);
        cpu.exec();
        assert_eq!(0xaa, cpu.e as u8);
        cpu.exec();
        assert_eq!(0x80, cpu.get_hl_h() as u8);
        cpu.exec();
        assert_eq!(0x7f, cpu.get_hl_l());
        cpu.exec();
        assert_eq!(0x04, cpu.a); assert!(cpu.flags_get_z() && cpu.flags_get_n());
        cpu.exec();
        assert_eq!(0x04, cpu.a); assert!(cpu.flags_get_c() && cpu.flags_get_s() && cpu.flags_get_n());
        cpu.exec();
        assert_eq!(0x04, cpu.a); assert!(cpu.flags_get_n());
        cpu.exec();
        assert_eq!(0x04, cpu.a); assert!(cpu.flags_get_c());
        // assert_eq!(0x04, cpu.a); assert!(flags(&cpu, SF|HF|NF|CF));
        // assert_eq!(0x04, cpu.a); assert!(flags(&cpu, HF|NF|CF));
        // assert_eq!(0x04, cpu.a); assert!(flags(&cpu, HF|NF|CF));
        // assert_eq!(0x04, cpu.a); assert!(flags(&cpu, SF|VF|NF|CF));
        // assert_eq!(0x04, cpu.a); assert!(flags(&cpu, SF|HF|NF|CF));
        // assert_eq!(0x04, cpu.a); assert!(flags(&cpu, ZF|NF));
    }

    #[test]
    fn test_jr_cc() {
        let mut cpu = z80::Z80::new(Memory::new_64k(TileDecoder::new(1)));
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
        cpu.mem.write(0x204, &prog);
        cpu.set_pc(0x0204);
        cpu.exec();
        assert_eq!(0x00, cpu.a);
        cpu.exec();
        assert_eq!(0x0207, cpu.pc());
        cpu.exec();
        assert_eq!(0x020A, cpu.pc());
        cpu.exec();
        assert_eq!(0x01, cpu.a);
        cpu.exec();
        assert_eq!(0x020E, cpu.pc());
        cpu.exec();
        assert_eq!(0x0211, cpu.pc());
        cpu.exec();
        assert_eq!(0xFE, cpu.a as u8);
        cpu.exec();
        assert_eq!(0x0215, cpu.pc());
        cpu.exec();
        assert_eq!(0x0218, cpu.pc());
    }

    #[test]
    fn test_djnz() {
        let mut cpu = z80::Z80::new(Memory::new_64k(TileDecoder::new(1)));
        let prog = [
            0x06, 0x03,     // LD BC,0x03
            0x97,           // SUB A
            0x3C,           // loop: INC A
            0x10, 0xFD,     // DJNZ loop
            0x00,           // NOP
        ];
        cpu.mem.write(0x0204, &prog);
        cpu.set_pc(0x0204);
        cpu.exec();
        assert_eq!(0x03, cpu.b);
        cpu.exec();
        assert_eq!(0x00, cpu.a);
        cpu.exec();
        assert_eq!(0x01, cpu.a);
        cpu.exec();
        assert_eq!(0x02, cpu.b); assert_eq!(0x0207, cpu.pc());
        cpu.exec();
        assert_eq!(0x02, cpu.a);
        cpu.exec();
        assert_eq!(0x01, cpu.b); assert_eq!(0x0207, cpu.pc());
        cpu.exec();
        assert_eq!(0x03, cpu.a);
        cpu.exec();
        assert_eq!(0x00, cpu.b); assert_eq!(0x020A, cpu.pc());
    }

    #[test]
    fn test_inc_dec_r() {
        let mut cpu = z80::Z80::new(Memory::new_64k(TileDecoder::new(1)));
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
        cpu.mem.write(0x0000, &prog);

        for _ in 0..8 {
            cpu.exec();
        }

        assert_eq!(0x01, cpu.a); assert!(cpu.flags == 0);
        cpu.exec();
        assert_eq!(0x00, cpu.a); assert!(cpu.flags_get_z() && cpu.flags_get_n());
        cpu.exec();
        assert_eq!(0x00, cpu.b); assert!(cpu.flags_get_z() & cpu.flags_get_h());
        cpu.exec();
        assert_eq!(0xFF, cpu.b as u8); assert!(cpu.flags_get_n() & cpu.flags_get_h() & cpu.flags_get_s());
        cpu.exec();
        assert_eq!(0x10, cpu.c); assert!(cpu.flags_get_h());
        cpu.exec();
        assert_eq!(0x0F, cpu.c); assert!(cpu.flags_get_n() & cpu.flags_get_h());
        cpu.exec();
        assert_eq!(0x0F, cpu.d); assert!(cpu.flags == 0);
        cpu.exec();
        assert_eq!(0x0E, cpu.d); assert!(cpu.flags_get_n());
        cpu.exec();
        assert_eq!(0x00, cpu.a); 
        assert!(cpu.flags_get_n() & cpu.flags_get_s() & cpu.flags_get_c());
        cpu.exec();
        assert_eq!(0x80 as u8, cpu.e as u8); assert!(cpu.flags_get_s() & cpu.flags_get_c());
        cpu.exec();
        assert_eq!(0x7F, cpu.e); assert!(cpu.flags_get_n() & cpu.flags_get_h() & cpu.flags_get_c());
        cpu.exec();
        assert_eq!(0x3F, cpu.get_hl_h()); assert!(cpu.flags_get_c());
        cpu.exec();
        assert_eq!(0x3E, cpu.get_hl_h()); assert!(cpu.flags_get_n() & cpu.flags_get_c());
        cpu.exec();
        assert_eq!(0x24, cpu.get_hl_l()); assert!(cpu.flags_get_c());
        cpu.exec();
        assert_eq!(0x23, cpu.get_hl_l()); assert!(cpu.flags_get_n() & cpu.flags_get_c());        
    }

    #[test]
    fn test_adc_r() {
        let mut cpu = z80::Z80::new(Memory::new_64k(TileDecoder::new(1)));
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
        cpu.mem.write(0x0000, &prog);
        cpu.exec();
        assert_eq!(0x00, cpu.a);
        cpu.exec();
        assert_eq!(0x41, cpu.b);
        cpu.exec();
        assert_eq!(0x61, cpu.c);
        cpu.exec();
        assert_eq!(0x81, cpu.d as u8);
        cpu.exec();
        assert_eq!(0x41, cpu.e);
        cpu.exec();
        assert_eq!(0x61, cpu.get_hl_h());
        cpu.exec();
        assert_eq!(0x81 as u8, cpu.get_hl_l() as u8);
        cpu.exec();
        assert_eq!(0x00, cpu.a); assert!(cpu.flags_get_z());
        cpu.exec();
        assert_eq!(0x41, cpu.a); assert_eq!(cpu.flags, 0);
        cpu.exec();
        assert_eq!(0xA2, cpu.a as u8); assert!(cpu.flags_get_s() && cpu.flags_get_pe());
        cpu.exec();
        assert_eq!(0x23, cpu.a); assert!(cpu.flags_get_c());
        cpu.exec();
        assert_eq!(0x65, cpu.a); assert_eq!(cpu.flags, 0);
        cpu.exec();
        assert_eq!(0xC6, cpu.a as u8); assert!(cpu.flags_get_s() && cpu.flags_get_pe());
        cpu.exec();
        assert_eq!(0x47, cpu.a); assert!(cpu.flags_get_c());
        cpu.exec();
        assert_eq!(0x49, cpu.a); assert_eq!(cpu.flags, 0);
    }
}
