extern crate core;


//
// unsigned integer division
//

#[repr(C)]
pub struct UDivModResult {pub quot: usize, pub rem: usize}
impl UDivModResult {
    pub fn round_up(&self) -> usize { self.quot + if self.rem > 0 { 1 } else { 0 } }
}

#[no_mangle]
pub fn __aeabi_uidivmod(lhs: usize, rhs: usize) -> UDivModResult {
    // TODO: trigger div-0 isr instead?
    //if rhs == T::from(0) { return T::from(-1); }
    
    let mut cnt: usize = 0;
    let mut iter = lhs;
    while iter > rhs { // TODO: need to check rollover?
        iter = iter - rhs;
        cnt += 1;
    }
    
    UDivModResult{quot: cnt, rem: iter}
}



//
// signed integer division
//


#[repr(C)]
pub struct IDivModResult {pub quot: isize, pub rem: isize}
impl IDivModResult {
    pub fn round_up(&self) -> isize { self.quot + if self.rem > 0 { 1 } else { 0 } }
}

#[no_mangle]
 pub fn __aeabi_idivmod(lhs: isize, rhs: isize) -> IDivModResult {
    // TODO: trigger div-0 isr instead?
    //if rhs == T::from(0) { return T::from(-1); }

    let mut cnt: isize = 0;
    let mut iter = lhs;
    while iter > rhs { // TODO: need to check rollover?
        iter = iter - rhs;
        cnt += 1;
    }

    IDivModResult{quot: cnt, rem: iter}
 }
