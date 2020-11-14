#![no_std]
#![feature(asm)]
#![feature(global_asm)]
#![feature(core_panic_info)]

mod fpioa;
mod sysctl;
mod uart;

//use
use self::uart::{Uart, UART3_BASE_ADDR};
use core::panic;

#[no_mangle]
pub extern "C" fn main(core_id: u8) {
    if (core_id == 0) {
        let text = "Hello,world!\r\nあいうえお\r\n";
        let uart = unsafe { &mut *(UART3_BASE_ADDR as *mut Uart) };
        uart.init_uart3();
        uart.puts(text);
    }
    halt();
}

#[panic_handler]
#[no_mangle]
pub fn panic(_info: &panic::PanicInfo) -> ! {
    halt()
}

fn halt() -> ! {
    loop {
        unsafe { asm!("wfi") };
    }
}

#[no_mangle]
pub fn abort() -> ! {
    halt()
}

#[cfg(target_arch = "riscv64")]
global_asm!(
    r#"
.global _start, stack

.section .text

_start:
lla  sp, stack
csrw mideleg, 0
csrw medeleg, 0
la   t0, abort
csrw mtvec, t0
li   t0, 0x00006000
csrs mstatus, t0
csrw mie, 0
csrw mip, 0
csrr a0, mhartid
j    main

.align 8

.section .bss

.skip 2048
stack:
"#
);
