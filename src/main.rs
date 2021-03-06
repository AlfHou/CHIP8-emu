use std::io::prelude::*;
use rand::Rng;
use std::env;
use std::process;
use std::io::BufReader;
use std::fs::File;

#[allow(non_snake_case)]
#[allow(dead_code)]
struct CPU {
    regs: [u8; 16],
    I: u16, // Address register
    PC: u16, // Program counter
    // Put timers in cpu struct to simplify design
    sound_timer: u8,
    delay_timer: u8
}


#[allow(non_snake_case)]
fn init_cpu() -> CPU {
    let cpu = CPU {
        regs: [0; 16],
        I: 0,
        PC: 0,
        sound_timer: 0,
        delay_timer: 0
    };
    println!("CPU initiated");
    return cpu;
}

fn set_reg_const(opcode: u16, cpu: &mut CPU) {
    let reg = ((opcode & 0x0F00) >> 8) as usize;
    let val = (opcode & 0x00FF) as u8;
    println!("Setting register {:#02x} to value {:#02x}", reg, val);
    cpu.regs[reg] = val;
}

fn set_reg_reg(opcode: u16, cpu: &mut CPU) {
    let reg1 = ((opcode & 0x0F00) >> 8) as usize;
    let reg2 = ((opcode & 0x00F0) >> 4) as usize;
    cpu.regs[reg1] = cpu.regs[reg2];
}

fn add_const_to_reg(opcode: u16, cpu: &mut CPU) {
    let reg = ((opcode & 0x0F00) >> 8) as usize;
    let val = (opcode & 0x00FF) as u8;
    cpu.regs[reg] += val;
}

fn bitwise_or(opcode: u16, cpu: &mut CPU) {
    let reg1 = ((opcode & 0x0F00) >> 8) as usize;
    let reg2 = ((opcode & 0x00F0) >> 4) as usize;

    let val1 = cpu.regs[reg1];
    let val2 = cpu.regs[reg2];

    cpu.regs[reg1] = val1 | val2;
}

fn bitwise_and(opcode: u16, cpu: &mut CPU) {
    let reg1 = ((opcode & 0x0F00) >> 8) as usize;
    let reg2 = ((opcode & 0x00F0) >> 4) as usize;

    let val1 = cpu.regs[reg1];
    let val2 = cpu.regs[reg2];

    cpu.regs[reg1] = val1 & val2;
}

fn bitwise_xor(opcode: u16, cpu: &mut CPU) {
    let reg1 = ((opcode & 0x0F00) >> 8) as usize;
    let reg2 = ((opcode & 0x00F0) >> 4) as usize;

    let val1 = cpu.regs[reg1];
    let val2 = cpu.regs[reg2];

    cpu.regs[reg1] = val1 ^ val2;
}

fn right_shift(opcode: u16, cpu: &mut CPU) {
    let reg = ((opcode & 0x0F00) >> 8) as usize;
    let val = cpu.regs[reg];

    // Save least sigfig of register before shifting
    cpu.regs[0xF] = val & 0x1;

    cpu.regs[reg] = val >> 1;
}

fn left_shift(opcode: u16, cpu: &mut CPU) {
    let reg = ((opcode & 0x0F00) >> 8) as usize;
    let val = cpu.regs[reg];

    // Save most sigfig of register before shifting
    cpu.regs[0xF] = (val & 0x80) >> 7;

    cpu.regs[reg] = val << 1;
}

fn set_address_register_const(opcode: u16, cpu: &mut CPU) {
    let addr = opcode & 0x0FFF;

    cpu.I = addr;
}
    
fn add_reg_address_register(opcode: u16, cpu: &mut CPU) {
    let reg = ((opcode & 0x0F00) >> 8) as usize;
    cpu.I += cpu.regs[reg] as u16;
}

fn set_sound_timer(opcode: u16, cpu: &mut CPU) {
    let reg = ((opcode & 0x0F00) >> 8) as usize;
    
    cpu.sound_timer = cpu.regs[reg]
}

fn set_delay_timer(opcode: u16, cpu: &mut CPU) {
    let reg = ((opcode & 0x0F00) >> 8) as usize;
    
    cpu.delay_timer = cpu.regs[reg]
}

