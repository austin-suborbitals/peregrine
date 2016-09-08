use ::libc::memory::IOVec;


pub struct Bitmap {
    mem: IOVec,
    total: usize,
    used: usize,
}

impl Bitmap {
    //
    // creation
    //

    /// Create a new IOVec from the info, and initialize a bitmap
    pub fn new(addr: *const u8, size: usize) -> Bitmap {
        Bitmap::from_iov(IOVec{ptr:addr, size:size})
    }

    /// Consume the IOVec and return the initialized bitmap
    pub fn from_iov(iov: IOVec) -> Bitmap {
        let num_bits = iov.size * 8;
        let mut result = Bitmap{
            mem: iov,
            total: num_bits,
            used: 0,
        };
        result.clear_all();

        result
    }

    //
    // initialization/cleanup
    //

    /// Convenience function for clearing the entire bitmap. Internal use only, hence no-pub.
    fn clear_all(&mut self) {
        self.used = 0;
        unsafe { ::libc::memory::memset(self.mem.as_mut(), 0, self.mem.size); }
    }


    //
    // state fetching
    //

    /// Get the total number of bits in the bitmap.
    pub fn count(&self) -> usize { self.total }
    /// Get the number of set bits in the bitmap.
    pub fn used(&self) -> usize { self.used }
    /// Get the number of unset bits in the bitmap.
    pub fn free(&self) -> usize { self.total - self.used }

    /// Get the state of the Nth bit.
    pub fn is_set(&self, index: usize) -> bool {
        let val = unsafe { *(self.mem.ptr.offset((index/8) as isize)) };
        (val & (1 << (index%8))) > 0
    }


    //
    // setting
    //

    /// Sets the bit at the given index.
    ///
    /// An error is returned if the index is out of the bitmap's bounds.
    pub fn set(&mut self, index: usize) -> Result<(), &'static str> {
        if index > (self.total-1) {
            return Err("requested index to set is not in the bounds of the bitmap");
        }

        unsafe { *(self.mem.as_mut().offset((index/8) as isize)) |= 1 << (index % 8); }
        self.used += 1;
        Ok(())
    }

    /// Sets the bit at the given index.
    ///
    /// An error is returned if the index is out of the bitmap's bounds or if the bit was already set.
    pub fn checked_set(&mut self, index: usize) -> Result<(), &'static str> {
        if index > self.total {
            return Err("requested index to set is not in the bounds of the bitmap");
        }

        let mask = 1 << (index % 8);
        let ptr = unsafe { self.mem.as_mut().offset((index/8) as isize) };

        if unsafe { *ptr } & mask != 0 {
            return Err("bit was already set!");
        }

        unsafe { *ptr |= mask; }
        self.used += 1;
        Ok(())
    }

    /// Sets the bits starting at the given index through the given range.
    ///
    /// An error is returned if the index is out of the bitmap's bounds.
    pub fn set_range(&mut self, index: usize, count: usize) -> Result<(), &'static str> {
        // TODO: this can be optimized wayyyyyyy more. but naive is ok for POC impl
        for i in index..(index+count) {
            try!(self.set(i));
        }
        Ok(())
    }


    //
    // unsetting
    //

    /// Clears the bit at the given index.
    ///
    /// An error is returned if the index is out of the bitmap's bounds.
    pub fn clear(&mut self, index: usize) -> Result<(), &'static str> {
        if index > (self.total-1) {
            return Err("requested index to clear is not in the bounds of the bitmap");
        }

        unsafe { *(self.mem.as_mut().offset((index/8) as isize)) &= !(1 << (index % 8)); }
        self.used -= 1;
        Ok(())
    }

    /// Clears the bit at the given index.
    ///
    /// An error is returned if the index is out of the bitmap's bounds or if the bit was already unset.
    pub fn checked_clear(&mut self, index: usize) -> Result<(), &'static str> {
        if index > self.total {
            return Err("requested index to clear is not in the bounds of the bitmap");
        }

        let mask = 1 << (index % 8);
        let ptr = unsafe { self.mem.as_mut().offset((index/8) as isize) };

        if unsafe { *ptr } & mask == 0 {
            return Err("bit was already clear!");
        }

        unsafe { *ptr &= !mask; }
        self.used -= 1;
        Ok(())
    }


    //
    // searching
    //

    /// Search the bitmap for `count` contiguous unset bits. If found, return the block number that starts the region.
    pub fn find(&self, count: usize) -> Option<usize> {
        let mut contig = 0;
        let mut first_contig = 0;

        // TODO: should we optimize the count%8==0 case? it could lead to unnecessary fragmentation

        // iterate every byte in the bitmap
        for index in 0..self.mem.size {
            for b in 0..8 {
                if self.is_set((index*8)+b) {           // TODO: can be done inline with bit ops, but POC
                    contig = 0; // reset the count
                } else {
                    if contig == 0 { first_contig = (index*8)+b; }
                    contig += 1;
                    if contig == count {
                        return Some(first_contig);
                    }
                }
            }
        }

        None
    }


    /// Search the bitmap for `count` contiguous unset bits. If found, set them.
    pub fn find_and_set(&mut self, count: usize) -> Result<usize, &'static str> {
        let found = self.find(count);
        if found.is_none() { return Err("could not find enough contiguous bits"); }

        let bit = found.unwrap();
        let result = self.set_range(bit, count);
        if result.is_err() {
            Err(result.err().unwrap())
        } else {
            Ok(bit)
        }
    }
}



#[cfg(test)]
mod tests {
    use ::libc::memory::IOVec;

