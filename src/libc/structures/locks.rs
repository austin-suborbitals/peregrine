extern crate core;
use self::core::intrinsics::atomic_cxchg;


pub struct SpinLock { in_use: u8 }
impl SpinLock {
    pub fn new() -> SpinLock { SpinLock{in_use:0} }

    #[inline(always)]
    pub fn is_locked(&self) -> bool { self.in_use == 1 }

    /// Acquires the lock. If the lock is currently held, wait for it to clean and then take it.
    pub fn acquire(&mut self) {
        loop {
            let state = unsafe { atomic_cxchg(&self.in_use as *const u8 as *mut u8, 0, 1) };
            if state.1 { return; }
        }
    }

    /// Release the lock, but return error if not locked.
    pub fn release(&mut self) -> Result<(), &'static str> {
        if ! self.is_locked() {
            Err("attempt to release unlocked lock")
        } else {
            self.in_use = 0;
            Ok(())
        }
    }

    /// Waits for the lock to be released, but does not acquire it.
    pub fn wait(&self) {
        while self.is_locked() {}
    }
}

unsafe impl Sync for SpinLock {}

#[cfg(test)]
mod spin_lock {
    extern crate std;
    use self::std::{time, thread};
    use self::std::vec::Vec;

    #[test]
    fn defaults_unlocked() {
        let lock = super::SpinLock::new();
        assert_eq!(false, lock.is_locked());
    }

    #[test]
    fn full_cycle() {
        let mut lock = super::SpinLock::new();
        assert_eq!(false, lock.is_locked());
        lock.acquire();
        assert_eq!(true, lock.is_locked());
        lock.release().expect("could not release lock");
        assert_eq!(false, lock.is_locked());
    }

    #[test]
    fn error_on_release_unlocked() {
        let mut lock = super::SpinLock::new();
        assert!(lock.release().is_err());
    }

    static mut WAIT_TEST_LOCK: super::SpinLock = super::SpinLock{in_use: 0};
    #[test]
    fn wait() {
        unsafe { WAIT_TEST_LOCK.acquire(); }

        let releaser = thread::spawn(|| {
            thread::sleep(time::Duration::from_millis(100));
            unsafe { WAIT_TEST_LOCK.release() }.expect("could not release lock");
        });

        unsafe { WAIT_TEST_LOCK.wait(); }
        releaser.join().expect("error in releaser thread");
        assert!(unsafe{WAIT_TEST_LOCK.is_locked()} == false);
    }

    static mut THRASH_TEST_STOP: bool = false;
    static mut THRASH_TEST_COUNT: usize = 0;
    static mut THRASH_TEST_VALUE: usize = 0;
    static mut THRASH_TEST_LOCK: super::SpinLock = super::SpinLock{in_use: 0};
    #[test]
    fn thrash() {
        let mut threads = Vec::<thread::JoinHandle<()>>::new();

        for _ in 0..10 {
            threads.push(thread::spawn(|| {
                unsafe {
                    while THRASH_TEST_STOP == false {
                        THRASH_TEST_LOCK.acquire();
                        assert_eq!(THRASH_TEST_VALUE, THRASH_TEST_COUNT);
                        THRASH_TEST_COUNT += 1;
                        THRASH_TEST_VALUE = THRASH_TEST_COUNT; // make a dependency
                        THRASH_TEST_LOCK.release().expect("could not release lock");
                    }
                }
            }));
        }

        thread::sleep(time::Duration::from_millis(2_000));
        unsafe { THRASH_TEST_STOP = true; } // tell threads to stop

        for t in threads {
            t.join().expect("error in thrasher thread");
        }
        assert!(unsafe{THRASH_TEST_LOCK.is_locked()} == false);
    }
}
