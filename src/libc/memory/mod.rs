use core::intrinsics;
use core::cmp::{PartialOrd, PartialEq};


//------------------------------------------------
//
// missing intrinsics
//
//------------------------------------------------

#[no_mangle]
pub unsafe fn __aeabi_memclr(dest: *mut u8, val: u8, cnt: usize) {
    for i in 0..cnt { *(dest.offset(i as isize)) = val; }
}

#[no_mangle]
pub unsafe fn __aeabi_memset(dest: *mut u8, val: u8, cnt: usize) {
    __aeabi_memclr(dest, val, cnt);
}


//------------------------------------------------
//
// memset
//
//------------------------------------------------

/// Set a region of memory to a given byte value.
///
/// Internally, this uses compiler intrinsics (volatile_set_memory).
///
pub unsafe fn memset(dest: *mut u8, val: u8, cnt: usize) {
    // TODO: negative byte safety -- compiler seems to nop it

    // undefined __aeabi_memclr reference :/
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


/// Compares two regions of memory.
///
/// This is a thin wrapper around memcmp_t::\<usize\>()
///
pub unsafe fn memcmp(a: *mut u8, b: *mut u8, byte_cnt: usize) -> i8 {
    memcmp_t::<usize>(a, b, byte_cnt)
}


/// Compare regions of memory with a generic iterator size.
///
/// While this function is generic, we give the count in bytes.
/// This allows the user to choose their step size and thus optimization.
///
/// Returns the lexical ordering of A compared to B.
/// Thus, all possible returns are [-1, 0, 1].
///
pub unsafe fn memcmp_t<T>(a: *mut u8, b: *mut u8, byte_cnt: usize)
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
    fn use_alias() {
        const TEST_LEN: usize = 64;
        let mut buff: [u8; TEST_LEN] = [0xB4; TEST_LEN];
        let mut expect: [u8; TEST_LEN] = [0xAF; TEST_LEN];
        unsafe {
            super::memset(buff.as_mut_ptr(), 0xAF, TEST_LEN);
            assert_eq!(0, super::memcmp(buff.as_mut_ptr(), expect.as_mut_ptr(), TEST_LEN));
        }
    }

    #[test]
    fn as_u8() {
        const TEST_LEN: usize = 64;
        let mut buff: [u8; TEST_LEN] = [0xB4; TEST_LEN];
        let mut expect: [u8; TEST_LEN] = [0xAF; TEST_LEN];
        unsafe {
            super::memset(buff.as_mut_ptr(), 0xAF, TEST_LEN);
            assert_eq!(0, super::memcmp_t::<u8>(buff.as_mut_ptr(), expect.as_mut_ptr(), TEST_LEN));
        }
    }

    #[test]
    fn as_u16() {
        const TEST_LEN: usize = 64;
        let mut buff: [u8; TEST_LEN] = [0xB4; TEST_LEN];
        let mut expect: [u8; TEST_LEN] = [0xAF; TEST_LEN];
        unsafe {
            super::memset(buff.as_mut_ptr(), 0xAF, TEST_LEN);
            assert_eq!(0, super::memcmp_t::<u16>(buff.as_mut_ptr(), expect.as_mut_ptr(), TEST_LEN));
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
            assert_eq!(1, super::memcmp_t::<u16>(buff.as_mut_ptr(), expect.as_mut_ptr(), TEST_LEN));
        }
    }

    #[test]
    fn as_u32() {
        const TEST_LEN: usize = 64;
        let mut buff: [u8; TEST_LEN] = [0xB4; TEST_LEN];
        let mut expect: [u8; TEST_LEN] = [0xAF; TEST_LEN];
        unsafe {
            super::memset(buff.as_mut_ptr(), 0xAF, TEST_LEN);
            assert_eq!(0, super::memcmp_t::<u32>(buff.as_mut_ptr(), expect.as_mut_ptr(), TEST_LEN));
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
            assert_eq!(-1, super::memcmp_t::<u32>(buff.as_mut_ptr(), expect.as_mut_ptr(), TEST_LEN));
        }
    }

    #[test]
    fn as_u64() {
        const TEST_LEN: usize = 64;
        let mut buff: [u8; TEST_LEN] = [0xB4; TEST_LEN];
        let mut expect: [u8; TEST_LEN] = [0xAF; TEST_LEN];
        unsafe {
            super::memset(buff.as_mut_ptr(), 0xAF, TEST_LEN);
            assert_eq!(0, super::memcmp_t::<u64>(buff.as_mut_ptr(), expect.as_mut_ptr(), TEST_LEN));
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
            assert_eq!(-1, super::memcmp_t::<u64>(buff.as_mut_ptr(), expect.as_mut_ptr(), TEST_LEN));
        }
    }

    #[test]
    fn as_u64_unaligned() {
        const TEST_LEN: usize = 123;
        let mut buff: [u8; TEST_LEN] = [0xB4; TEST_LEN];
        let mut expect: [u8; TEST_LEN] = [0xAF; TEST_LEN];
        unsafe {
            super::memset(buff.as_mut_ptr(), 0xAF, TEST_LEN);
            assert_eq!(0, super::memcmp_t::<u64>(buff.as_mut_ptr(), expect.as_mut_ptr(), TEST_LEN));
        }
    }
}


