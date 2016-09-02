//------------------------------------------------
//
// stack unwinding
//
//------------------------------------------------

#[cfg(not(test))]
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

    #[lang="panic_fmt"]
    #[no_mangle]
    pub extern "C" fn rust_begin_unwind(_fmt: &Arguments,
                                        _file_line: &(&'static str, usize))
                                        -> ! { loop {} }
}


//------------------------------------------------
//
// error / exception handling
//
//------------------------------------------------

#[cfg(not(test))]
pub mod personality {
    #[lang="eh_personality"]
    extern "C" fn eh_personality() {}
}