fn set_reg_and_rand_const(opcode: u16, cpu: &mut CPU) {
    let reg = ((opcode & 0x0F00) >> 8) as usize;
    let constant = (opcode & 0x00FF) as u8;

    let mut rng = rand::thread_rng();
    let rand_num = rng.gen_range(0..=255);

    cpu.regs[reg] = constant & rand_num;
}


fn handle_opcode(opcode: u16, cpu: &mut CPU) -> Result<(), &str> {

    println!("Handling opcode: {:#04x}", opcode);
    match opcode {
        0x00E0 => { 
            println!("'Display clear' not implemented!");
            return Err("Not Implemented");
        },
        0x00EE => {
            println!("'return' not implemented!");
            return Err("Not implemented");
        }
        op if 0xF0FF & op == 0xF065 => {
            println!("'reg_load' not implemented!");
            return Err("Not implemented");
        }
        op if 0xF0FF & op == 0xF055 => {
            println!("'reg_dump' not implemented!");
            return Err("Not implemented");
        }
        op if 0xF0FF & op == 0xF033 => {
            println!("'store binary-coded decimal' not implemented!");
            return Err("Not implemented");
        }
        op if 0xF0FF & op == 0xF029 => {
            println!("'set I to loc of sprite' not implemented!");
            return Err("Not implemented");
        }
        op if 0xF0FF & op == 0xF01E => { 
            add_reg_address_register(op, cpu);
            return Ok(());
        }
        op if 0xF0FF & op == 0xF018 => {
            set_sound_timer(op, cpu);
            return Ok(());
        }
        op if 0xF0FF & op == 0xF015 => {
            set_delay_timer(op, cpu);
            return Ok(());
        }
        op if 0xF0FF & op == 0xF00A => {
            println!("'wait keypress' not implemented!");
            return Err("Not implemented");
        }
        op if 0xF0FF & op == 0xF007 => {
            println!("'set reg d_timer' not implemented!");
            return Err("Not implemented");
        }
        op if 0xF0FF & op == 0xE0A1 => {
            println!("'not keypress' not implemented!");
            return Err("Not implemented");

        }
        op if 0xF0FF & op == 0xE09E => {
            println!("'keypress' not implemented!");
            return Err("Not implemented");
        }
        op if 0xF000 & op == 0xD000 => {
            println!("'draw' not implemented!");
            return Err("Not implemented");
        }
        op if 0xF000 & op == 0xC000 => {
            set_reg_and_rand_const(op, cpu);
            return Ok(());
        }
        op if 0xF000 & op == 0xB000 => {
            println!("'jump addr reg plus const' not implemented!");
            return Err("Not implemented");
        }
        op if 0xF000 & op == 0xA000 => {
            set_address_register_const(op, cpu);
            return Ok(());
        }
        op if 0xF000 & op == 0x9000 => {
            println!("'jneq reg' not implemented!");
            return Err("Not implemented");
        }
        op if 0xF00F & op == 0x800E => {
            left_shift(op, cpu);
            return Ok(());
        }
        op if 0xF00F & op == 0x8007 => {
            println!("'set reg1 to reg2 minus reg1' not implemented!");
            return Err("Not implemented");
        }
        op if 0xF00F & op == 0x8006 => {
            right_shift(op, cpu);
            return Ok(());
        }
        op if 0xF00F & op == 0x8005 => {
            println!("'subtract reg from reg' not implemented!");
            return Err("Not implemented");
        }
        op if 0xF00F & op == 0x8004 => {
            println!("'add reg to reg' not implemented!");
            return Err("Not implemented");
        }
        op if 0xF00F & op == 0x8003 => {
            bitwise_xor(op, cpu);
            return Ok(());
        }
        op if 0xF00F & op == 0x8002 => {
            bitwise_and(op, cpu);
            return Ok(());
        }
        op if 0xF00F & op == 0x8001 => {
            bitwise_or(op, cpu);
            return Ok(());
        }
        op if 0xF00F & op == 0x8000 => {
            set_reg_reg(op, cpu);
            return Ok(());
        }
        op if 0xF000 & op == 0x7000 => {
            add_const_to_reg(op, cpu);
            return Ok(());
        }
        op if 0xF000 & op == 0x6000 => {
            set_reg_const(op, cpu);
            return Ok(());
        }
        op if 0xF00F & op == 0x5000 => {
            println!("'jeq reg' not implemented!");
            return Err("Not implemented");
        }
        op if 0xF000 & op == 0x4000 => {
            println!("'jeq const' not implemented!");
            return Err("Not implemented");
        }
        op if 0xF000 & op == 0x3000 => {
            println!("'jneq const' not implemented!");
            return Err("Not implemented");
        }
        op if 0xF000 & op == 0x2000 => {
            println!("'call at addr' not implemented!");
            return Err("Not implemented");
        }
        op if 0xF000 & op == 0x1000 => {
            println!("'jump' not implemented!");
            return Err("Not implemented");
        }
        op if 0xF000 & op == 0x0000 => {
            println!("'machine code routine' not implemented!");
            return Err("Not implemented");
        }
        _ => {
            println!("Opcode not recognized");
            return Err("Not implemented");
        }
    }

}
fn load_rom(rom_path: String, memory: &mut [u8]) -> std::io::Result<()> {
    println!("Rom path: {}!", rom_path);
    let f = File::open(rom_path)?;
    let mut reader = BufReader::new(f);

    reader.read(&mut memory[0x200 .. 0x1000])?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let mut cpu = init_cpu();
    let mut memory: [u8; 4096] = [0; 4096];
    println!("Memory initiated");

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run <rom_path>");
        process::exit(1);
    }
    let rom_path = &args[1];

    load_rom(rom_path.to_string(), &mut memory)?;

    // Set program counter to start of ROM
    cpu.PC = 0x200;


    while true {
        let opcode = ((memory[cpu.PC as usize] as u16) << 8)
            + memory[cpu.PC as usize + 1] as u16;
        println!("opcode: {:#04x}", opcode);
        cpu.PC += 8;
        println!("cpu.PC: {:#04x}", cpu.PC);

        let res = handle_opcode(opcode, &mut cpu);
        if res.is_err() {
            println!("Error handling opcode");
            panic!("Error handling opcode");
        }
        if (cpu.PC > 0x220) {
            break;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_reg_set_const() {
        let mut cpu = init_cpu();
        handle_opcode(0x63B2, &mut cpu);
        assert_eq!(cpu.regs[0x3], 0xB2);

        handle_opcode(0x6F02, &mut cpu);
        assert_eq!(cpu.regs[0xF], 0x2);

        handle_opcode(0x6010, &mut cpu);
        assert_eq!(cpu.regs[0x0], 0x10);
    }

    #[test]
    fn test_reg_set_reg() {
        let mut cpu = init_cpu();
        // Set V9 to 0xF2
        handle_opcode(0x69F2, &mut cpu);

        // Should set reg VA to the same as V9
        handle_opcode(0x8A90, &mut cpu);
        assert_eq!(cpu.regs[0xA], 0xF2);

        // Should change back to 0
        handle_opcode(0x8AF0, &mut cpu);
        assert_eq!(cpu.regs[0xA], 0x0);
    }

    #[test]
    fn test_reg_add_const() {
        let mut cpu = init_cpu();
        handle_opcode(0x7102, &mut cpu);
        assert_eq!(cpu.regs[0x1], 0x2);

        handle_opcode(0x7110, &mut cpu);
        assert_eq!(cpu.regs[0x1], 0x12);
    }

    #[test]
    fn test_bit_or() {
        let mut cpu = init_cpu();
        // Set V1 to 0101 1011
        handle_opcode(0x615B, &mut cpu);

        // Set V2 to 1001 1111
        handle_opcode(0x629F, &mut cpu);

        // Do the OR operation and store in V1
        handle_opcode(0x8121, &mut cpu);

        assert_eq!(cpu.regs[0x1], 0xDF);
    }

    #[test]
    fn test_bit_and() {
        let mut cpu = init_cpu();
        // Set V1 to 0101 0101
        handle_opcode(0x6155, &mut cpu);

        // Set V2 to 1000 1101
        handle_opcode(0x628D, &mut cpu);

        // Do the AND operation and store in V1
        handle_opcode(0x8122, &mut cpu);

        assert_eq!(cpu.regs[0x1], 0x5);
    }

    #[test]
    fn test_bit_xor() {
        let mut cpu = init_cpu();
        // Set V1 to 0110 1001
        handle_opcode(0x6169, &mut cpu);

        // Set V2 to 0100 0010
        handle_opcode(0x6242, &mut cpu);

        // Do the XOR operation and store in V1
        handle_opcode(0x8123, &mut cpu);

        assert_eq!(cpu.regs[0x1], 0x2B);


        // Set V3 to 1011 1011
        handle_opcode(0x63BB, &mut cpu);

        // Set V4 to 1011 1011
        handle_opcode(0x64BB, &mut cpu);

        // Do the XOR operation and store in V1
        handle_opcode(0x8343, &mut cpu);

        assert_eq!(cpu.regs[0x3], 0x00);
    }

    #[test]
    fn test_right_shift() {
        let mut cpu = init_cpu();

        // Test that VF stores 1
        //
        // Set V1 to 0100 0011
        handle_opcode(0x6143, &mut cpu);

        // Set V2 to 1111 1101. V2 is not used, but set it just for the hell of it
        handle_opcode(0x62FD, &mut cpu);

        // Do the bitshift 
        handle_opcode(0x8126, &mut cpu);

        assert_eq!(cpu.regs[0x1], 0x21);
        // Assert VF flag stores bit that was shifted out
        assert_eq!(cpu.regs[0xF], 0x1);


        // Test that VF stores 0
        // Set V3 to 0101 0010
        handle_opcode(0x6352, &mut cpu);

        // Do the bitshift 
        handle_opcode(0x8346, &mut cpu);

        assert_eq!(cpu.regs[0x3], 0x29);
        // Assert VF flag stores bit that was shifted out
        assert_eq!(cpu.regs[0xF], 0x0);
    }

    #[test]
    fn test_left_shift() {
        let mut cpu = init_cpu();

        // Test that VF stores 1
        //
        // Set V1 to 1110 1000
        handle_opcode(0x61E8, &mut cpu);

        // Set V2 to 1111 1101. V2 is not used, but set it just for the hell of it
        handle_opcode(0x62FD, &mut cpu);

        // Do the bitshift 
        handle_opcode(0x812E, &mut cpu);

        assert_eq!(cpu.regs[0x1], 0xD0);
        // Assert VF flag stores bit that was shifted out
        assert_eq!(cpu.regs[0xF], 0x1);


        // Test that VF stores 0
        // Set V3 to 0101 0011
        handle_opcode(0x6353, &mut cpu);

        // Do the bitshift 
        handle_opcode(0x834E, &mut cpu);

        assert_eq!(cpu.regs[0x3], 0xA6);
        // Assert VF flag stores bit that was shifted out
        assert_eq!(cpu.regs[0xF], 0x0);
    }

    #[test]
    fn test_set_address_register() {
        let mut cpu = init_cpu();
        handle_opcode(0xA123, &mut cpu);
        assert_eq!(cpu.I, 0x123);

        handle_opcode(0xAFF0, &mut cpu);
        assert_eq!(cpu.I, 0xFF0);
    }

    #[test]
    fn test_add_reg_to_address_register() {
        let mut cpu = init_cpu();

        // Set address register to 1100 000 0001
        handle_opcode(0xAC01, &mut cpu);

        // Set V1 to 0101 0101
        handle_opcode(0x6155, &mut cpu);

        // Do add operation
        handle_opcode(0xF11E, &mut cpu);

        assert_eq!(cpu.I, 0xC56);
    }

    #[test]
    fn test_set_sound_timer() {
        let mut cpu = init_cpu();

        // Set V1 to 1110 1110
        handle_opcode(0x61EE, &mut cpu);

        handle_opcode(0xF118, &mut cpu);

        assert_eq!(cpu.sound_timer, 0xEE);
    }

    #[test]
    fn test_set_delay_timer() {
        let mut cpu = init_cpu();

        // Set V1 to 0011 0100
        handle_opcode(0x6134, &mut cpu);

        handle_opcode(0xF115, &mut cpu);

        assert_eq!(cpu.delay_timer, 0x34);
    }


}
