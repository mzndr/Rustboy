//! Memory Bank Controller implementation.

pub const MBC_TYPE_OFFSET: usize = 0x0147;
pub const MBC_ROM_SIZE_OFFSET: usize = 0x0148;
pub const MBC_RAM_SIZE_OFFSET: usize = 0x0149;
pub const ROM_TITLE_OFFSET: usize = 0x0134;

pub mod mbc_0;
pub mod mbc_1;

pub trait MBC {
    fn read_rom(&self, address: u16) -> u8;
    fn read_ram(&self, address: u16) -> u8;
    fn write_rom(&mut self, address: u16, val: u8);
    fn write_ram(&mut self, address: u16, val: u8);
}

pub struct ROM {

    pub data: Vec<u8>,
}

pub fn load_cartridge(rom: &[u8]) -> Box<dyn MBC> {
    assert!(rom.len() > MBC_TYPE_OFFSET);

    let mbc_type = rom[MBC_TYPE_OFFSET];
    let rom_size = rom[MBC_ROM_SIZE_OFFSET];
    let ram_size = rom[MBC_RAM_SIZE_OFFSET];

    match mbc_type {
        mbc_0::ID => Box::new(mbc_0::MBC0::new(rom)),
        mbc_1::ID => Box::new(mbc_1::MBC1::new(rom, rom_size, ram_size)),
        _ => panic!("Unsupported MBC '0x{mbc_type:x}'"),
    }
}
