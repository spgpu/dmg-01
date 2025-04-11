/// Holds the different memory regions
#[derive(Debug, Clone)]
pub struct MemoryBus {
    // See https://gbdev.io/pandocs/Memory_Map.html
    // Ignores unused / GBC regions
    rom_bank_00: [u8; 16 * 1024],
    rom_bank_nn: [u8; 16 * 1024],
    vram: [u8; 8 * 1024],
    ext_ram: [u8; 8 * 1024],
    wram_0: [u8; 4 * 1024],
    wram_n: [u8; 4 * 1024],
    oam: [u8; 160],
    io_registers: [u8; 128],
    hram: [u8; 127],
    interrupt: u8,
}

impl MemoryBus {
    /// Initialize memory bus
    pub fn new() -> Self {
        Self {
            rom_bank_00: [0; 16 * 1024],
            rom_bank_nn: [0; 16 * 1024],
            vram: [0; 8 * 1024],
            ext_ram: [0; 8 * 1024],
            wram_0: [0; 4 * 1024],
            wram_n: [0; 4 * 1024],
            oam: [0; 160],
            io_registers: [0; 128],
            hram: [0; 127],
            interrupt: 0,
        }
    }

    /// Reads the byte at address given
    pub fn read_byte(&self, address: u16) -> u8 {
        let a = address as usize;
        match address {
            0x0000..=0x3FFF => self.rom_bank_00[a],
            0x4000..=0x7FFF => self.rom_bank_nn[a - 0x4000],
            0x8000..=0x9FFF => self.vram[a - 0x8000],
            0xA000..=0xBFFF => self.ext_ram[a - 0xA000],
            0xC000..=0xCFFF => self.wram_0[a - 0xC000],
            0xD000..=0xDFFF => self.wram_n[a - 0xD000],
            0xE000..=0xEFFF => self.wram_0[a - 0xE000], // Echo ram for wram_0
            0xF000..=0xFDFF => self.wram_n[a - 0xF000], // Echo ram for wram_n
            0xFE00..=0xFE9F => self.oam[a - 0xFE00],
            0xFF00..=0xFF7F => self.io_registers[a - 0xFF00],
            0xFF80..=0xFFFE => self.hram[a - 0xFF80],
            0xFFFF => self.interrupt,
            _ => 0xFF,
        }
    }

    /// Writes byte at address if writable
    pub fn write_byte(&mut self, address: u16, value: u8) {
        let a = address as usize;
        let byte: &mut u8 = match address {
            0x8000..=0x9FFF => &mut self.vram[a - 0x8000],
            0xA000..=0xBFFF => &mut self.ext_ram[a - 0xA000],
            0xC000..=0xCFFF => &mut self.wram_0[a - 0xC000],
            0xD000..=0xDFFF => &mut self.wram_n[a - 0xD000],
            0xE000..=0xEFFF => &mut self.wram_0[a - 0xE000],
            0xF000..=0xFDFF => &mut self.wram_n[a - 0xF000],
            0xFE00..=0xFE9F => &mut self.oam[a - 0xFE00],
            0xFF00..=0xFF7F => &mut self.io_registers[a - 0xFF00],
            0xFF80..=0xFFFE => &mut self.hram[a - 0xFF80],
            0xFFFF => &mut self.interrupt,
            _ => return,
        };
        *byte = value;
    }
}
