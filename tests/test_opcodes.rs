#[allow(unused_imports)]

mod test_opcodes {
  use Z80::z80;

  #[test]
  fn test_ld_ihl_n() {
      let mut cpu = z80::Z80::new();
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
      assert!(0x2000 == cpu.HL);
      cpu.exec();
      assert!(0x33 == cpu.mem.r8(0x2000));
      cpu.exec();
      assert!(0x1000 == cpu.HL);
      cpu.exec();
      assert!(0x65 == cpu.mem.r8(0x1000));
  }

  #[test]
  fn test_ld_ihl() {
      let mut cpu = z80::Z80::new();
      let prog = [
          0x77,       // LD (HL),A
          0x46,       // LD B,(HL)
          0x4E,       // LD C,(HL)
          0x56,       // LD D,(HL)
          0x5E,       // LD E,(HL)
          0x66,       // LD H,(HL)
      ];
      cpu.mem.write(0x0100, &prog);

      cpu.set_A(0x33);
      cpu.set_HL(0x1000);
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
      assert_eq!(0x33, cpu.get_HL_H());
  }
}

