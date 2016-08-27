extern crate core;
use self::core::intrinsics::{volatile_load, volatile_store};

/* TODO: coprocessor access register
    0x0000 => access r32 rw {
        20..23 => {
            disable => [0x0];
            privilidged => [0x1];
            full_access => [0x3];
        }
    };
*/

ioreg!(
    name => FPU;

    0x0000 => context r32 rw {
        // 0..29 are statuses to the context i.e. nothing to toggle/set

        30 => {
            enable_lazy_preservation => [0x1];
            disable_lazy_preservation => [0x0];
        }

        31 => {
            enable_automatic_preservation => [0x1];
            disable_automatic_preservation => [0x0];
        }
    };

    0x0004 => address r32 ro {
        // contains the address where the context is stored
    };
);
