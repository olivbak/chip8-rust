mod memory;
mod cpu;

fn main() {
    let mut mem = memory::Memory::init();
    let mut cpu1 = cpu::CPU::init(&mut mem);

    let opcode = cpu::CPU::fetch_opcode(&mut cpu1);
    cpu::CPU::execute_opcode(&mut cpu1, opcode);

    println!("hola!")

}
