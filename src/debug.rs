use std::{ops::Deref, sync::Arc};

use crate::cpu::disassembler::disassemble_rom;

#[derive(Debug, Clone)]
pub struct Debug {
    pub inner: Arc<Inner>,
}

#[derive(Debug)]
pub struct Inner {
    pub gb_doc_enable: bool,
    pub rom: Vec<u8>,
    pub disassembled_rom: Vec<String>,
}

impl Debug {
    pub fn new(rom: &[u8], gbd_enable: bool) -> Self {
        return Self {
            inner: Arc::new(Inner::new(rom, gbd_enable)),
        };
    }

    pub fn disassembly_get_range(&self, start: u16, stop: u16) -> Vec<String> {
        let mut ret = Vec::new();
        for i in start..stop {
            ret[i as usize] = self.disassembled_rom[i as usize].clone()
        }
        ret
    }
}

impl Inner {
    pub fn new(rom: &[u8], gbd_enable: bool) -> Self {
        return Self {
            gb_doc_enable: gbd_enable,
            rom: Vec::from(rom),
            disassembled_rom: disassemble_rom(rom),
        };
    }
}

impl Deref for Debug {
    type Target = Inner;
    fn deref(&self) -> &Self::Target {
        return &self.inner;
    }
}
