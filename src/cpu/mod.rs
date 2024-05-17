use self::registers::Registers;
mod extended_instructions;
mod instructions;
pub mod disassembler;

pub mod registers;
pub mod utils;

/**
* Emulating the LR35902 CPU
*
* For Opcodes see: <https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html>
*/

/** Working RAM **/
// TODO: Fixen, es ist nicht WRAM sondern der gesammte memory.
const WRAM_SIZE: usize = 0x10000; //0x20 * 0x400;
type WRam = [u8; WRAM_SIZE];

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Cpu {
    pub registers: Registers,

    busy_for: u8,
    wram: WRam,
}

impl Cpu {
    /// Checks if an address is in valid space,
    /// prints an error message and quits if not.
    fn check_address(address: u16) {
        assert!((address as usize).lt(&WRAM_SIZE));
    }

    pub fn print_wram(&self, from: usize, to: usize) {
        let string = String::from_utf8_lossy(&self.wram[from..to]).to_ascii_lowercase();
        tracing::info!("{}", string);
    }

    /// Needs to be changed for bigger games, since they
    /// are too big to fit into ram, so banking has to be
    /// implemented.
    pub fn load_rom(&mut self, rom: &[u8]) {
        for (address, byte) in rom.iter().enumerate() {
            self.wram[address] = *byte;
        }
    }

    /// Initialize cpu memory
    pub fn new() -> Cpu {
        tracing::info!("initializing cpu");
        Cpu {
            registers: registers::Registers::new(),
            wram: [0x00; WRAM_SIZE],
            busy_for: 0x00,
        }
    }

    // Execute a machine cycle.
    #[tracing::instrument(skip(self), fields(regs = %self.registers))]
    pub fn cycle(&mut self) {
        if self.busy_for == 0 {
            self.busy_for = self.exec_instruction();
            //self.print_wram(0x0026, 0x003a);
        } else {
            self.busy_for -= 1;
        }
    }

    /// Push a u8 value onto the stack.
    pub fn push_stack_u8(&mut self, val: u8) {
        self.registers.sp = self.registers.sp.wrapping_sub(1);
        self.write_u8(self.registers.sp, val);
    }

    /// Pop a u8 value from the stack.
    pub fn pop_stack_u8(&mut self) -> u8 {
        let val = *self.read(self.registers.sp);
        self.registers.sp = self.registers.sp.wrapping_add(1);
        val
    }

    /// Push a u16 value onto the stack.
    pub fn push_stack_u16(&mut self, val: u16) {
        let (l, h) = utils::split_u16(val);
        self.push_stack_u8(h);
        self.push_stack_u8(l);
    }

    /// Pop a u16 value from the stack.
    pub fn pop_stack_u16(&mut self) -> u16 {
        let l = self.pop_stack_u8();
        let h = self.pop_stack_u8();
        utils::merge_u8s(l, h)
    }

    /// Reads from wram at address.
    pub fn read_mut(&mut self, address: u16) -> &mut u8 {
        let u_addr = address as usize;
        Self::check_address(address);
        &mut self.wram[u_addr]
    }

    /// Reads from wram at address.
    pub fn read(&self, address: u16) -> &u8 {
        let u_addr = address as usize;
        Self::check_address(address);
        &self.wram[u_addr]
    }

    /// Writes u8 to wram at address.
    pub fn write_u8(&mut self, address: u16, val: u8) {
        let u_addr = address as usize;
        Self::check_address(address);
        self.wram[u_addr] = val;
    }

    /// Writes u16 to wram at address.
    pub fn write_u16(&mut self, address: u16, val: u16) {
        let split = utils::split_u16(val);
        self.write_u8(address, split.1);
        self.write_u8(address + 1, split.0);
    }

    /// Reads a byte from wram at pc and increases pc by one.
    pub fn read_u8_at_pc_and_increase(&mut self) -> u8 {
        let val = *self.read(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        val
    }

    /// Reads a byte from wram at pc.
    pub fn read_u8_at_pc(&self) -> u8 {
        let val = *self.read(self.registers.pc);
        val
    }

    /// Reads two bytes from wram at pc.
    pub fn read_u16_at_pc(&self) -> u16 {
        let a = self.read_u8_at_pc();
        let b = self.read_u8_at_pc();
        // Little endian in memory
        utils::merge_u8s(b, a)
    }

    /// Reads two bytes from wram at pc and increases pc by two.
    pub fn read_u16_at_pc_and_increase(&mut self) -> u16 {
        let a = self.read_u8_at_pc_and_increase();
        let b = self.read_u8_at_pc_and_increase();

        // Little endian in memory
        utils::merge_u8s(b, a)
    }

    /// Check for u8 half carries on additions. (carry from 3rd to 4th bit).
    pub fn check_add_u8_hc(left: u8, right: u8) -> bool {
        ((left & 0xf).wrapping_add(right & 0xf)) & 0x10 == 0x10
    }

    /// Check for u8 half carries on additions. (carry from 7th to 8th bit).
    pub fn check_add_u16_hc(left: u16, right: u16) -> bool {
        ((left & 0xff).wrapping_add(right & 0xff)) & 0x100 == 0x100
    }

    /// Check for u8 half carries on subtractions. (carry from 3rd to 4th bit).
    pub fn check_sub_u8_hc(left: u8, right: u8) -> bool {
        ((left & 0xf).wrapping_sub(right & 0xf)) & 0x10 == 0x10
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::Cpu;

    #[test]
    fn test_check_add_u8_hc() {
        assert!(Cpu::check_add_u8_hc(1, 0xF));
        assert!(!Cpu::check_add_u8_hc(1, 0xE));
    }

    #[test]
    fn test_check_add_u16_hc() {
        assert!(Cpu::check_add_u16_hc(0xFF, 1));
        assert!(!Cpu::check_add_u16_hc(0xFE, 1));
    }

    #[test]
    fn test_check_sub_u8_hc() {
        assert!(Cpu::check_sub_u8_hc(1, 0xF));
        assert!(!Cpu::check_sub_u8_hc(0xF, 0xE));
    }
}
