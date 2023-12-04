#[derive(Debug)]
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

            self.position_in_memory += 2; //advancing...

            match opcode {
                0x0000 => { return; },
                0x00E0 => { /* CLEAR SCREEN */ },
                0x00EE => { self.ret(); },
                0x1000..=0x1FFF => { self.jmp(addr); }, 
                0x2000..=0x2FFF => { self.call(addr); },
                0x3000..=0x3FFF => { self.se(x, kk); },
                0x4000..=0x4FFF => { self.sne(x, kk); },
                0x5000..=0x5FFF => { self.se(x, y); },
                0x6000..=0x6FFF => { self.ld(x, kk); },
                0x7000..=0x7FFF => { self.add(x, kk); },
                0x8000..=0x8FFF => {
                    match op_minor { //last bit
                        0 => { self.ld(x, self.registers[y as usize]) },
                        1 => { self.or_xy(x, y) },
                        //2 => { self.and_xy(x, y) },
                        //3 => { self.xor_xy(x, y) },
                        4 => { self.add_xy(x, y); },
                        _ => { todo!("opcode: {:04x}", opcode); },
                    }
                },
                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }

    // opcode 2xxx - CALL at addr (xxx)
    fn call(&mut self, addr: u16) {
        println!("At call to addr: {:x}",addr);
        let sp=self.stack_pointer;
        let stack=&mut self.stack;
        if sp>=stack.len() {
            panic!("Stack overflow!")
        }
        stack[sp]=self.position_in_memory as u16;
        self.stack_pointer+=1;
        self.position_in_memory=addr as usize;
    }

    // opcode 1xxx - JMP at addr (xxx)
    fn jmp(&mut self, addr: u16) {
        self.position_in_memory=addr as usize;
    }

    // opcode 0x3xkk - Store if equals OR
    // opcode 0x5xyN (vx=x, kk=y)
    fn se(&mut self, vx:u8, kk:u8) {
        if vx==kk {
            self.position_in_memory+=2;
        }
    }

    // opcode 0x4xkk - Store if NOT equals
    fn sne(&mut self, vx:u8, kk:u8) {
        if vx!=kk {
            self.position_in_memory+=2;
        }
    }

    // opcode 0x6xkk - Load Data - sets the value kk into register vx
    fn ld(&mut self, vx:u8, kk:u8){
        self.registers[vx as usize] = kk;
    }

    // opcode 0x7xkk - Add value kk into register vx
    fn add(&mut self, vx:u8, kk:u8) {
        self.registers[vx as usize] += kk;
    }

    // opcode 8xy4 - ADD (reg x=reg x + reg y)
    fn add_xy(&mut self, x:u8, y:u8) {
        self.registers[x as usize] += self.registers[y as usize];
        //TODO: CARRY FLAG
    }

    //opcode 8xy1 - OR (reg x= reg x OR reg y) 
    fn or_xy(&mut self, x:u8, y:u8) {
        let x_ = self.registers[x as usize];
        let y_ = self.registers[y as usize];
        self.registers[x as usize] = x_ | y_;
    }

    // opcode 00EE RET (return from subroutine)
    fn ret(&mut self) {
        if self.stack_pointer==0 {
            panic!("Stack underflow");
        }
        self.stack_pointer-=1;
        self.position_in_memory = self.stack[self.stack_pointer] as usize;
    }
}

fn main() {
    let mut cpu = CPU {
        registers: [0;16],
        memory: [0;4096],
        position_in_memory: 0,
        stack: [0;16],
        stack_pointer: 0,
    };

    //initializing a few reg with values
    cpu.registers[0]=5;
    cpu.registers[1]=10;
    
    cpu.memory[0x000] = 0x21; cpu.memory[0x001] = 0x00; // 2=call 0x100
    cpu.memory[0x002] = 0x21; cpu.memory[0x003] = 0x00; // 2=call 0x100

    //Pos 0x100 of the Call
    cpu.memory[0x100] = 0x80; cpu.memory[0x101] = 0x14; // ADD reg 0 with reg 1 (registers: [15, 10,...])  reg0=5+10=15
    cpu.memory[0x102] = 0x80; cpu.memory[0x103] = 0x14; // ADD reg 0 with reg 1 (registers: [15, 10,...])  reg0=15+10=25
    cpu.memory[0x104] = 0x00; cpu.memory[0x105] = 0xEE; // return of function (function is called once again... so reg0 will be summed with +20...45)

    cpu.run();
    println!("{:?}",cpu.registers);
}
