use super::sysctl::{SysCtl, SYSCTL_BASE_ADDR};

const NUM_FPIOA_IO: usize = 48;
pub const FPIOA_BASE_ADDR: usize = 0x502B0000;

#[repr(C, packed)]
pub struct Fpioa {
    fpioa_config: [u32; NUM_FPIOA_IO],
    fpoia_tie: [u32; 256 / 32],
    fpoia_val: [u32; 256 / 32],
}

impl Fpioa {
    pub fn enable_uart3(&mut self) {
        self.fpioa_config[4] = 0x900044;
        self.fpioa_config[5] = 0x1f45;
    }

    pub fn init() {
        let sys_ctl = unsafe { &mut *(SYSCTL_BASE_ADDR as *mut SysCtl) };
        sys_ctl.enable_fpioa_clock();
    }
}
