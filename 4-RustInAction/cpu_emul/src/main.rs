struct CPU {
    registers: [u8;16], //16 registers
    position_in_memory: usize, //program counter (next instruction)
    memory: [u8; 4096],
    stack: [u16; 16],
    stack_pointer: usize,

}

impl CPU {
    fn run(&mut self) {
        loop {
            let op_byte1 = self.memory[self.position_in_memory] as u16;
            let op_byte2 = self.memory[self.position_in_memory+1] as u16;
            let opcode: u16=op_byte1 << 8 | op_byte2; //basically converting 2 bytes in 1 u16 because opcode is defined as u16

            let x = ((opcode & 0x0F00) >>  8) as u8; // high byte, low nibble, CPU register. See page 161
            let y = ((opcode & 0x00F0) >>  4) as u8; // low byte, high nibble, CPU register
            let kk = (opcode & 0x00FF) as u8; // low byte, both nibbles
            let op_minor = (opcode & 0x000F) as u8;
            let addr = opcode & 0x0FFF; // high byte, low nibble and low byte, both nibbles

            //3 maneiras: 
            // 0x73EE = 7(Opcode) | 3 (Reg x) | EE(Arg (kk)) = Add 238 (0xEE) to reg 3
            // 0x1200 = 1(Opcode) | 200 (address) = Jump to memory addr 0x200
            // 0x8231 = 8(Opcode) | 2 (reg x) | 3 (reg y) | 1 (opcode subtype) = bitwise OR with reg x and reg y, stores at reg x
        }
    }
}

fn main() {
    println!("Hello, world!");
}
