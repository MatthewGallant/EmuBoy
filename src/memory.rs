pub struct Memory {
    pub memory: Vec<u8>
}

impl Memory {
    pub fn new() -> Self {
        let mut memory = Memory {
            memory: vec![0; 65536]
        };

        // Initialize I/O register memory addresses on start
        // Below excludes addresses that are set to 0x00 since that's already done up above.
        // See page 18 of http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf for more info.
        memory.set_byte(0x80, 0xFF10); // NR10
        memory.set_byte(0xBF, 0xFF11); // NR11
        memory.set_byte(0xF3, 0xFF12); // NR12
        memory.set_byte(0xBF, 0xFF14); // NR14
        memory.set_byte(0x3F, 0xFF16); // NR21
        memory.set_byte(0xBF, 0xFF19); // NR24
        memory.set_byte(0x7F, 0xFF1A); // NR30
        memory.set_byte(0xFF, 0xFF1B); // NR31
        memory.set_byte(0x9F, 0xFF1C); // NR32
        memory.set_byte(0xBF, 0xFF1E); // NR33
        memory.set_byte(0xFF, 0xFF20); // NR41
        memory.set_byte(0xBF, 0xFF23); // NR30
        memory.set_byte(0x77, 0xFF24); // NR50
        memory.set_byte(0xF3, 0xFF25); // NR51
        memory.set_byte(0xF1, 0xFF26); // NR52
        memory.set_byte(0x91, 0xFF40); // LCDC
        memory.set_byte(0xFC, 0xFF47); // BGP
        memory.set_byte(0xFF, 0xFF48); // OBP0
        memory.set_byte(0xFF, 0xFF49); // OBP1

        // Add SameBoy DMG Boot ROM - https://github.com/LIJI32/SameBoy
        memory.set_bytes(vec![0x31, 0xFE, 0xFF, 0x21, 0x00, 0x80, 0x22, 0xCB, 0x6C, 0x28, 0xFB, 0x3E, 0x80, 0xE0, 0x26, 0xE0, 0x11, 0x3E, 0xF3, 0xE0, 0x12, 0xE0, 0x25, 0x3E, 0x77, 0xE0, 0x24, 0x3E, 0x54, 0xE0, 0x47, 0x11, 0x04, 0x01, 0x21, 0x10, 0x80, 0x1A, 0x47, 0xCD, 0xA2, 0x00, 0xCD, 0xA2, 0x00, 0x13, 0x7B, 0xEE, 0x34, 0x20, 0xF2, 0x11, 0xD1, 0x00, 0x0E, 0x08, 0x1A, 0x13, 0x22, 0x23, 0x0D, 0x20, 0xF9, 0x3E, 0x19, 0xEA, 0x10, 0x99, 0x21, 0x2F, 0x99, 0x0E, 0x0C, 0x3D, 0x28, 0x08, 0x32, 0x0D, 0x20, 0xF9, 0x2E, 0x0F, 0x18, 0xF5, 0x3E, 0x1E, 0xE0, 0x42, 0x3E, 0x91, 0xE0, 0x40, 0x16, 0x89, 0x0E, 0x0F, 0xCD, 0xB7, 0x00, 0x7A, 0xCB, 0x2F, 0xCB, 0x2F, 0xE0, 0x42, 0x7A, 0x81, 0x57, 0x79, 0xFE, 0x08, 0x20, 0x04, 0x3E, 0xA8, 0xE0, 0x47, 0x0D, 0x20, 0xE7, 0x3E, 0xFC, 0xE0, 0x47, 0x3E, 0x83, 0xCD, 0xCA, 0x00, 0x06, 0x05, 0xCD, 0xC3, 0x00, 0x3E, 0xC1, 0xCD, 0xCA, 0x00, 0x06, 0x3C, 0xCD, 0xC3, 0x00, 0x21, 0xB0, 0x01, 0xE5, 0xF1, 0x21, 0x4D, 0x01, 0x01, 0x13, 0x00, 0x11, 0xD8, 0x00, 0xC3, 0xFE, 0x00, 0x3E, 0x04, 0x0E, 0x00, 0xCB, 0x20, 0xF5, 0xCB, 0x11, 0xF1, 0xCB, 0x11, 0x3D, 0x20, 0xF5, 0x79, 0x22, 0x23, 0x22, 0x23, 0xC9, 0xE5, 0x21, 0x0F, 0xFF, 0xCB, 0x86, 0xCB, 0x46, 0x28, 0xFC, 0xE1, 0xC9, 0xCD, 0xB7, 0x00, 0x05, 0x20, 0xFA, 0xC9, 0xE0, 0x13, 0x3E, 0x87, 0xE0, 0x14, 0xC9, 0x3C, 0x42, 0xB9, 0xA5, 0xB9, 0xA5, 0x42, 0x3C, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xE0, 0x50], 0x0000);

        return memory;
    }

    pub fn byte(&self, address: u16) -> &u8 {
        return &self.memory[address as usize];
    }

    pub fn bytes(&self, start_address: u16, end_address: u16) -> &[u8] {
        return &self.memory[start_address as usize..end_address as usize];
    }

    pub fn set_byte(&mut self, data: u8, address: u16) {
        self.memory[address as usize] = data;
    }

    pub fn set_bytes(&mut self, data: Vec<u8>, address: u16) {
        self.memory.splice(address as usize..address as usize + data.len(), data);
    }

    pub fn read16(&self, address: u16) -> u16 {
        (self.memory[address as usize] as u16) | ((self.memory[(address + 1) as usize] as u16) << 8)
    }

    pub fn write16(&mut self, data: u16, address: u16) {
        self.set_byte((data & 0xFF) as u8, address);
        self.set_byte((data >> 8) as u8, address + 1);
    }
}
