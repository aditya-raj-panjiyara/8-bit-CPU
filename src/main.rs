// this is the the implementation on CHIP-8

// for this implementaion the CPU will only contian only 2 resgister and the space for only one
// op code. (op here is a "Operation")
//
// the step we need
// 1. Initializ the CPU
// 2. Load u8 values into `current_opreation`
// 3. Load the addition opcode into currnt_opreation.
// 4. Perform the opreation

struct CPU {
    current_opration: u16,
    registers: [u8; 2],
    postion_in_memory: usize, // this is a variable which indicate which istruction to execute next
    memory: [u8; 0x1000],
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        self.current_opration // will become more complex as we read form memory
    }

    // understanding the main loop
    // 1. read the op code eventually form the memory
    // 2. decode instruciton
    // 3. match the decoded instructiuon to known opcode
    // 4. dispatches the execution to known opcodes to spcific function
    fn run(&mut self) {
        // loop {
        let opcode = self.read_opcode();
        let c = ((opcode & 0xF000) >> 12) as u8; // here by using the and opreator with the here we will get the nibble on where F is
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let d = ((opcode & 0x000F) >> 0) as u8;

        match (c, x, y, d) {
            (0x8, _, _, 0x4) => self.add_xy(x, y),
            _ => todo!("opcode {:04x}", opcode),
        }

        // }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] += self.registers[y as usize];
    }
}

//Initializ the CPU
fn main() {
    let mut cpu = CPU {
        current_opration: 0, // init with a no op -> 0 "zero" means no op
        registers: [0; 2],
        postion_in_memory: 10,
    };

    cpu.current_opration = 0x8014; // 8 -> its has 2 register, 0&1 -> register address, 4 -> we have to do addition
    cpu.registers[0] = 5; // register can hold u8 only
    cpu.registers[1] = 10;
    cpu.run();

    assert_eq!(cpu.registers[0], 15);

    println!("the sum is {}", cpu.registers[0]);
}
