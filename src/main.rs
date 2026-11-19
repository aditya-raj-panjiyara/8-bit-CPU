// define the CPU
struct CPU {
    /// this is normally refered as *program_conuter* but to make it obiovous we are calling it *position_in_mem*
    position_in_mem: usize,
    /// this repersent 4kb i.e *4096 bytes* of RAM which is 0x1000 in hexadecimal
    memory: [u8; 0x1000],
    registers: [u8; 16],
}

impl CPU {
    /// just a function to read te opcode
    fn current_opcode(&self) -> u16 {
        let p = self.position_in_mem;
        let p1 = self.memory[p] as u16;
        let p2 = self.memory[p + 1] as u16;

        (p1 << 8) | p2
    }

    /// this is what will run the main loop.
    fn run(&mut self) {
        loop {
            // fetch
            let opcode = self.current_opcode();
            // advance
            self.position_in_mem += 2; // becuase we are so storing it in two slots

            // decode
            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;

            // execute
            match (c, x, y, d) {
                (0, 0, 0, 0) => {
                    return;
                }
                (0x8, _, _, 0x4) => self.adder(x, y),
                _ => todo!("implment the rest of the functions here too"),
            }
        }
    }

    /// the added fuction here x and y are the register position
    fn adder(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow) = arg1.overflowing_add(arg2);

        self.registers[x as usize] = val;
        // In chip-8 0xf register is the flag or the status flag for overflow

        self.registers[0xF] = overflow as u8; // 1 if true 0 if false
    }
}

fn main() {
    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 4096],
        position_in_mem: 0,
    };
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.registers[2] = 10;
    cpu.registers[3] = 10;

    let mem = &mut cpu.memory;
    mem[0] = 0x80;
    mem[1] = 0x14; // opcode 0x8014: add reg1 to reg0
    mem[2] = 0x80;
    mem[3] = 0x24; // opcode 0x8024: add reg2 to reg0
    mem[4] = 0x80;
    mem[5] = 0x34; // opcode 0x8034: add reg3 to reg0

    cpu.run();
    println!("{}", cpu.registers[0]);
    assert_eq!(cpu.registers[0], 35);
}
