// define the CPU
struct CPU {
    opcode: u16,
    registers: [u8; 2],
}

impl CPU {
    /// just a function to read te opcode
    fn read_opcode(&self) -> u16 {
        self.opcode
    }

    fn run(&mut self) {
        let opcode = self.read_opcode();
        let c = ((opcode & 0xF000) >> 12) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let d = ((opcode & 0x000F) >> 0) as u8;

        match (c, x, y, d) {
            (0x8, _, _, 0x4) => self.adder(x, y),
            _ => todo!("implment the rest of the functions here too"),
        }
    }

    fn adder(&mut self, x: u8, y: u8) {
        self.registers[x as usize] += self.registers[y as usize];
    }
}

fn main() {
    let mut cpu = CPU {
        opcode: 0, // zero means none
        registers: [0; 2],
    };

    cpu.opcode = 0x8014;
    cpu.registers[0] = 43;
    cpu.registers[1] = 32;
    cpu.run();

    println!("the sum is {}", cpu.registers[0]);
}
