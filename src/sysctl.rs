pub const SYSCTL_BASE_ADDR: usize = 0x50440000;

#[repr(C)]
pub struct SysCtl {
    git_id: u32, //なんぞこれ
    clock_freq: u32,
    pll0_ctl: u32,
    pll1_ctl: u32,
    pll2_ctl: u32,
    reserved0: u32,
    pll_lock: u32,
    rom_error: u32,
    clk_sel0: u32,
    clk_sel1: u32,
    clk_en_cent: u32,
    clk_en_peri: u32,
    soft_reset: u32,
    peri_reset: u32,
    clk_th0: u32,
    clk_th1: u32,
    clk_th2: u32,
    clk_th3: u32,
    clk_th4: u32,
    clk_th5: u32,
    clk_th6: u32,
    misc: u32,
    peri: u32,
    spi_sleep: u32,
    reset_status: u32,
    dma_sel0: u32,
    dma_sel1: u32,
    power_sel: u32,
}

impl SysCtl {
    pub fn enable_uart3_clock_bus(&mut self) {
        // https://github.com/sipeed/kendryte-standalone-sdk/blob/Maixpy-dev/lib/drivers/sysctl.c より
        self.clk_en_cent |= 1 << 3;
    }

    pub fn enable_uart3_clock(&mut self) {
        self.clk_en_peri |= 1 << 18;
    }

    pub fn enable_fpioa_clock(&mut self) {
        self.clk_en_cent |= 1 << 3;
        self.clk_en_peri |= 1 << 20;
    }

    pub fn get_apb0_clock_freq(&mut self) -> u32 {
        let clock_select = self.clk_sel0 & 1;
        let source = match clock_select {
            0 => 26000000,
            1 => {
                let freq_in = 26000000f64;
                let nr = ((self.pll0_ctl & 0x000F) >> 0 + 1) as f64;
                let nf = ((self.pll0_ctl & 0x03F0) >> 4 + 1) as f64;
                let od = ((self.pll0_ctl & 0x3C00) >> 10 + 1) as f64;
                let freq_pll0 = (freq_in / nr * nf / od) as u32;
                let aclk_divider_sel = (self.clk_sel0 >> 1) & 0x03;
                let divider_sel = 2 << aclk_divider_sel;
                (freq_pll0 / divider_sel)
            }
            _ => 0,
        };
        return source / (((self.clk_sel0 >> 3) & 7) + 1);
    }
}
