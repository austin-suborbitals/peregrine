extern crate core;
use core::intrinsics::{volatile_copy_nonoverlapping_memory, volatile_set_memory};

use ::mcus::cortexm4;

pub mod wdog;

extern {
    fn entry(mcu: K64) -> !;
}

mcu!(
    name => K64;
    doc_srcs => [
        "http://www.nxp.com/files/microcontrollers/doc/ref_manual/K64P144M120SF5RM.pdf"
    ];

    link_script => "MK64FN1M0VLL12.ld"; // TODO
    bootloader_exit => entry;

    constants => {
    };

    externs => {
        __stack_begin:  u32;
        __stack_end:    u32;

        __heap_begin:   u32;
        __heap_end:     u32;

        __bss_begin:    u32;
        __bss_end:      u32;
    };

    nvic => ::traits::NVIC {
        addr => 0xE000_E000;
        prio_bits => 4;
    };

    interrupts => [128] @ .interrupt_table {
        // TODO
    };


    memory => {
        stack => {
            base        => __stack_begin;
            limit       => __stack_end;
        };

        heap => {
            base        => __heap_begin;
            limit       => __heap_end;
        };

        data => {
            src_begin   => 0x1000; // TODO
            src_end     => 0x1000; // TODO
            dest        => 0x1000; // TODO
        };

        bss => {
            base        => __bss_begin;
            limit       => __bss_end;
        };
    };


    peripherals => {
        // we initialize the watchdog first
        wdog        => wdog::Watchdog                       @ 0x4005_2000;

        // core modules
        systick     => cortexm4::core::systick::SysTick     @ 0xE000_E010;
        fpu_coproc  => cortexm4::core::fpu::Access          @ 0xE000_ED88;  // enables full access
        fpu         => cortexm4::core::fpu::Unit            @ 0xE000_EF34;
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
        ::libc::memory::IOVec{ptr: __stack_begin as *const u8, size: (__stack_end-__stack_begin) as usize}
    }

    /// Get heap region information.
    fn heap_memory(&self) -> ::libc::memory::IOVec {
        ::libc::memory::IOVec{ptr: __heap_begin as *const u8, size: (__heap_end-__heap_begin) as usize}
    }
}