//------------------------------------------------
//
// memcpy
//
//------------------------------------------------

/// Copies one region of memory to another.
///
/// Based on the addresses given and the number of bytes, this function
/// will delegate to one of the following intrinsics:
///
/// * volatile_copy_memory
/// * volatile_copy_nonoverlapping_memory
///
pub unsafe fn memcpy(dst: *mut u8, src: *mut u8, byte_cnt: usize) {
    if (dst < src && dst.offset(byte_cnt as isize) > src) || (src < dst && src.offset(byte_cnt as isize) > dst) {
        intrinsics::volatile_copy_memory(dst, src, byte_cnt);
    } else {
        intrinsics::volatile_copy_nonoverlapping_memory(dst, src, byte_cnt);
    }
}
#[cfg(test)]
mod memcpy {
    #[test]
    fn non_overlapping() {
        const TEST_LEN: usize = 64;
        let mut buff: [u8; TEST_LEN] = [0; TEST_LEN];
        let mut expect: [u8; TEST_LEN] = [0xAF; TEST_LEN];
        unsafe {
            super::memcpy(buff.as_mut_ptr(), expect.as_mut_ptr(), TEST_LEN);
            assert_eq!(0, super::memcmp(buff.as_mut_ptr(), expect.as_mut_ptr(), TEST_LEN));
        }
    }

    #[test]
    fn overlapping() {
        const TEST_LEN: usize = 64;
        const HALF_TEST_LEN: usize = TEST_LEN/2;
        let mut buff: [u8; TEST_LEN] = [0xB4; TEST_LEN];
        unsafe {
            super::memset(buff.as_mut_ptr().offset(HALF_TEST_LEN as isize), 0xAF, HALF_TEST_LEN);

            // we will now copy the first half into the second half, but shifted back `distance` bytes
            let distance = HALF_TEST_LEN / 2;
            let overlap = (HALF_TEST_LEN)-distance;
            super::memcpy(buff.as_mut_ptr(), buff.as_mut_ptr().offset(overlap as isize), HALF_TEST_LEN);

            for i in 0..distance {
                assert_eq!(0xB4, buff[i]);
            }
            for i in distance..TEST_LEN {
                assert_eq!(0xAF, buff[i]);
            }
        }
    }
}



//------------------------------------------------
//
// iovecs
//
//------------------------------------------------

/// Defines an I/O Vector that contains a pointer and size.
#[derive(Copy)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub struct IOVec {
    pub ptr: *const u8,
    pub size: usize,
}
impl IOVec {
    /// Instantiates a new IOVec given the pointer and size.
    pub fn new(ptr: *const u8, size: usize) -> IOVec { IOVec{ptr:ptr, size:size} }

    /// Instantiates a new IOVec given a u32 address and size
    pub fn from_addr(addr: u32, size: usize) -> IOVec { IOVec{ptr:(addr as *const u8), size:size} }

    /// Instantiates a new IOVec given two pointers
    pub fn from_range<T>(begin: *const T, end: *const T) -> IOVec {
        IOVec{ptr:begin as *const u8, size:(end as usize)-(begin as usize)}
    }

    /// Casts the contained pointer to a pointer of the given generic type.
    pub fn convert_ptr<T>(&self) -> *const T { self.ptr as *const T }

    /// Casts the contained pointer to a pointer of the given generic type as well as making it mutable.
    pub fn convert_mut_ptr<T>(&self) -> *mut T { self.ptr as *mut T }

    /// Returns the pointer as a mutable pointer.
    pub fn as_mut(&self) -> *mut u8 { self.ptr as *mut u8 }
}
