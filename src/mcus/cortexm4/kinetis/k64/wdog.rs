extern crate core;
use core::intrinsics::{volatile_load, volatile_store};


ioreg!(
    name => Watchdog;
    doc_srcs => [
        "http://www.nxp.com/files/microcontrollers/doc/ref_manual/K64P144M120SF5RM.pdf"
    ];

    // define "register global" constants
    constants => {
        disabled        = 0;
        enabled         = 1;
    };

    //
    // status and control
    //

    0x0000 => status_control r16 rw {
        // define constants used in the segment of the register
        constants => {
            // clocks
            lpo_clock_src   = 0;
            alt_clock_src   = 1;

            // self tests
            quick_test      = 0;
            byte_test       = 1;
        };

        0 => { // WDOGEN
            enable => [enabled];
            disable => [disabled];
        }

        1 => { // CLKSRC
            use_lpo_clock => [lpo_clock_src];
            use_alt_clock => [alt_clock_src];
        }

        2 => { // IRQRSTEN
            reset_breadcrumb_mode => [enabled];
            reset_default_mode => [disabled];
        }

        3 => { // WINEN
            enable_windowing => [enabled];
            disable_windowing => [disabled];
        }

        4 => { // ALLOWUPDATE
            allow_updates => [enabled];
            disallow_updates => [disabled];
        }

        5 => { // DBGEN
            enable_in_debug => [enabled];
            disable_in_debug => [disabled];
        }

        6 => { // STOPEN
            enable_in_stop => [enabled];
            disable_in_stop => [disabled];
        }

        7 => { // WAITEN
            enable_in_wait => [enabled];
            disable_in_wait => [disabled];
        }

        // bits 8 and 9 are reserved

        10 => { // TESTWDOG
            run_self_test => [enabled];
        }

        11 => { // TESTSEL
            use_quick_test => [quick_test];
            use_byte_test => [byte_test];
        }

        12..13 => { // BYTESEL
            use_byte_zero_test => [0];
            use_byte_one_test => [1];
            use_byte_two_test => [2];
            use_byte_three_test => [3];
        }

        14 => { // DISTESTWDOG
            enable_self_test => [enabled];
            disable_self_test => [disabled];
        }

        // bit 15 is reserved, read only, and should always == 0
    };


    //
    // status and control (low)
    //

    0x0002 => status_control_low r16 rw {
        // 0..14 are reserved and should not be written to

        15 => { // INTFLG
            set_interrupt_flag => [enabled];
        }
    };

    //
    // timeout value
    //

    0x0004 => timeout r32 rw {
        0..31 => {  set_timeout => ();  }
    };

    //
    // window
    //

    0x0008 => window r32 rw {
        0..31 => {  set_window => ();  }
    };

    //
    // refresh
    //

    0x000C => refresh r16 wo {
        constants => {
             value_one = 0xA602;
             value_two = 0xB480;
        };

        0..15 => {  refresh => [value_one, value_two];  }
    };

    //
    // unlock
    //

    0x000E => unlock r16 wo {
        constants => {
             value_one = 0xC520;
             value_two = 0xD928;
        };

        0..15 => {  unlock => [value_one, value_two];  }
    };

    //
    // timer output
    //

    0x0010 => timer_output r32 rw {
        0..31 => {  set_timer_output => ();  }
    };

    //
    // reset count
    //

    0x0014 => reset_count r32 rw {
        0..31 => {  clear_reset_count => [0xFFFF];  }
    };

    //
    // prescaler
    //

    0x0016 => prescaler r32 rw {
        8..10 => {  set_prescaler => ();  }
    };
);
