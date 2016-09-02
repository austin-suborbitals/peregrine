extern crate core;
use self::core::intrinsics::{ctlz, cttz};


#[inline(always)]
pub fn find_first_set(word: usize) -> u8 {
	(unsafe{cttz(word)} - 1) as u8
}

pub fn find_last_set(word: usize) -> u8 {
	(if word != 0 { 32 - unsafe{ctlz(word)} } else { 0 } - 1) as u8
}

// TODO: rest of tlsf
