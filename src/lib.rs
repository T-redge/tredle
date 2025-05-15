use std::io::Read;

pub mod commands;
pub mod cpu;

pub fn load_rom(path: String) -> Vec<u8>{
    let mut file = std::fs::File::open(path).unwrap();
    let mut tmp_vec: Vec<u8> = Vec::new();

    file.read_to_end(&mut tmp_vec).unwrap();

    tmp_vec
}
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