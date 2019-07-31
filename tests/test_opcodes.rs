#[allow(unused_imports)]
// use z80::Z80;

mod test_opcodes {
  use Z80::z80;
  // use super::z80::Z80;

  #[test]
  fn test_ld_ihl_n() {
      let mut cpu = z80::Z80::new();
      let prog = [
          0x21, 0x00, 0x20,   // LD HL,0x2000
          0x36, 0x33,         // LD (HL),0x33
          0x21, 0x00, 0x10,   // LD HL,0x1000
          0x36, 0x65,         // LD (HL),0x65
      ];
      cpu.mem.write(0x0000, &prog);

      cpu.step();
      assert!(0x2000 == cpu.HL);
      cpu.step();
      assert!(0x33 == cpu.mem.r8(0x2000));
      cpu.step();
      assert!(0x1000 == cpu.HL);
      cpu.step();
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
      assert_eq!(0x33, cpu.mem.r8(0x1000));
      assert_eq!(0x33, cpu.b);
      assert_eq!(0x33, cpu.c);
      assert_eq!(0x33, cpu.d);
      assert_eq!(0x33, cpu.e);
      assert_eq!(0x33, cpu.get_HL_H());
  }
}

