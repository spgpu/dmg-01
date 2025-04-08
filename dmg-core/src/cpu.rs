use crate::memory::Memory;

pub struct Cpu {
    a: u8, // Accumulator
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    f: u8,   // Flags
    sp: u16, // Stack Pointer
    pc: u16, // Program Counter
}

enum Flag {
    Zero = 7,
    Negative = 6,
    HalfCarry = 5,
    Carry = 4,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            f: 0,
            sp: 0,
            pc: 0,
        }
    }

    fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8; // High byte
        self.l = value as u8; // Low byte
    }

    fn set_flag(&mut self, flag: Flag, value: bool) {
        let bit = flag as u8;
        if value {
            self.f |= 1 << bit;
        } else {
            self.f &= !(1 << bit);
        }
    }

    pub fn step(&mut self, memory: &mut dyn Memory) {
        let opcode = memory.read(self.pc);
        self.pc += 1;
        match opcode {
            0x00 => self.nop(),
            0x01 => self.ld_bc_d16(memory),
            0x02 => self.ld_bc_a(memory),
            0x03 => self.inc_bc(),
            _ => panic!("Unknown opcode: {:02X}", opcode),
        }
    }

    fn nop(&self) {
        // Do nothing
    }

    fn ld_bc_d16(&mut self, memory: &mut dyn Memory) {
        let low = memory.read(self.pc);
        self.pc += 1;
        let high = memory.read(self.pc);
        self.pc += 1;
        self.set_bc(((high as u16) << 8) | (low as u16));
    }

    fn ld_bc_a(&mut self, memory: &mut dyn Memory) {
        let bc = self.get_bc();
        memory.write(bc, self.a);
    }

    fn inc_bc(&mut self) {
        let bc = self.get_bc().wrapping_add(1);
        self.set_bc(bc);
    }

    // Helper methods for BC
    fn get_bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }

    fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = value as u8;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::SimpleMemory;

    #[test]
    fn test_ld_bc_d16() {
        let mut cpu = Cpu::new();
        let mut mem = SimpleMemory::new();
        mem.write(0, 0x01); // LD BC, d16
        mem.write(1, 0x34); // Low byte
        mem.write(2, 0x12); // High byte
        cpu.step(&mut mem);
        assert_eq!(cpu.get_bc(), 0x1234);
        assert_eq!(cpu.pc, 3);
    }
}
