extern crate sdl2;
mod memory;
mod cpu;
mod vram;
mod display;
mod cartridge;
mod font;
mod keyboard;

use std::thread;
use std::time::Duration;
use std::env;

fn main() {
    // command line args
    let args: Vec<String> = env::args().collect();

    //initialize SDL2
    let sdl_context = sdl2::init().unwrap();

    // Hardware drivers
    let mut display_driver = display::DisplayDriver::new(&sdl_context);
    let mut keyboard_driver = keyboard::InputDriver::new(&sdl_context);

    // Hardware emulation
    let mut vram = vram::Vram::init();
    let mut mem = memory::Memory::init();

    let cartridge = cartridge::Cartridge::load(&args[1]);
    mem.load_cartridge(&cartridge.data);
    mem.load_font_set(&font::FONT_SET);

    let mut cpu = cpu::CPU::init(&mut mem, &mut vram);

    // Event loop

    while let Ok(keypad) = keyboard_driver.poll() {

	cpu::CPU::set_keymap(&mut cpu, keypad);
	cpu::CPU::decrement_timers(&mut cpu);

	let opcode = cpu::CPU::fetch_opcode(&mut cpu);
	cpu::CPU::execute_opcode(&mut cpu, opcode);
	thread::sleep(Duration::from_millis(2));
	display_driver.draw(&cpu::CPU::fetch_vram(&mut cpu));
    }
}
