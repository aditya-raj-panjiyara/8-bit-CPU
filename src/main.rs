// this is the the implementation on CHIP-8

// for this implementaion the CPU will only contian only 2 resgister and the space for only one
// op code. (op here is a "Operation")
struct CPU {
    current_opration: u16,
    registers: [u8; 2],
}

// the step we need
// 1. Initializ the CPU
// 2. Load u8 values into `current_opreation`
// 3. Load the addition opcode into currnt_opreation.
// 4. Perform the opreation

//Initializ the CPU

fn main() {
    let mut cpu = CPU {
        current_opration: 0,
        registers: [0; 2],
    };

    cpu.current_opration = 0x8014;
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
}
