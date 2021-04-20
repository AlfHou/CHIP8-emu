
#[allow(non_snake_case)]
#[allow(dead_code)]
struct CPU {
    regs: [u8; 16],
    I: u16, // Address register
    PC: u16 // Program counter
}


#[allow(non_snake_case)]
fn init_cpu() -> CPU {
    let cpu = CPU {
        regs: [0; 16],
        I: 0,
        PC: 0
    };
    println!("CPU initiated");
    return cpu;
}

fn set_reg_const(opcode: u16, cpu: &mut CPU) {
    let reg = ((opcode & 0x0F00) >> 8) as usize;
    let val = (opcode & 0x00FF) as u8;
    cpu.regs[reg] = val;
    println!("\nCPU register {} set to {}!\n", reg, cpu.regs[reg]);
}

fn set_reg_reg(opcode: u16, cpu: &mut CPU) {
    let reg1 = ((opcode & 0x0F00) >> 8) as usize;
    let reg2 = ((opcode & 0x00F0) >> 4) as usize;
    cpu.regs[reg1] = cpu.regs[reg2];
    println!("\nreg1: {}, reg2: {} \n", cpu.regs[reg1], cpu.regs[reg2]);
}

fn add_const_to_reg(opcode: u16, cpu: &mut CPU) {
    let reg = ((opcode & 0x0F00) >> 8) as usize;
    let val = (opcode & 0x00FF) as u8;
    cpu.regs[reg] += val;
    println!("\nCPU register {} set to {}!\n", reg, cpu.regs[reg]);

}



fn handle_opcode(opcode: u16, cpu: &mut CPU) {
    match opcode {
        0x00E0 => println!("'Display clear' not implemented!"),
        0x00EE => println!("'return' not implemented!"),
        op if 0xF0FF & op == 0xF065 => println!("'reg_load' not implemented!"),
        op if 0xF0FF & op == 0xF055 => println!("'reg_dump' not implemented!"),
        op if 0xF0FF & op == 0xF033 => println!("'store binary-coded decimal' not implemented!"),
        op if 0xF0FF & op == 0xF029 => println!("'set I to loc of sprite' not implemented!"),
        op if 0xF0FF & op == 0xF01E => println!("'set I to reg' not implemented!"),
        op if 0xF0FF & op == 0xF018 => println!("'set s_timer to reg' not implemented!"),
        op if 0xF0FF & op == 0xF015 => println!("'set d_timer to reg' not implemented!"),
        op if 0xF0FF & op == 0xF00A => println!("'wait keypress' not implemented!"),
        op if 0xF0FF & op == 0xF007 => println!("'set reg d_timer' not implemented!"),
        op if 0xF0FF & op == 0xE0A1 => println!("'not keypress' not implemented!"),
        op if 0xF0FF & op == 0xE09E => println!("'keypress' not implemented!"),
        op if 0xF000 & op == 0xD000 => println!("'draw' not implemented!"),
        op if 0xF000 & op == 0xC000 => println!("'set reg to bitwise AND with rand' not implemented!"),
        op if 0xF000 & op == 0xB000 => println!("'jump addr reg plus const' not implemented!"),
        op if 0xF000 & op == 0xA000 => println!("'set I const' not implemented!"),
        op if 0xF000 & op == 0x9000 => println!("'jneq reg' not implemented!"),
        op if 0xF00F & op == 0x800E => println!("'bitwise left shift by 1' not implemented!"),
        op if 0xF00F & op == 0x8007 => println!("'set reg1 to reg2 minus reg1' not implemented!"),
        op if 0xF00F & op == 0x8006 => println!("'bitwise right shift by 1' not implemented!"),
        op if 0xF00F & op == 0x8005 => println!("'subtract reg from reg' not implemented!"),
        op if 0xF00F & op == 0x8004 => println!("'add reg to reg' not implemented!"),
        op if 0xF00F & op == 0x8003 => println!("'bitwise xor' not implemented!"),
        op if 0xF00F & op == 0x8002 => println!("'bitwise and' not implemented!"),
        op if 0xF00F & op == 0x8001 => println!("'bitwise or' not implemented!"),
        op if 0xF00F & op == 0x8000 => set_reg_reg(op, cpu),
        op if 0xF000 & op == 0x7000 => add_const_to_reg(op, cpu),
        op if 0xF000 & op == 0x6000 => set_reg_const(op, cpu),
        op if 0xF00F & op == 0x5000 => println!("'jeq reg' not implemented!"),
        op if 0xF000 & op == 0x4000 => println!("'jeq const' not implemented!"),
        op if 0xF000 & op == 0x3000 => println!("'jneq const' not implemented!"),
        op if 0xF000 & op == 0x2000 => println!("'call at addr' not implemented!"),
        op if 0xF000 & op == 0x1000 => println!("'jump' not implemented!"),
        op if 0xF000 & op == 0x0000 => println!("'machine code routine' not implemented!"),
        _ => println!("Opcode not recognized")
    }

}

fn main() {
    let mut cpu = init_cpu();
    let memory: [u8; 4096] = [0; 4096];
    println!("Memory initiated");
    handle_opcode(0xFFFF, &mut cpu);
    // handle_opcode(0x00E0, &mut cpu);
    // handle_opcode(0x00EE, &mut cpu);
    // handle_opcode(0x204E, &mut cpu);
    // handle_opcode(0x314E, &mut cpu);
    // handle_opcode(0x43AE, &mut cpu);
    // handle_opcode(0x53B0, &mut cpu);
    // handle_opcode(0x63B2, &mut cpu);
    // handle_opcode(0x7FB2, &mut cpu);
    // handle_opcode(0x8AB0, &mut cpu);
    // handle_opcode(0x8281, &mut cpu);
    // handle_opcode(0x8F12, &mut cpu);
    // handle_opcode(0x8E43, &mut cpu);
    // handle_opcode(0x8E44, &mut cpu);
    // handle_opcode(0x8E45, &mut cpu);
    // handle_opcode(0x8E46, &mut cpu);
    // handle_opcode(0x8E47, &mut cpu);
    // handle_opcode(0x8E4E, &mut cpu);
    // handle_opcode(0x9E40, &mut cpu);
    // handle_opcode(0xAE40, &mut cpu);
    // handle_opcode(0xBE40, &mut cpu);
    // handle_opcode(0xCE40, &mut cpu);
    // handle_opcode(0xDE40, &mut cpu);
    // handle_opcode(0xEA9E, &mut cpu);
    // handle_opcode(0xEBA1, &mut cpu);
    // handle_opcode(0xFB07, &mut cpu);
    // handle_opcode(0xFB0A, &mut cpu);
    // handle_opcode(0xFB15, &mut cpu);
    // handle_opcode(0xFB18, &mut cpu);
    // handle_opcode(0xFB1E, &mut cpu);
    // handle_opcode(0xFB29, &mut cpu);
    // handle_opcode(0xFB33, &mut cpu);
    // handle_opcode(0xFB55, &mut cpu);
    // handle_opcode(0xFB65, &mut cpu);
}

#[cfg(test)]
mod tests {
    use super::*;
    // #[test]
    // fn test_machine_code_routine_opcode() {
    //     handle_opcode(0x0132);
    // }
    // #[test]
    // fn test_jump_opcode() {
    //     handle_opcode(0x10EE);
    // }
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
}
