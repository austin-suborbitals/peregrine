use core::intrinsics;
use core::cmp::{PartialOrd, PartialEq};


//------------------------------------------------
//
// memset
//
//------------------------------------------------

/// Set a region of memory to a given byte value.
///
/// Internally, this uses compiler intrinsics (volatile_set_memory).
pub unsafe fn memset(dest: *mut u8, val: u8, cnt: usize) {
    // TODO: negative byte safety -- compiler seems to nop it
    intrinsics::volatile_set_memory(dest, val, cnt);
}
#[cfg(test)]
mod memset {
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


//------------------------------------------------
//
// memcmp
//
//------------------------------------------------

macro_rules! lex_cmp {
    ($a:expr, $b:expr) => {
        if $a < $b { -1 }
        else if $a > $b { 1 }
        else { 0 }
    };

    ($a:ident, $b:ident, $off:expr) => {
        lex_cmp!(*$a.offset($off), *$b.offset($off))
    };
}

/// Compare regions of memory.
///
/// While this function is generic, we give the count in bytes.
/// This allows the user to choose their step size and thus optimization.
///
/// Returns the lexical ordering of A compared to B.
/// Thus, all possible returns are [-1, 0, 1].
pub unsafe fn memcmp<T>(a: *mut u8, b: *mut u8, byte_cnt: usize)
    -> i8
    where T: PartialEq + PartialOrd
{
    let t_sz: isize = intrinsics::size_of::<T>() as isize; // TODO: isize cast
    let t_off: isize = (byte_cnt as isize) / t_sz;
    let a_iter: *mut T = a as *mut T;
    let b_iter: *mut T = b as *mut T;

    for i in 0isize..t_off {
        // TODO: usize -> isize in offest
        if lex_cmp!(a_iter, b_iter, i) != 0 {
            let curr_off: isize = i*t_sz;
            for k in curr_off..(curr_off + t_sz) {
                let cmp = lex_cmp!(a, b, k);
                if cmp != 0 { return cmp; }
            }
        }
    }

    // TODO: byte_cnt to isize
    let mut new_begin = t_off * t_sz;
    if new_begin > 0 { new_begin += t_sz; }
    for i in new_begin..(byte_cnt as isize) {
        let cmp = lex_cmp!(a, b, i);
        if cmp != 0 { return cmp; }
    }
    0
}
#[cfg(test)]
mod memcmp {
    #[test]
    fn as_u8() {
        const TEST_LEN: usize = 64;
        let mut buff: [u8; TEST_LEN] = [0xB4; TEST_LEN];
        let mut expect: [u8; TEST_LEN] = [0xAF; TEST_LEN];
        unsafe {
            super::memset(buff.as_mut_ptr(), 0xAF, TEST_LEN);
            assert_eq!(0, super::memcmp::<u8>(buff.as_mut_ptr(), expect.as_mut_ptr(), TEST_LEN));
        }
    }

    #[test]
    fn as_u16() {
        const TEST_LEN: usize = 64;
        let mut buff: [u8; TEST_LEN] = [0xB4; TEST_LEN];
        let mut expect: [u8; TEST_LEN] = [0xAF; TEST_LEN];
        unsafe {
            super::memset(buff.as_mut_ptr(), 0xAF, TEST_LEN);
            assert_eq!(0, super::memcmp::<u16>(buff.as_mut_ptr(), expect.as_mut_ptr(), TEST_LEN));
        }
    }

    #[test]
    fn as_u16_last_byte() {
        const TEST_LEN: usize = 64;
        let mut buff: [u8; TEST_LEN] = [0xB4; TEST_LEN];
        let mut expect: [u8; TEST_LEN] = [0xAF; TEST_LEN];
        unsafe {
            super::memset(buff.as_mut_ptr(), 0xAF, TEST_LEN);
            buff[TEST_LEN-1] = 0xC9;
            // expect the buffer to now return LARGER than the expected
            assert_eq!(1, super::memcmp::<u16>(buff.as_mut_ptr(), expect.as_mut_ptr(), TEST_LEN));
        }
    }

    #[test]
    fn as_u32() {
        const TEST_LEN: usize = 64;
        let mut buff: [u8; TEST_LEN] = [0xB4; TEST_LEN];
        let mut expect: [u8; TEST_LEN] = [0xAF; TEST_LEN];
        unsafe {
            super::memset(buff.as_mut_ptr(), 0xAF, TEST_LEN);
            assert_eq!(0, super::memcmp::<u32>(buff.as_mut_ptr(), expect.as_mut_ptr(), TEST_LEN));
        }
    }

    #[test]
    fn as_u32_last_byte() {
        const TEST_LEN: usize = 64;
        let mut buff: [u8; TEST_LEN] = [0xB4; TEST_LEN];
        let mut expect: [u8; TEST_LEN] = [0xAF; TEST_LEN];
        unsafe {
            super::memset(buff.as_mut_ptr(), 0xAF, TEST_LEN);
            buff[TEST_LEN-1] = 0x09;
            // expect the buffer to now return LARGER than the expected
            assert_eq!(-1, super::memcmp::<u32>(buff.as_mut_ptr(), expect.as_mut_ptr(), TEST_LEN));
        }
    }

    #[test]
    fn as_u64() {
        const TEST_LEN: usize = 64;
        let mut buff: [u8; TEST_LEN] = [0xB4; TEST_LEN];
        let mut expect: [u8; TEST_LEN] = [0xAF; TEST_LEN];
        unsafe {
            super::memset(buff.as_mut_ptr(), 0xAF, TEST_LEN);
            assert_eq!(0, super::memcmp::<u64>(buff.as_mut_ptr(), expect.as_mut_ptr(), TEST_LEN));
        }
    }

    #[test]
    fn as_u64_last_byte() {
        const TEST_LEN: usize = 64;
        let mut buff: [u8; TEST_LEN] = [0xB4; TEST_LEN];
        let mut expect: [u8; TEST_LEN] = [0xAF; TEST_LEN];
        unsafe {
            super::memset(buff.as_mut_ptr(), 0xAF, TEST_LEN);
            buff[TEST_LEN-1] = 0x09;
            // expect the buffer to now return LARGER than the expected
            assert_eq!(-1, super::memcmp::<u64>(buff.as_mut_ptr(), expect.as_mut_ptr(), TEST_LEN));
        }
    }

    #[test]
    fn as_u64_unaligned() {
        const TEST_LEN: usize = 123;
        let mut buff: [u8; TEST_LEN] = [0xB4; TEST_LEN];
        let mut expect: [u8; TEST_LEN] = [0xAF; TEST_LEN];
        unsafe {
            super::memset(buff.as_mut_ptr(), 0xAF, TEST_LEN);
            assert_eq!(0, super::memcmp::<u64>(buff.as_mut_ptr(), expect.as_mut_ptr(), TEST_LEN));
        }
    }
}
