pub trait Memory {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8);
}

pub struct SimpleMemory {
    data: [u8; 0x10000], // 64KB
}

impl SimpleMemory {
    pub fn new() -> Self {
        SimpleMemory { data: [0; 0x10000] }
    }
}

impl Memory for SimpleMemory {
    fn read(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    fn write(&mut self, address: u16, value: u8) {
        self.data[address as usize] = value;
    }
}
