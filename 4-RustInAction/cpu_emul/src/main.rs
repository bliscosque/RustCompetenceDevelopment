struct CPU {
    registers: [u8;16], //16 registers
    position_in_memory: usize, //program counter (next instruction)
    memory: [u8; 4096],
    stack: [u16; 16],
    stack_pointer: usize,

}
fn main() {
    println!("Hello, world!");
}
