
#[allow(non_snake_case)]
#[allow(dead_code)]
struct CPU {
    V0: u8,
    V1: u8,
    V2: u8,
    V3: u8,
    V4: u8,
    V5: u8,
    V6: u8,
    V7: u8,
    V8: u8,
    V9: u8,
    VA: u8,
    VB: u8,
    VC: u8,
    VD: u8,
    VE: u8,
    VF: u8, // Doubles as flag for some instructions
    I: u16, // Address register
    PC: u16 // Program counter
}


#[allow(non_snake_case)]
fn init_cpu() -> CPU {
    let cpu = CPU {
        V0: 0,
        V1: 0,
        V2: 0,
        V3: 0,
        V4: 0,
        V5: 0,
        V6: 0,
        V7: 0,
        V8: 0,
        V9: 0,
        VA: 0,
        VB: 0,
        VC: 0,
        VD: 0,
        VE: 0,
        VF: 0,
        I: 0,
        PC: 0
    };
    println!("CPU initiated");
    return cpu;
}

fn handle_opcode(opcode: u16) {
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
        op if 0xF00F & op == 0x8000 => println!("'set reg to reg' not implemented!"),
        op if 0xF000 & op == 0x7000 => println!("'add const to reg' not implemented!"),
        op if 0xF000 & op == 0x6000 => println!("'set reg to const' not implemented!"),
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
    let cpu = init_cpu();
    let memory: [u8; 4096] = [0; 4096];
    println!("Memory initiated");
    handle_opcode(0x00E0);
    handle_opcode(0x00EE);
    handle_opcode(0x204E);
    handle_opcode(0x314E);
    handle_opcode(0x43AE);
    handle_opcode(0x53B0);
    handle_opcode(0x63B2);
    handle_opcode(0x7FB2);
    handle_opcode(0x8AB0);
    handle_opcode(0x8281);
    handle_opcode(0x8F12);
    handle_opcode(0x8E43);
    handle_opcode(0x8E44);
    handle_opcode(0x8E45);
    handle_opcode(0x8E46);
    handle_opcode(0x8E47);
    handle_opcode(0x8E4E);
    handle_opcode(0x9E40);
    handle_opcode(0xAE40);
    handle_opcode(0xBE40);
    handle_opcode(0xCE40);
    handle_opcode(0xDE40);
    handle_opcode(0xEA9E);
    handle_opcode(0xEBA1);
    handle_opcode(0xFB07);
    handle_opcode(0xFB0A);
    handle_opcode(0xFB15);
    handle_opcode(0xFB18);
    handle_opcode(0xFB1E);
    handle_opcode(0xFB29);
    handle_opcode(0xFB33);
    handle_opcode(0xFB55);
    handle_opcode(0xFB65);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_machine_code_routine_opcode() {
        handle_opcode(0x0132);
    }
    #[test]
    fn test_jump_opcode() {
        handle_opcode(0x10EE);
    }
}
