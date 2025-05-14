use crate::cpu::*;
#[derive(Debug, PartialEq)]
pub enum Commands{
    Unknown,
    Cb,
    LoadNNn,
    LoadR1R2,
    LoadAn,
    LoadnA,
    LoadAc,
    LoadCa,
    LoaddAhl,
    LoaddHLa,
    LoadiAhl,
    LoadiHLa,
    LoadhNa,
    LoadhAn,
    LoadnNN,
    LoadSPhl,
    LoadHLspN,
    LoadNNsp,
    PushNN,
    PopNN,
    AddAn,
    AdcAn,
    SubN,
    SbcAn,
    AndN,
    OrN,
    XorN,
    CpN,
    IncN,
    DecN,
    AddHLn,
    AddSPn,
    IncNN,
    DecNN,
    Daa,
    Cpl,
    Ccf,
    Scf,
    Nop,
    Halt,
    Stop,
    Di,
    Ei,
    Rlca,
    Rla,
    Rrca,
    Rra,
    JumpNN,
    JumpCCnn,
    JumpHL,
    JrN,
    JrCCn,
    CallNN,
    CallCCnn,
    RstN,
    Ret,
    RetCC,
    Reti,
}
impl Commands {
    pub fn print(&self) {
        println!("{:#?}", self);
    }
}
#[derive(Debug)]
pub enum CbPrefix {
    Unknown,
    RlcN,
    RlN,
    RrcN,
    RrN,
    SlaN,
    SraN,
    SrlN,
    BitBr,
    SetBr,
    ResBr,
    SwapN,
}
impl CbPrefix {
    pub fn print(&self) {
        println!("{:#?}", self);
    }
}

pub fn nop(){}
pub fn jump_nn(cpu: &mut Cpu) {
    let imm16 = cpu.get_imm16();
    cpu.set_pc(imm16);
    cpu.increment_flag = false;
    
}
pub fn load_n_nn(cpu: &mut Cpu) {
    let imm16 = cpu.get_imm16();

    cpu.set_r16("de", imm16);
}
pub fn cp_n(cpu: &mut Cpu) {
    let mut flags = (0,1,0,0);
    let a = cpu.get_r8('a');
    let n = match cpu.opcode {
        0xFE => cpu.get_imm8(),
        _ => panic!("Error: Unknown opcode!"),
    };
    if a == n {
        flags.0 = 1;
    } 
    if a < n {
        flags.3 = 1;
    }
    if flags.3 == 1 {
        flags.2 = 1;
    }

    cpu.set_flags(flags);
}
pub fn jr_cc_n(cpu: &mut Cpu) {
    let address = cpu.get_pc();
    let n = address + cpu.get_imm8() as u16;
    let cc = match cpu.opcode {
        0x20|0x28 => cpu.get_flag('z'),
        _ => panic!("Error: Unknown opcode!"),
    };
    if (cc == 0) && (cpu.opcode == 0x20) {
        cpu.set_pc(n);
        cpu.increment_flag = false;
    }
    if (cc == 1) && (cpu.opcode == 0x28) {
        cpu.set_pc(n);
        cpu.increment_flag = false;
    }
}
pub fn xor_n(cpu: &mut Cpu) {
    let mut flags = (0,0,0,0);
    let n = match cpu.opcode {
        0xAF => cpu.get_r8('a'),
        _ => panic!("Error: Unknown opcode!"),
    };
    let a = cpu.get_r8('a');

    let result = n ^ a;

    if result == 0 {
        flags.0 = 1;
    }

    cpu.set_r8('a', result);
    cpu.set_flags(flags);

}
pub fn jr_n(cpu: &mut Cpu) {
    let p_c = cpu.get_pc();
    let n = cpu.get_imm8() as u16;
    

    let address = n + p_c;

    cpu.set_pc(address);
    cpu.increment_flag = false;
}
pub fn load_n_a(cpu:&mut Cpu) {
    let a = cpu.get_r8('a');
    match cpu.opcode {
        0x47 => cpu.set_r8('c', a),
        0xEA => {
            let nn = cpu.get_imm16() as usize;
            cpu.set_memory(nn, a);
        }
        _ => panic!("Error: Unknown opcode!"),
    }
    
}
pub fn load_a_n(cpu: &mut Cpu) {
    let n = match cpu.opcode {
        0x3E => cpu.get_imm8(),
        _ => panic!("Error: Unknown opcode!"),
    };
    cpu.set_r8('a', n);
}
pub fn di(cpu: &mut Cpu) {
    cpu.set_ime_flag();
}
pub fn loadh_n_a(cpu: &mut Cpu) {
    let a = cpu.get_r8('a');

    let n = cpu.get_imm8() as u16;
    let address = (0xFF00 + n) as usize;
    // May have to increment P_C after a write
    cpu.set_memory(address, a);
}
pub fn call_nn(cpu: &mut Cpu) {
    let push = cpu.get_pc() + 1;
    let nn = cpu.get_imm16();

    cpu.push_to_stack(push);

    cpu.set_pc(nn);
    cpu.increment_flag = false;
}
pub fn loadh_a_n(cpu: &mut Cpu) {
    let n = cpu.get_imm8() as u16;
    let address = (0xFF00 + n) as usize;

    let load = cpu.get_memory(address);
    //May have to increment P_C after write
    cpu.set_r8('a', load);
}
pub fn res_b_r(cpu: &mut Cpu) {
    let p_c = cpu.get_pc() as usize;
    let opcode = cpu.get_memory(p_c + 1);
    let reg = match opcode {
        0x87 => cpu.get_r8('a'),
        _ => panic!("Error: Unknown opcode!"),
    };
    let bit = opcode & 0x38;
    
    let result = reg & (!(1 << bit));

    match opcode {
         0x87 => cpu.set_r8('a', result),
        _ => panic!("Error: Unknown opcode!"),
    }
    cpu.set_pc((p_c+1) as u16);
    
}
pub fn cpl(cpu: &mut Cpu) {
    let f_z = cpu.get_flag('z');
    let f_c = cpu.get_flag('c');
    let a = cpu.get_r8('a');
    let value = a ^ 0xFF;
    cpu.set_flags((f_z,1,1,f_c));
    cpu.set_r8('a', value);
}
pub fn and_n(cpu: &mut Cpu) {
    let mut flags = (0,0,1,0);
    let n = cpu.get_imm8();
    let a = cpu.get_r8('a');

    let result = a & n;
    if result == 0 {
        flags.0 = 1;
    }

    cpu.set_flags(flags);
    cpu.set_r8('a', result);
}
pub fn swap_n(cpu: &mut Cpu) {
    let p_c = cpu.get_pc() as usize;
    let reg = match cpu.get_memory(p_c + 1) {
        0x37 => cpu.get_r8('a'),
        _ => panic!("Error: Unknown Opcode!"),
    };

    
}