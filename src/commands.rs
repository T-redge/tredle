use crate::cpu::*;
#[derive(Debug, PartialEq)]
pub enum Commands{
    Unknown,
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
    SwapN,
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
pub enum CbPrefix {
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
        0x28 => cpu.get_flag('z'),
        _ => panic!("Error: Unknown opcode!"),
    };
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
    let nn = cpu.get_imm16() as usize;
    let a = cpu.get_r8('a');

    cpu.set_memory(nn, a);
}
pub fn load_a_n(cpu: &mut Cpu) {
    let n = match cpu.opcode {
        0x3E => cpu.get_imm8(),
        _ => panic!("Error: Unknown opcode!"),
    };
    cpu.set_r8('a', n);
}