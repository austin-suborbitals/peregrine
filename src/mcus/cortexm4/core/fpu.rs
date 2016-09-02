extern crate core;
use core::intrinsics::{volatile_load, volatile_store};

ioreg!(
    name => Access;
    init => pub fn init(&self) {
        self.full_access();
        // TODO: flush the bus
    };

    0x000 => access r32 rw{
        20..23 => {
            disallow => [0x0];
            priviledged => [0x1];
            full_access => [0x3];
        }
    };
);

ioreg!(
    name => Unit;

    0x0000 => context r32 rw {
        31 => {
            enable_automatic_state_saving => [0x1];
            disable_automatic_state_saving => [0x1];
        }

        30 => {
            enable_lazy_state_saving => [0x1];
            disable_lazy_state_saving => [0x1];
        }
    };

    0x0004 => context_address r32 ro {};
);
