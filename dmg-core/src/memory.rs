/// Holds the different memory regions
#[derive(Debug, Clone, Copy)]
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
}
