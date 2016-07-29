use core::intrinsics;


//------------------------------------------------
//
// memset
//
//------------------------------------------------

pub unsafe fn memset(dest: *mut u8, val: u8, cnt: usize) {
    // TODO: negative byte safety -- compiler seems to nop it
    intrinsics::volatile_set_memory(dest, val, cnt);
}
#[cfg(test)]
mod memset {
    #[test]
    fn sanity() {
        const TEST_LEN: usize = 64;
        let mut buff: [u8; TEST_LEN] = [0; TEST_LEN];
        unsafe {
            super::memset(buff.as_mut_ptr(), 0xAF, TEST_LEN);
        }
        for i in 0..TEST_LEN {
            assert_eq!(0xAF, buff[i]);
        }
    }

    #[test]
    fn zero_length() {
        const TEST_LEN: usize = 64;
        let mut buff: [u8; TEST_LEN] = [0xB4; TEST_LEN];
        for i in 0..TEST_LEN {
            unsafe { super::memset(buff.as_mut_ptr().offset(i as isize), 0xAF, 0); }
            assert_eq!(0xB4, buff[i]);
        }
    }

    #[test]
    fn single_byte() {
        const TEST_LEN: usize = 64;
        let mut buff: [u8; TEST_LEN] = [0xB4; TEST_LEN];
        for i in 0..TEST_LEN {
            unsafe { super::memset(buff.as_mut_ptr().offset(i as isize), 0xAF, 1); }
            assert_eq!(0xAF, buff[i]);
        }
    }

    #[test]
    fn entire_buffer() {
        const TEST_LEN: usize = 64;
        let mut buff: [u8; TEST_LEN] = [0xB4; TEST_LEN];
        unsafe { super::memset(buff.as_mut_ptr(), 0xAF, TEST_LEN); }
        for i in 0..TEST_LEN {
            assert_eq!(0xAF, buff[i]);
        }
    }
}
