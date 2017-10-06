
// Flags poisition in the F register
const ZERO_FLAG: u8 = 0x80;
const OPERATION_FLAG: u8 = 0x40;
const HALF_CARRY_FLAG: u8 = 0x20;
const CARRY_FLAG: u8 = 0x10;


struct Clocks {
    m: i32, t: i32  // TODO: check if i32 is the right type
}

impl Clocks {
    fn new() -> Clocks {
        Clocks { m: 0, t: 0 }
    }
}

struct Registers {
    a: u8, b: u8, c: u8, d: u8,
    e: u8, h: u8, l: u8, f: u8,

    pc: u16, sp: u16,
    m: u8, t: u8
}

impl Registers {
    fn new() -> Registers {
        Registers {
            a: 0, b: 0, c: 0, d: 0,
            e: 0, h: 0, l:0, f: 0,

            pc: 0, sp: 0,
            m: 0, t: 0
        }
    }
}

pub struct CPU {
    clks: Clocks,
    regs: Registers
}

impl CPU {
    pub fn new() -> CPU {
        CPU { clks: Clocks::new(), regs: Registers::new() }
    }

    // adds E to A
    fn addr_e(&mut self) {
        let result = self.regs.a as u32 + self.regs.e as u32;

        self.regs.f = 0; // reset the flags!

        // Zero
        if (result & 0xFF) == 0 {
            self.regs.f |= ZERO_FLAG; // if result is 0 set the first bit to 1
        }

        // Half Carry
//        if ((self.regs.a & 0xF) + (self.regs.e & 0xF)) & 0x10 {
//            self.regs.f |= HALF_CARRY_FLAG;
//        }

        // Carry
        if result > 0xFF {
            self.regs.f |= CARRY_FLAG;
        }

        // save it in the A register
        self.regs.a = (result & 0xFF) as u8;

        self.regs.m = 1;
        self.regs.t = 4;
    }

    // no operation
    fn nop(&mut self) {
        self.regs.m = 1;
        self.regs.t = 4;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cpu_inizialization() {
        let CPU { clks, regs } = CPU::new();

        assert_eq!(clks.m, 0);
        assert_eq!(clks.t, 0);

        assert_eq!(regs.a, 0);
        assert_eq!(regs.b, 0);
        assert_eq!(regs.c, 0);
        assert_eq!(regs.d, 0);
        assert_eq!(regs.e, 0);
        assert_eq!(regs.h, 0);
        assert_eq!(regs.l, 0);
        assert_eq!(regs.f, 0);
        assert_eq!(regs.pc, 0);
        assert_eq!(regs.sp, 0);
        assert_eq!(regs.m, 0);
        assert_eq!(regs.t, 0);
    }

    #[test]
    fn op_nop() {
        let mut cpu = CPU::new();

        cpu.nop();

        assert_eq!(cpu.regs.m, 1);
        assert_eq!(cpu.regs.t, 4);
    }

    #[test]
    fn op_addr_e() {
        let mut cpu = CPU::new();

        cpu.regs.e = 0xFF;

        cpu.addr_e();

        assert_eq!(cpu.regs.a, 0xFF);
        assert_eq!(cpu.regs.f, 0);

        assert_eq!(cpu.regs.m, 1);
        assert_eq!(cpu.regs.t, 4);
    }

    #[test]
    fn op_addr_e_carry() {
        let mut cpu = CPU::new();

        cpu.regs.a = 0x01;
        cpu.regs.e = 0xFF;

        cpu.addr_e();

        assert_eq!(cpu.regs.a, 0);
        assert_eq!(cpu.regs.f, ZERO_FLAG | CARRY_FLAG);

        assert_eq!(cpu.regs.m, 1);
        assert_eq!(cpu.regs.t, 4);
    }
}