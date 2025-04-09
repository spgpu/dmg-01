/// CPU Registers
#[derive(Debug, Default)]
pub struct Registers {
    pub a: u8, // Accumulator
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub pc: u16,
    pub sp: u16,
    pub f: FlagsRegister,
}

impl Registers {
    /// Initialize Registers
    pub fn new() -> Self {
        Self::default()
    }

    // Getters
    // Turn both bytes into u16 (0x00AB)
    // Shift the upper byte to the left (0xAB00)
    // Combine with OR (0xABAB)

    /// Get bc register
    pub fn bc(&self) -> u16 {
        (self.b as u16) << 8 | (self.c as u16)
    }

    /// Get de register
    pub fn de(&self) -> u16 {
        (self.d as u16) << 8 | (self.e as u16)
    }

    /// Get hl register
    pub fn hl(&self) -> u16 {
        (self.h as u16) << 8 | (self.l as u16)
    }

    // Setters
    // Shift upper byte to the right (0x00AB) and convert
    // Convert lower byte to u8 and store as is (0xAB)

    /// Set bc register
    pub fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = value as u8;
    }

    /// Set de register
    pub fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = value as u8;
    }

    /// Set hl register
    pub fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = value as u8;
    }
}

/// Representation of the upper byte in Flags register
#[derive(Debug, Default, Copy, Clone)]
pub struct FlagsRegister {
    pub z: bool, // Zero
    pub n: bool, // Subtract
    pub h: bool, // Half Carry
    pub c: bool, // Carry
}

impl FlagsRegister {
    /// Initialize FlagsRegister
    pub fn new() -> Self {
        Self::default()
    }
}

/// Main CPU struct
#[derive(Debug)]
pub struct Cpu {
    pub registers: Registers,
}

impl Cpu {
    /// Initialize CPU
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
        }
    }
}
