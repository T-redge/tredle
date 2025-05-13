use crate::{commands::{*, Commands}, load_rom};
#[derive(Default)]
struct Flags {
    z: u8,
    n: u8,
    h: u8,
    c: u8,
}
impl Flags {
    pub fn debug_flags(&mut self) {
        println!(
            "[Z: {}, N: {}, H: {}, C: {}]",
            self.z, self.n, self.h, self.c
        );
    }
}
struct R8 {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
}
impl Default for R8 {
    fn default() -> Self {
        Self {
            a: 0x00,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,

        }
    }
}
impl R8 {
    pub fn debug_r8(&mut self) {
        println!(
            "[A: {:#X}, B: {:#X}, C: {:#X}, D: {:#X}, E: {:#X}, H: {:#X}, L: {:#X}]",
                    self.a, self.b, 
                    self.c, self.d, self.e, 
                    self.h, self.l);
    }
}
pub struct Cpu {
    rom: Vec<u8>,
    pub opcode: u8,
    p_c: usize,
    cmd: Commands,
    r8: R8,
    flags: Flags,
    pub increment_flag: bool,
}
impl Default for Cpu {
    fn default() -> Self {
        Self {
            rom: load_rom("rom/pkm_blue.gb".to_string()),
            opcode: 0x00,
            p_c: 0x100,
            cmd: Commands::Unknown,
            r8: R8::default(),
            flags: Flags::default(),
            increment_flag: true,
        }
    }
}
//Public Functions
impl Cpu {
    pub fn cycle(&mut self) {
        let mut l_c = 0;
        
        loop {
            if l_c > 10 {
                break;
            }
            println!("\nLoop: {}", l_c);
            self.get_opcode();
            self.debug_cpu();
            self.process_opcode();
            self.execute_cmd();
            self.increment_pc();
            l_c += 1;
        }
    }
    pub fn get_imm8(&mut self) -> u8 {
        let imm8 = self.rom[self.p_c + 1];
        if self.opcode != 0x18 | 0x28 | 0xC3 {
            self.p_c += 1;
        }
        imm8
    }
    pub fn get_imm16(&mut self) -> u16 {
        let imm16 = (self.rom[self.p_c + 1] as u16) | (self.rom[self.p_c + 2] as u16) << 8;
        self.p_c += 2;
        imm16
    }
    pub fn set_pc(&mut self, value: u16) {
        self.p_c = value as usize;
    }
    pub fn get_pc(&mut self) -> u16 {
        self.p_c as u16
    }
    pub fn set_r8(&mut self, reg: char, value: u8) {
        match reg {
            'a' => self.r8.a = value,
            'b' => self.r8.b = value,
            'c' => self.r8.c = value,
            'd' => self.r8.d = value,
            'e' => self.r8.e = value,
            'h' => self.r8.h = value,
            'l' => self.r8.l = value,
            _ => panic!("Error: Unknown register!"),
        }
    }pub fn get_r8(&mut self, reg: char) -> u8 {
        match reg {
            'a' => self.r8.a,
            'b' => self.r8.b,
            'c' => self.r8.c,
            'd' => self.r8.d,
            'e' => self.r8.e,
            'h' => self.r8.h,
            'l' => self.r8.l,
            _ => panic!("Error: Unknown register!"),
        }
    }
    pub fn set_r16(&mut self, reg: &str, value: u16) {
        let reg_one = (value & 0xFF00) as u8;
        let reg_two = (value & 0x00FF) as u8;
        match reg {
            "bc" => {
                self.r8.b = reg_one;
                self.r8.c = reg_two;
            }
            "de" => {
                self.r8.d = reg_one;
                self.r8.e = reg_two;
            }
            "hl" => {
                self.r8.h = reg_one;
                self.r8.l = reg_two;
            }
            _ => panic!("Error: Unknown register!"),
        }
    }
    pub fn get_flag(&mut self, flag: char) -> u8 {
        match flag {
            'z' => self.flags.z,
            'n' => self.flags.n,
            'h' => self.flags.h,
            'c' => self.flags.c,
            _ => panic!("Error: Unknown flag!"),
        }
    }
    pub fn set_flags(&mut self, f_value: (u8,u8,u8,u8)) {
        self.flags.z = f_value.0;
        self.flags.n = f_value.1;
        self.flags.h = f_value.2;
        self.flags.c = f_value.3;
    }
    pub fn set_memory(&mut self,address: usize, value: u8) {
        self.rom[address] = value;
    }
}



//Private Functions
impl Cpu {
    fn get_opcode(&mut self) {
        self.opcode = self.rom[self.p_c];
    }
    fn debug_cpu(&mut self) {
        println!("Program Counter: {:#X}", self.p_c);
        println!("Opcode: {:#X}", self.opcode);
        println!("Imm8: {:#X}", self.rom[self.p_c + 1]);
        println!("Imm16: {:#X}", self.rom[self.p_c + 2]);
        self.r8.debug_r8();
        self.flags.debug_flags();
    }
    fn process_opcode(&mut self) {
        self.cmd = match self.opcode{
            0x0 => Commands::Nop,
            0x11 => Commands::LoadnNN,
            0x18 => Commands::JrN,
            0x28 => Commands::JrCCn,
            0x3E => Commands::LoadAn,
            0xAF => Commands::XorN,
            0xC3 => Commands::JumpNN,
            0xEA => Commands::LoadnA,
            0xFE => Commands::CpN,
            _ => Commands::Unknown,
        };
    }
    fn execute_cmd(&mut self) {
        self.cmd.print();
        match self.cmd {
            Commands::Nop => nop(),
            Commands::LoadnNN => load_n_nn(self),
            Commands::JrN => jr_n(self),
            Commands::JrCCn => jr_cc_n(self),
            Commands::LoadAn => load_a_n(self),
            Commands::XorN => xor_n(self),
            Commands::JumpNN => jump_nn(self),
            Commands::LoadnA => load_n_a(self),
            Commands::CpN => cp_n(self),
            Commands::Unknown => panic!("Error: Unkown command!"),
            _ => panic!("Error: Unknown command!"),
        }
    }
    fn increment_pc(&mut self) {
        if self.increment_flag {
            self.p_c += 1;
        } else {
            self.increment_flag = true;
        }
    }
}