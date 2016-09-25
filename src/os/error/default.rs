#![cfg(not(test))]
#![cfg(feature = "panic_default")]

//------------------------------------------------
//
// stack unwinding
//
//------------------------------------------------

pub mod unwind {
    extern crate core;
    use core::fmt::Arguments;

    #[no_mangle]
    pub extern "C" fn __aeabi_unwind_cpp_pr0() -> () {
        loop {}
    }

    #[no_mangle]
    pub extern "C" fn __aeabi_unwind_cpp_pr1() -> () {
        loop {}
    }

    #[no_mangle]
    #[lang="panic_fmt"]
    pub extern "C" fn rust_begin_unwind(_fmt: &Arguments,
                                        _file_line: &(&'static str, usize))
                                        -> ! { loop {} }

    #[allow(non_snake_case)]
    #[no_mangle]
    pub extern "C" fn _Unwind_Resume() -> ! {
        loop {}
    }
}


//------------------------------------------------
//
// error / exception handling
//
//------------------------------------------------

pub mod personality {
    #[lang="eh_personality"]
    extern "C" fn eh_personality() {}
}
