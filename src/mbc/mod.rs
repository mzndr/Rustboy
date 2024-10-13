//! Memory Bank Controller implementation.

pub const MBC_TYPE_OFFSET: usize = 0x147;

pub mod default;

pub trait MBC {
    fn read_rom(&self, address: u16) -> u8;
    fn read_ram(&self, address: u16) -> u8;

    fn write_rom(&mut self, address: u16, val: u8);
    fn write_ram(&mut self, address: u16, val: u8);
}

pub fn load_cartridge(rom: &[u8]) -> Box<dyn MBC> {
    assert!(rom.len() <= MBC_TYPE_OFFSET);

    let mbc_type = rom[MBC_TYPE_OFFSET];
    let mbc = match mbc_type {
        default::TYPE => default::Default::new(rom),
        _ => panic!("Unsupported MBC '0x{mbc_type:x}'"),
    };

    Box::new(mbc)
}