    #[test]
    fn count_sanity() {
        let buff = [0u8; 4096];
        for i in 1..buff.len()+1 { // the "size" is not 0-indexed
            let iov = IOVec {ptr: &buff[0], size: i};
            let bmap = super::Bitmap::from_iov(iov);

            assert_eq!(i*8, bmap.count(), "wrong bit count for size: {}", i);
            assert_eq!(0, bmap.used(), "wrong used count for size: {}", i);
            assert_eq!(bmap.count(), bmap.free(), "wrong free count for size: {}", i);
        }
    }

    #[test]
    fn bounds_check() {
        let buff = [0u8; 1];
        let mut bmap = super::Bitmap::new(&buff[0] as *const u8, buff.len());
        assert!(bmap.set(8).is_err(), "did not receive error for out of bounds set");
        assert!(bmap.set(8).is_err(), "did not receive error for out of bounds clear");
    }

    #[test]
    fn set_and_clear_all() {
        let buff = [0u8; 16];
        let mut bmap = super::Bitmap::new(&buff[0] as *const u8, buff.len());
        for b in 0..buff.len() {
            for i in 0..8 {
                let index = (b*8)+i;
                bmap.set(index).expect("could not set index");
                assert!(bmap.is_set(index), "is_set claims the bit is not set");
            }
        }

        // all bits should be set
        for b in 0..buff.len() { assert_eq!(0xFF, buff[b]); }

        // clear all bits, and assert the bitmap is back to nulls
        for index in 0..bmap.count() { bmap.clear(index).expect("could not unset bit"); }
        for b in 0..buff.len() { assert_eq!(0x0, buff[b]); }
    }

    #[test]
    fn checked_set_and_clear() {
        let buff = [0u8; 16];
        let mut bmap = super::Bitmap::new(&buff[0] as *const u8, buff.len());

        let bit = 18;

        // make sure it works at all
        assert!(bmap.checked_set(bit).is_ok());
        assert!(bmap.checked_clear(bit).is_ok());

        // now expect failure
        assert!(bmap.checked_set(bit).is_ok());
        assert!(bmap.checked_set(bit).is_err());
        assert!(bmap.checked_clear(bit).is_ok());
        assert!(bmap.checked_clear(bit).is_err());
    }

    #[test]
    fn set_first_and_last() {
        let buff = [0u8; 16];
        let mut bmap = super::Bitmap::new(&buff[0] as *const u8, buff.len());

        for i in 0..buff.len() {
            bmap.set(i*8).expect("could not set first bit");
            bmap.set((i*8)+7).expect("could not set last bit");
        }
        for i in 0..buff.len() { assert_eq!(0x81, buff[i]); }
    }

    #[test]
    fn find_all_bits() {
        let buff = [0u8; 16];
        let mut bmap = super::Bitmap::new(&buff[0] as *const u8, buff.len());

        for i in 0..bmap.count() {
            let found = bmap.find(1);
            assert_eq!(Some(i), found, "either no blocks found, or incorrect block returned");
            assert_eq!(i, bmap.used());
            bmap.set(i).expect("could not set bit in bitmap"); // set it so we don't find it again
        }

        for i in 0..buff.len() { assert_eq!(0xFF, buff[i]); }
    }

    #[test]
    fn find_all_bytes() {
        let buff = [0u8; 16];
        let mut bmap = super::Bitmap::new(&buff[0] as *const u8, buff.len());

        for i in 0..buff.len() {
            let found = bmap.find(8);
            assert_eq!(Some(i*8), found, "either no blocks found, or incorrect block returned");
            assert_eq!(i*8, bmap.used());
            for b in 0..8 {
                bmap.set((i*8)+b).expect("could not set bit in bitmap"); // set it so we don't find it again
            }
        }

        for i in 0..buff.len() { assert_eq!(0xFF, buff[i]); }
    }

    #[test]
    fn find_multiple_bytes() {
        let buff = [0u8; 4];
        let mut bmap = super::Bitmap::new(&buff[0] as *const u8, buff.len());

        for i in 0..2 {
            let found = bmap.find_and_set(16); // find half the buffer
            assert_eq!(Ok(i*16), found, "either no blocks found, or incorrect block returned");
            assert_eq!((i+1)*16, bmap.used(), "incorrect number of used bits");
        }

        for i in 0..buff.len() { assert_eq!(0xFF, buff[i]); }
    }

    #[test]
    fn find_split_bytes() {
        let buff = [0u8; 4];
        let mut bmap = super::Bitmap::new(&buff[0] as *const u8, buff.len());

        // set the first 4 bits of the first byte
        for i in 0..4 { bmap.set(i).expect("could not set bit in setup"); }

        // get a whole byte
        let byte_block = bmap.find_and_set(8).expect("could not find 8 contiguous bits");
        assert_eq!(4, byte_block, "expected this range to start at 4");

        // get the "rest of that byte" + 1 bit
        bmap.find_and_set(5).expect("could not get remnants of byte + 1");

        assert_eq!(0xFF, buff[0], "did not reserve entire first byte");
        assert_eq!(0xFF, buff[1], "did not reserve entire second byte");
        assert_eq!(0x01, buff[2], "did not reserve one bit overhang");
    }

    #[test]
    fn find_whole_bitmap() {
        let buff = [0u8; 4];
        let mut bmap = super::Bitmap::new(&buff[0] as *const u8, buff.len());
        bmap.find_and_set(buff.len()*8).expect("could no find and set whole bitmap");
        assert_eq!(bmap.count(), bmap.used());
        assert_eq!(0, bmap.free());
    }

    #[test]
    fn find_in_gap() {
        // NOTE: used(), free(), etc cannot be used as we manually form the bitmap
        let buff = [0xCFu8; 4]; // 0xCF = 0b11001111
        let mut bmap = super::Bitmap::new(&buff[0] as *const u8, buff.len());

        for _ in 0..buff.len() {
            bmap.find_and_set(2).expect("could not find or set in gap");
        }
    }
}
