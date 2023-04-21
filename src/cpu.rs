extern crate rand;

use rand::Rng;
use crate::memory::Memory;
use crate::vram::Vram;

enum PCAction {
    Forward(usize), // forwards by X opcodes
    Set(usize) // sets memory location
}

pub struct CPU <'a>{
    vram: &'a mut Vram,
    ram: &'a mut Memory,
    stack: [usize; 16],
    i: usize,
    sp: usize,
    v: [u8; 16],
    pc: usize,
    delay_timer: u8,
    sound_timer: u8,
    keymap: [bool; 16]
}

impl<'a> CPU <'a>{

    pub fn init(mem : &'a mut Memory, vram : &'a mut Vram) -> CPU <'a>{
	CPU {
	    vram: vram,
	    ram: mem,
	    stack: [0;16],
	    i: 0,
	    sp: 0,
	    pc: 0x200,
	    v: [0;16],
	    delay_timer: 0,
	    sound_timer: 0,
	    keymap: [false;16]
	}
    }

    pub fn fetch_vram(&mut self) -> Vram {
	return *self.vram;
    }

    pub fn fetch_opcode(&mut self) -> u16 {
	return Memory::fetch_opcode(&mut self.ram, self.pc);
    }
    pub fn set_keymap(&mut self, keymap: [bool;16]) -> () {
	self.keymap = keymap;
    }

    pub fn decrement_timers(&mut self) -> () {
	self.delay_timer =
	    if self.delay_timer > 0 { self.delay_timer - 1} else { 0 };
	self.sound_timer =
	    if self.sound_timer > 0 { self.sound_timer - 1} else { 0 };
    }

    pub fn execute_opcode(&mut self, opcode: u16) -> () {
	let opcode_half_bytes = (
            (opcode & 0xF000) >> 12 as u8,
            (opcode & 0x0F00) >> 8 as u8,
            (opcode & 0x00F0) >> 4 as u8,
            (opcode & 0x000F) as u8,
        );
        let nnn = (opcode & 0x0FFF) as usize;
        let nn = (opcode & 0x00FF) as u8;
        let x = opcode_half_bytes.1 as usize;
        let y = opcode_half_bytes.2 as usize;
        let n = opcode_half_bytes.3 as usize;

	match opcode_half_bytes {
	    (a,b,c,d) => println!("opcode: {}, {}, {}, {}\n pc: {}", a,b,c,d, self.pc)
	}

	let delta_pc = match opcode_half_bytes {
	    (0x00, 0x00, 0x0e, 0x00) => self.inst_00e0(),
	    (0x00, 0x00, 0x0e, 0x0e) => self.inst_00ee(),
	    (0x01, _, _, _) => self.inst_1nnn(nnn),
            (0x02, _, _, _) => self.inst_2nnn(nnn),
            (0x03, _, _, _) => self.inst_3xnn(x, nn),
            (0x04, _, _, _) => self.inst_4xnn(x, nn),
            (0x05, _, _, 0x00) => self.inst_5xy0(x, y),
            (0x06, _, _, _) => self.inst_6xnn(x, nn),
            (0x07, _, _, _) => self.inst_7xnn(x, nn),
            (0x08, _, _, 0x00) => self.inst_8xy0(x, y),
            (0x08, _, _, 0x01) => self.inst_8xy1(x, y),
            (0x08, _, _, 0x02) => self.inst_8xy2(x, y),
            (0x08, _, _, 0x03) => self.inst_8xy3(x, y),
            (0x08, _, _, 0x04) => self.inst_8xy4(x, y),
            (0x08, _, _, 0x05) => self.inst_8xy5(x, y),
            (0x08, _, _, 0x06) => self.inst_8xy6(x, y),
            (0x08, _, _, 0x07) => self.inst_8xy7(x, y),
            (0x08, _, _, 0x0e) => self.inst_8xye(x, y),
            (0x09, _, _, 0x00) => self.inst_9xy0(x, y),
            (0x0a, _, _, _) => self.inst_annn(nnn),
            (0x0b, _, _, _) => self.inst_bnnn(nnn),
            (0x0c, _, _, _) => self.inst_cxnn(x, nn),
            (0x0d, _, _, _) => self.inst_dxyn(x, y, n),
            (0x0e, _, 0x09, 0x0e) => self.inst_ex9e(x),
            (0x0e, _, 0x0a, 0x01) => self.inst_exa1(x),
            (0x0f, _, 0x00, 0x07) => self.inst_fx07(x),
            (0x0f, _, 0x00, 0x0a) => self.inst_fx0a(x),
            (0x0f, _, 0x01, 0x05) => self.inst_fx15(x),
            (0x0f, _, 0x01, 0x08) => self.inst_fx18(x),
            (0x0f, _, 0x01, 0x0e) => self.inst_fx1e(x),
            (0x0f, _, 0x02, 0x09) => self.inst_fx29(x),
            (0x0f, _, 0x03, 0x03) => self.inst_fx33(x),
            (0x0f, _, 0x05, 0x05) => self.inst_fx55(x),
            (0x0f, _, 0x06, 0x05) => self.inst_fx65(x),
	    (_, _, _, _) => {
		println!("skip");
		PCAction::Forward(1)
	    }
	};

	self.pc = match delta_pc {
	    PCAction::Set(x) => x,
	    PCAction::Forward(x) => self.pc + x*2
	}

    }

    //OPCODE: clear display
    fn inst_00e0(&mut self) -> PCAction {
	*self.vram = Vram::init();
	return PCAction::Forward(1)
    }

    //RETURN
    fn inst_00ee(&mut self) -> PCAction {
	self.sp -= 1;
	return PCAction::Set(self.stack[self.sp]);
    }

    //JUMP
    fn inst_1nnn(&mut self, nnn: usize) -> PCAction {
	return PCAction::Set(nnn);
    }

    //CALL
    fn inst_2nnn(&mut self, nnn: usize) -> PCAction {
	self.stack[self.sp] = self.pc+2;
	self.sp += 1;
	return PCAction::Set(nnn);
    }

    // SKIP next instruction if V[x] == NN
    fn inst_3xnn(&mut self, x: usize, nn: u8) -> PCAction {
	return if self.v[x] == nn {
	    PCAction::Forward(2)
	} else {
	    PCAction::Forward(1)
	}
    }

    // SKIP next instruction if V[x] != NN
    fn inst_4xnn(&mut self, x: usize, nn: u8) -> PCAction {
	return if self.v[x] != nn {
	    PCAction::Forward(2)
	} else {
	    PCAction::Forward(1)
	}
    }

    // SKIP next instruction if V[x] != NN
    fn inst_5xy0(&mut self, x: usize, y: usize) -> PCAction {
	return if self.v[x] == self.v[y] {
	    PCAction::Forward(2)
	} else {
	    PCAction::Forward(1)
	}
    }

    // SKIP next instruction if V[x] != NN
    fn inst_6xnn(&mut self, x: usize, nn: u8) -> PCAction {
	self.v[x] = nn;
	return PCAction::Forward(1)
    }

    // Adds nn to V[x]. Carry flag is not set on overflow
    fn inst_7xnn(&mut self, x: usize, nn: u8) -> PCAction {
	self.v[x] = self.v[x].wrapping_add(nn);
	return PCAction::Forward(1)
    }

    // Adds nn to V[x]. Carry flag is not set on overflow
    fn inst_8xy0(&mut self, x: usize, y: usize) -> PCAction {
	self.v[x] = self.v[y];
	return PCAction::Forward(1)
    }

    // Adds nn to V[x]. Carry flag is not set on overflow
    fn inst_8xy1(&mut self, x: usize, y: usize) -> PCAction {
	self.v[x] |= self.v[y];
	return PCAction::Forward(1)
    }

    fn inst_8xy2(&mut self, x: usize, y: usize) -> PCAction {
	self.v[x] &= self.v[y];
	return PCAction::Forward(1)
    }

    fn inst_8xy3(&mut self, x: usize, y: usize) -> PCAction {
	self.v[x] ^= self.v[y];
	return PCAction::Forward(1)
    }

    fn inst_8xy4(&mut self, x: usize, y: usize) -> PCAction {
	let addition_overflow_check = self.v[x].checked_add(self.v[y]);
	self.v[x] = self.v[x].wrapping_add(self.v[y]);

	let f_flag = match addition_overflow_check {
	    Some(_) => 0,
	    None => 1 //overflow happened
	};

	self.v[0x0f] = f_flag;
	return PCAction::Forward(1)
    }

    fn inst_8xy5(&mut self, x: usize, y: usize) -> PCAction {
	let sub_overflow_check = self.v[x].checked_sub(self.v[y]);
	self.v[x] = self.v[x].wrapping_sub(self.v[y]);

	let f_flag = match sub_overflow_check {
	    Some(_) => 1,
	    None => 0 //overflow happened
	};

	self.v[0x0f] = f_flag;
	return PCAction::Forward(1)
    }

    fn inst_8xy6(&mut self, x: usize, _y: usize) -> PCAction {
	self.v[0x0f] = self.v[x] & 1;
        self.v[x] >>= 1;
        return PCAction::Forward(1)
    }

    fn inst_8xy7(&mut self, x: usize, y: usize) -> PCAction {
	self.v[0x0f] = if self.v[y] > self.v[x] { 1 } else { 0 };
        self.v[x] = self.v[y].wrapping_sub(self.v[x]);
	return PCAction::Forward(1)
    }

    fn inst_8xye(&mut self, x: usize, y: usize) -> PCAction {
	self.v[x] ^= self.v[y];
	return PCAction::Forward(1)
    }

    fn inst_9xy0(&mut self, x: usize, y: usize) -> PCAction {
	return if self.v[x] != self.v[y] {
	    PCAction::Forward(2)
	} else {
	    PCAction::Forward(1)
	}
    }
    
    fn inst_annn(&mut self, nnn: usize) -> PCAction {
	self.i = nnn;
	return PCAction::Forward(1)
    }

    fn inst_bnnn(&mut self, nnn: usize) -> PCAction {
	return PCAction::Set((self.v[0] as usize) + nnn)
    }

    fn inst_cxnn(&mut self, x: usize, nn: u8) -> PCAction {
        self.v[x] = rand::thread_rng().gen::<u8>() & nn;
	return PCAction::Forward(1)
    }

    fn inst_dxyn(&mut self, x: usize, y: usize, n: usize) -> PCAction {
	self.v[0x0f] = 0;
        for byte in 0..n {
            let y = (self.v[y] as usize + byte) % crate::vram::VRAM_HEIGHT;
            for bit in 0..8 {
                let x = (self.v[x] as usize + bit) % crate::vram::VRAM_WIDTH;
                let color = (self.ram.fetch(self.i + byte) >> (7 - bit)) & 1;
                self.v[0x0f] |= color & self.vram.fetch(y,x);
		let new_val = self.vram.fetch(y,x) ^ color;
                self.vram.set(y, x, new_val)
            }
        }
	return PCAction::Forward(1)
    }

    fn inst_ex9e(&mut self, x: usize) -> PCAction {
	if self.keymap[self.v[x] as usize] {
	    return PCAction::Forward(2)
	} else {
	    return PCAction::Forward(1)
	}
    }

    fn inst_exa1(&mut self, x: usize) -> PCAction {
	if !self.keymap[self.v[x] as usize] {
	    return PCAction::Forward(2)
	} else {
	    return PCAction::Forward(1)
	}
    }

    fn inst_fx07(&mut self, x: usize) -> PCAction {
	self.v[x] = self.delay_timer;
	return PCAction::Forward(1)
    }

    fn inst_fx0a(&mut self, x: usize) -> PCAction {
	//TODO key related
	return match self.keymap.iter()
	    .enumerate()
	    .find(|x| match x {(_,v) => **v})
	{
	    None => PCAction::Forward(0),
	    Some(v) => {
		let (key,_) = v;
		self.v[x] = key as u8;
		PCAction::Forward(1)
	    }
	}
    }

    fn inst_fx15(&mut self, x: usize) -> PCAction {
	self.delay_timer = self.v[x];
	return PCAction::Forward(1)
    }

    fn inst_fx18(&mut self, x: usize) -> PCAction {
	self.sound_timer = self.v[x];
	return PCAction::Forward(1)
    }

    fn inst_fx1e(&mut self, x: usize) -> PCAction {
	self.i = self.i.wrapping_add(self.v[x] as usize);
	return PCAction::Forward(1)
    }

    fn inst_fx29(&mut self, x: usize) -> PCAction {
	self.i = (self.v[x] as usize) * 5;
	return PCAction::Forward(1)
    }

    fn inst_fx33(&mut self, x: usize) -> PCAction {
        self.ram.set(self.i, self.v[x] / 100);
        self.ram.set(self.i + 1, (self.v[x] % 100) / 10);
        self.ram.set(self.i + 2, self.v[x] % 10);
	return PCAction::Forward(1)
    }	

    fn inst_fx55(&mut self, x: usize) -> PCAction {
	for i in 0..x+1 {
	    Memory::set(self.ram, self.i + i, self.v[i])
	}
	return PCAction::Forward(1)
    }

    fn inst_fx65(&mut self, x: usize) -> PCAction {
	for i in 0..x+1 {
	    self.v[i] = Memory::fetch(self.ram, self.i + i) 
	}
	return PCAction::Forward(1)
    }

}
