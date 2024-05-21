use std::rc::Rc;

use crate::gb::Memory;

pub struct Ppu {
    pub memory: Rc<Memory>
}

impl Ppu {
    pub fn new(memory: Rc<Memory>) -> Self {
        Self {
            memory
        }
    }
}
