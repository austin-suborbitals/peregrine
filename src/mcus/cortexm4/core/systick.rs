extern crate core;
use core::intrinsics::{volatile_load, volatile_store};

/// Core SysTick definition.
///
/// On init, the interrupt is disabled, ticking is disabled, and the reload value is set to 0.
ioreg!(
    name => SysTick;
    init => pub fn init(&self) {
        self.unset_exception_enable_bit();
        self.unset_enable_bit();
        self.reload(0);
    };

    0x000 => status_and_control r32 rw {
        0 => {
            set_enable_bit => [0x1];
            unset_enable_bit => [0x0];
        }

        1 => {
            set_exception_enable_bit => [0x1];
            unset_exception_enable_bit => [0x0];
        }

        // TODO: clock sourcing
    };

    0x0004 => reload r32 rw {
        0..23 => { reload => (); }
    };

    0x0008 => calibration r32 ro {};
);

impl ::traits::SysTick for SysTick {
    /// Enables ticking of the counter
    fn enable(&self) { self.set_enable_bit(); }

    /// Disables ticking of the counter. Does not reset the value.
    fn disable(&self) { self.unset_enable_bit(); }

    /// Detect if the counter has hit 0 since it was last read. Used if the interrupt is not enabled.
    fn has_reset(&self) -> bool {
        (self.read_status_and_control() & (0x1 << 15)) as usize > 0 // read 16th bit
    }

    /// Enables the SysTick exception when the counter hits 0.
    fn enable_interrupt(&self) { self.set_exception_enable_bit(); }

    /// Disables the SysTick exception.
    fn disable_interrupt(&self) { self.unset_exception_enable_bit(); }

    /// Sets the value the module will reload with when resetting.
    fn set_tick_reload_value(&self, val: usize) { self.reload(val as u32); }

    /// Fetch the current value of the countdown.
    fn current_tick(&self) -> usize { (self.read_reload() & 0xFFFFFF) as usize }

    /// Polls for whether the 10ms calibration value is reliable
    fn has_calibration_value(&self) -> bool { (self.read_calibration() & (0x1 << 29)) > 0 }

    /// Fetches the 10ms calibration value.
    fn calibration_value(&self) -> usize { (self.read_calibration() & 0xFFFFFF) as usize }
}
