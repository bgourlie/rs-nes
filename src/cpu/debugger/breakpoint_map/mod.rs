#[cfg(test)]
mod spec_tests;

pub struct BreakpointMap {
    map: [u8; 8192],
}

impl BreakpointMap {
    pub fn new() -> Self {
        BreakpointMap { map: [0; 8192] }
    }

    pub fn toggle(&mut self, addr: u16) -> bool {
        let i = addr as usize / 8;
        let bit_pos = addr % 8;
        let cur_byte = self.map[i];
        let mask = 1 << bit_pos;
        self.map[i] = cur_byte ^ mask;
        self.map[i] & mask > 0
    }

    pub fn is_set(&self, addr: u16) -> bool {
        let i = addr as usize / 8;
        let bit_pos = addr % 8;
        let mask: u8 = 1 << bit_pos;
        self.map[i] & mask > 0
    }
}

impl Default for BreakpointMap {
    fn default() -> Self {
        BreakpointMap::new()
    }
}
