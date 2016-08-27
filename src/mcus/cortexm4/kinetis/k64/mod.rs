extern crate core;
use core::intrinsics::{volatile_copy_nonoverlapping_memory, volatile_set_memory};

pub mod wdog;

mcu!(
    name => K64;
    bootloader_exit => ::main;

    doc_srcs => [
        "http://www.nxp.com/files/microcontrollers/doc/ref_manual/K64P144M120SF5RM.pdf"
    ];

    nvic => ::traits::NVIC {
        addr => 0xE000_E000;
        prio_bits => 4;
    };

    peripherals => {
        wdog => wdog::Watchdog @ 0xE000_E000;
    };
);

impl ::traits::MCU for K64 {
    //
    // ARM core
    //

    /// Fetches the NVIC for this specific MCU.
    fn get_nvic(&self) -> &::traits::NVIC { &self.nvic }


    //
    // memory
    //

    /// Get stack region information.
    fn stack_memory(&self) -> ::libc::memory::IOVec {
        ::libc::memory::IOVec{ptr: K64::STACK_BASE as *const u8, size: (K64::STACK_LIMIT-K64::STACK_BASE) as usize}
    }

    /// Get heap region information.
    fn heap_memory(&self) -> ::libc::memory::IOVec {
        ::libc::memory::IOVec{ptr: K64::HEAP_BASE as *const u8, size: (K64::HEAP_LIMIT-K64::HEAP_BASE) as usize}
    }
}
