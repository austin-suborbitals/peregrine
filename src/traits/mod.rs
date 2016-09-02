//------------------------------------------------
//
// mcu
//
//------------------------------------------------

/// Enables generic handling and initialization of an MCU.
pub trait MCU {
    //
    // ARM core
    //

    /// Returns the NVIC interface for the MCU.
    /// The NVIC returned must satisfy the NVIC trait.
    fn get_nvic(&self) -> &NVIC;


    //
    // memory
    //

    /// Gives an IOVec describing the region of memory dedicated to stack.
    fn stack_memory(&self) -> ::libc::memory::IOVec;

    /// Gives an IOVec describing the region of memory dedicated to the heap.
    fn heap_memory(&self) -> ::libc::memory::IOVec;
}


//------------------------------------------------
//
// nvic
//
//------------------------------------------------

/// Standard interface to the Nested Vector Interrupt Controller.
pub trait NVIC {
    /// Enables the given IRQ.
    fn enable_irq(&self, irq: u8);
    /// Disables the given IRQ.
    fn disable_irq(&self, irq: u8);
    /// Check whether the given IRQ is enabled.
    fn is_enabled(&self, irq: u8) -> bool;

    /// Set the given IRQ as pending, and to be run when priorities dictate.
    fn set_pending(&self, irq: u8);
    /// Clear the given IRQ from the pending list.
    fn clear_pending(&self, irq: u8);
    /// Check whether the given IRQ is pending.
    fn is_pending(&self, irq: u8) -> bool;

    /// Check if the given IRQ is currently being serviced;
    fn is_active(&self, irq: u8) -> bool;

    /// Set the priority of the given IRQ.
    ///
    /// Priorities are limited to the number of priority bits, and this function should handle
    /// all shifting/masking needed.
    fn set_priority(&self, irq: u8, prio: u8);
    /// Gets the priority of the given IRQ.
    ///
    /// Priorities should be returned in the range [0, priority_bits<<2].
    fn get_priority(&self, irq: u8) -> u8;
}


//------------------------------------------------
//
// SysTick
//
//------------------------------------------------

/// Enables generic handling and initialization of a SysTick core module.
pub trait SysTick {
    /// This function should enable the ticking of the SysTick module.
    fn enable(&self);
    /// This function should disable the ticking of the SysTick module. No resetting should be done
    /// unless specified by the mcu's manual.
    fn disable(&self);
    /// This function should indicate if the module has counted to 0 since the last time
    /// the status (or equivalent) register has been read. Used when interrupts are disabled.
    fn has_reset(&self) -> bool;
    /// This function should enable any interrupts availble to the module. Typically an
    /// exception when the module hits 0.
    fn enable_interrupt(&self);
    /// Disables the SysTick exception. No other side effects should be done, unless required to
    /// execute the disabling.
    fn disable_interrupt(&self);

	/// This function should set the value from which the counter will tick.
	fn set_tick_reload_value(&self, val: usize);

	/// This function should return the current value of the countdown. If this cannot be accessed,
	/// (-1) as usize should be returned.
	fn current_tick(&self) -> usize;

	/// This function should indicate if the provided calibration value is reliable, or given at all.
	fn has_calibration_value(&self) -> bool;
	/// This function should return the 10ms calibration value, regardless of whether it is valid
	/// or not. The caller is responsible for checking its validity.
	fn calibration_value(&self) -> usize;

    // TODO: clock sourcing
}
