const RAM_SIZE: usize = 4096;

pub struct Memory {
    pub ram: [u8; RAM_SIZE],
}

impl Memory {
    pub fn init() -> Memory {
	let ram = [0u8; RAM_SIZE];
	return Memory{
	    ram: ram,
	}
    }

    pub fn set(&mut self, index: usize, value: u8) -> () {
	self.ram[index] = value;
    }

    pub fn fetch(&mut self, index: usize) -> u8 {
	return self.ram[index];
    }

    pub fn fetch_opcode(&mut self, index: usize) -> u16 {
	 (self.ram[index] as u16) << 8 | (self.ram[index + 1] as u16)
    }

    pub fn load_cartridge(&mut self, data: &[u8]) {
	for (i, &byte) in data.iter().enumerate() {
            let addr = 0x200 + i;
            if addr < 4096 {
                self.set(0x200 + i, byte);
            } else {
                break;
            }
        }
    }

    pub fn load_font_set(&mut self, font_set: &[u8]) {
	for i in 0..font_set.len() {
            self.set(i, font_set[i]);
        }    
    }
}
