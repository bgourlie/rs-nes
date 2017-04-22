pub trait Dmc: Default {
    fn write_flags_and_rate_reg(&mut self, val: u8);
    fn write_direct_load_reg(&mut self, val: u8);
    fn write_sample_addr_reg(&mut self, val: u8);
    fn write_sample_len_reg(&mut self, val: u8);
}

#[derive(Default)]
pub struct DmcImpl {
    flags_and_rate_reg: u8,
    direct_load_reg: u8,
    sample_addr_reg: u8,
    sample_len_reg: u8,
}

impl Dmc for DmcImpl {
    fn write_flags_and_rate_reg(&mut self, val: u8) {
        self.flags_and_rate_reg = val
    }
    fn write_direct_load_reg(&mut self, val: u8) {
        self.direct_load_reg = val
    }

    fn write_sample_addr_reg(&mut self, val: u8) {
        self.sample_addr_reg = val
    }

    fn write_sample_len_reg(&mut self, val: u8) {
        self.sample_len_reg = val
    }
}
