use std::io::Read;

pub mod commands;
pub mod cpu;

pub fn load_rom(path: String) -> Vec<u8>{
    let mut file = std::fs::File::open(path).unwrap();
    let mut tmp_vec: Vec<u8> = Vec::new();

    file.read_to_end(&mut tmp_vec).unwrap();

    tmp_vec
}
