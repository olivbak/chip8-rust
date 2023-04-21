pub const VRAM_WIDTH: usize = 64;
pub const VRAM_HEIGHT: usize = 32;

#[derive(Copy, Clone)]
pub struct Vram {
    pub vram: [[u8; VRAM_WIDTH]; VRAM_HEIGHT],
}

impl Vram {
    pub fn init() -> Vram {
	let vram = [[0u8; VRAM_WIDTH]; VRAM_HEIGHT];
	return Vram{
	    vram: vram,
	}
    }

    pub fn set(&mut self, x: usize, y: usize, val: u8) -> () {
	self.vram[x][y] = val;
    }

    pub fn fetch(&mut self, x: usize, y: usize) -> u8 {
	return self.vram[x][y];
    }
}
