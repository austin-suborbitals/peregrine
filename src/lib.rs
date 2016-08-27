#![no_std]
#![plugin(dynamo)]

#![feature(plugin)]
#![feature(const_fn)]
#![feature(core_intrinsics)]
#![feature(associated_consts)]


//------------------------------------------------
//
// modules
//
//------------------------------------------------

pub mod libc;
pub mod mcus;

pub mod traits;


//------------------------------------------------
//
//
//
//------------------------------------------------

pub fn main<T: traits::MCU>(mcu: T) {

}
