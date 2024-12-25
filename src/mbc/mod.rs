//! Memory Bank Controller implementation.

const MBC_TYPE_OFFSET: usize = 0x0147;
const MBC_ROM_SIZE_OFFSET: usize = 0x0148;
const MBC_RAM_SIZE_OFFSET: usize = 0x0149;
const ROM_TITLE_OFFSET: usize = 0x0134;

mod mbc_0;
mod mbc_1;

pub trait MBC {
    fn read_rom(&self, address: u16) -> u8;
    fn read_ram(&self, address: u16) -> u8;
    fn write_rom(&mut self, address: u16, val: u8);
    fn write_ram(&mut self, address: u16, val: u8);
}

pub fn load_cartridge(rom: &[u8]) -> Box<dyn MBC> {
    assert!(rom.len() > MBC_TYPE_OFFSET);
    let mbc_type = rom[MBC_TYPE_OFFSET];
    match mbc_type {
        mbc_0::ID => Box::new(mbc_0::MBC0::new(rom)),
        mbc_1::ID => Box::new(mbc_1::MBC1::new(rom)),
        _ => panic!("Unsupported MBC '0x{mbc_type:x}'"),
    }
}

fn num_rom_banks(rom: &[u8]) -> u8 {
    let rom_size = rom[MBC_ROM_SIZE_OFFSET];
    match rom_size {
        0x00 => 0x02,
        0x01 => 0x04,
        _ => panic!("unknown rom size identifier"),
    }
}
