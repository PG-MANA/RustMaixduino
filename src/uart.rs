// https://github.com/sipeed/kendryte-standalone-sdk/blob/cbcf157696ef8aa0d6220fe96eae5557087d873c/lib/bsp/include/platform.h
// https://github.com/sipeed/kendryte-standalone-sdk/blob/cbcf157696ef8aa0d6220fe96eae5557087d873c/lib/drivers/uarths.c
// http://kendryte-docs.s3-website.cn-north-1.amazonaws.com.cn/docs/standalone_programming_guide/en/UARTHS.html#api
use super::fpioa::{Fpioa, FPIOA_BASE_ADDR};
use super::sysctl::{SysCtl, SYSCTL_BASE_ADDR};

pub const UART3_BASE_ADDR: usize = 0x50230000;

#[repr(C)]
union Union1 {
    RBR: u32,
    DLL: u32,
    THR: u32,
}

#[repr(C)]
union Union2 {
    DLH: u32,
    IER: u32,
}

#[repr(C)]
union Union3 {
    FCR: u32,
    IIR: u32,
}

#[repr(C)]
union Union4 {
    SRBR: [u32; 16],
    STHR: [u32; 16],
}
#[repr(C)]
pub struct Uart {
    union1: Union1,
    union2: Union2,
    union3: Union3,

    LCR: u32,
    MCR: u32,
    LSR: u32,
    MSR: u32,
    SCR: u32,
    LPDLL: u32,
    LPDLH: u32,
    RSV0: u32,
    RSV1: u32,

    union4: Union4,

    FAR: u32,
    TFR: u32,
    RFW: u32,
    USR: u32,
    TFL: u32,
    RFL: u32,
    SRR: u32,
    SRTS: u32,
    SBCR: u32,
    SDMAM: u32,
    SFE: u32,
    SRT: u32,
    STET: u32,
    HTX: u32,
    DMASA: u32,
    TCR: u32,
    DE_EN: u32,
    RE_EN: u32,
    DET: u32,
    TAT: u32,
    DLF: u32,
    RAR: u32,
    TAR: u32,
    LCR_EXT: u32,
    RSV3: [u32; 9],
    CPR: u32,
    UCV: u32,
    CTR: u32,
}

impl Uart {
    pub fn init_uart3(&mut self) {
        let sys_ctl = unsafe { &mut *(SYSCTL_BASE_ADDR as *mut SysCtl) };
        let fpioa = unsafe { &mut *(FPIOA_BASE_ADDR as *mut Fpioa) };

        sys_ctl.enable_uart3_clock_bus();
        sys_ctl.enable_uart3_clock();

        let freq = sys_ctl.get_apb0_clock_freq();
        let divisor = freq / 115200;
        let dlh = divisor >> 12;
        let dll = (divisor - (dlh << 12)) / 16;
        let dlf = divisor - (dlh << 12) - dll * 16;
        self.LCR |= 1 << 7;
        self.union2.DLH = dlh;
        self.union1.DLL = dll;
        self.DLF = dlf;
        self.LCR = 0;
        self.LCR = 3;
        self.LCR &= 0xFFFFFF7F;
        unsafe {
            self.union2.IER |= 0x80;
            self.union3.FCR = (0 << 6) | (3 << 4) | (1 << 3) | 1;
        }
        self.SRT = 0;
        fpioa.enable_uart3();
        Fpioa::init();
    }

    pub fn getc(&self) -> u8 {
        while unsafe { core::ptr::read_volatile(&(self.LSR) as *const _) & 1u32 } == 0u32 {}
        return (unsafe { core::ptr::read_volatile(&(self.union1.RBR)) } & 0xff) as u8;
    }

    pub fn putc(&mut self, c: u8) {
        while ((self.LSR & (1 << 5)) != 0) {}
        while unsafe { core::ptr::read_volatile(&(self.LSR) as *const _) & (1u32 << 5) } != 0u32 {}
        unsafe { core::ptr::write_volatile(&mut (self.union1.THR) as *mut _ as *mut u8, c) };
    }

    pub fn puts(&mut self, s: &str) {
        for c in s.as_bytes() {
            self.putc(*c);
        }
    }
}
