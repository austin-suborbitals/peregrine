extern crate core;

use ::libc::memory::IOVec;

pub struct RingBuffer<T: Sized> {
    mem: *const T,
    num_bytes: usize,
    scribe: usize,
    has_rolled: bool,
}
impl<T> RingBuffer<T> {
    /// Creates a new ring buffer spanning the region defined by the IOVec.
    ///
    /// __NOTE:__ because pointers are indexed using signed integers, you **should not** create
    /// a buffer larger than 2^31 elements.
    pub fn new(mem: IOVec) -> RingBuffer<T> {
        RingBuffer{
            mem: mem.convert_mut_ptr::<T>(),
            num_bytes: mem.size,
            scribe: 0,
            has_rolled: false,
        }
    }

    /// Creates a ring buffer from the given slice using the slice's `.len()` function and a pointer to the
    /// first index.
    ///
    /// __NOTE:__ because pointers are indexed using signed integers, you **should not** create
    /// a buffer larger than 2^31 elements.
    pub fn from(mem: &[T]) -> RingBuffer<T> {
        RingBuffer{
            mem: &mem[0] as *const T,
            num_bytes: mem.len() * core::mem::size_of::<T>(),
            scribe: 0,
            has_rolled: false,
        }
    }

    /// Returns the (1-indexed) number of items the ring buffer can hold.
    pub fn size(&self) -> usize {
        self.num_bytes / core::mem::size_of::<T>()
    }

    /// Get the value at the "relative index" of the ring buffer.
    ///
    /// By "relative index", the API means the following contract:
    ///
    ///   - `buff.get(0)` will give the oldest value in the buffer
    ///   - `buff.get(buff.size()-1)` will give the newest element in the list.
    ///
    /// __NOTE__: offsetting using `isize` effectively limits the __actual__ max size of the ring buffer
    /// to 2^31.
    ///
    /// Errors are generated on:
    ///
    ///   - Using an index either:
    ///     - Beyond the size of the buffer
    ///     - Beyond the number of inserted items (... only if the buffer has not wrapped yet)
    ///   - The calculated pointer (then converted to ref) to the index is `nullptr`. This can happen when:
    ///     - The user creates a ring buffer starting at `0x0usize` and calls `.get(0)`
    ///     - The buffer is located _high_ in memory such that the offsetting wraps the number space to 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use peregrine::libc::structures::RingBuffer;
    ///
    /// // create a "memory region" to use as the buffer
    /// let arr = [0u32; 16];
    /// let mut buff = RingBuffer::from(&arr);
    ///
    /// // fill the buffer twice
    /// for i in 0..(arr.len()*2) { buff.push(i as u32); }
    ///
    /// // assert our values
    /// assert_eq!(
    ///     arr.len() as u32,
    ///     *buff.get(0).expect("failed to get index 0"),
    ///     "incorrect value for .get(0) case"
    /// );
    /// assert_eq!(
    ///     ((arr.len()*2)-1) as u32,
    ///     *buff.get(buff.size()-1).expect("failed to get last index"),
    ///     "incorrect value for .get(last) case"
    /// );
    ///
    /// // assert they are equal to the `newest()` and `oldest()` helper functions
    /// assert_eq!(
    ///     *buff.newest().expect("failed to get newest"),
    ///     *buff.get(buff.size()-1).expect("failed to get last index"),
    ///     "incorrect value for get-newest case"
    /// );
    /// assert_eq!(
    ///     *buff.oldest().expect("failed to get oldest"),
    ///     *buff.get(0).expect("failed to get index 0"),
    ///     "incorrect value for get-oldest case"
    /// );
    /// ```
    pub fn get(&self, index: usize) -> Result<&T, &'static str> {
        if ! self.has_rolled {
            if index >= self.scribe {
                Err("index is beyond the items pushed so far")
            } else {
                match unsafe { self.mem.offset( index as isize ).as_ref() } {
                    None => { Err("received nullptr from ring buffer lookup") }
                    Some(r) => { Ok(r) }
                }
            }
        } else if index >= self.size() {
            Err("index is out of bounds")
        } else {
            match unsafe { self.mem.offset( ((self.scribe + index) % self.size()) as isize ).as_ref() } {
                None => { Err("received nullptr from ring buffer lookup") }
                Some(r) => { Ok(r) }
            }
        }
    }

    /// Get a pointer derived from the `self.scribe` index.
    ///
    /// __NOTE__: offsetting using `isize` effectively limits the __actual__ max size of the ring buffer
    /// to 2^31.
    fn scribe_ptr(&self) -> *mut T {
        unsafe { self.mem.offset( self.scribe as isize ) as *mut T }
    }

    /// Pushes a value into the ring buffer, overwriting the oldest value in the buffer.
    ///
    /// If the boundry of the buffer is hit, the internal iterator wraps the buffer, restarting
    /// at "index 0".
    ///
    /// # Examples
    ///
    /// ```
    /// use peregrine::libc::structures::RingBuffer;
    ///
    /// // create a "memory region" to use as the buffer
    /// let arr = [0u32; 16];
    /// let mut buff = RingBuffer::from(&arr);
    ///
    /// // fill the buffer exactly
    /// for i in 0..(arr.len() as u32) { buff.push(i); }
    ///
    /// // expect nothing to be overwritten yet
    /// assert_eq!(0, *buff.get(0).unwrap());
    /// assert_eq!((arr.len() as u32) - 1, *buff.get(buff.size()-1).unwrap());
    ///
    /// // now loop, and overwrite
    /// buff.push(0xDEADBEEF);
    /// assert_eq!(0xDEADBEEF, *buff.newest().unwrap());
    /// assert_eq!(0xDEADBEEF, arr[0]);
    /// ```
    ///
    pub fn push(&mut self, val: T) {
        unsafe { *(self.scribe_ptr()) = val; }
        self.scribe += 1;
        if self.scribe == self.size() {
            self.scribe = 0;
            self.has_rolled = true; // no sense if loading, checking, storing
        }
    }

    /// Returns the newest element in the buffer.
    ///
    /// __NOTE:__ calling `.newest()` on an empty buffer will result in an error.
    ///
    /// # Examples
    ///
    /// ```
    /// use peregrine::libc::structures::RingBuffer;
    ///
    /// // create a "memory region" to use as the buffer
    /// let arr = [0u32; 16];
    /// let mut buff = RingBuffer::from(&arr);
    ///
    /// // fill and assert
    /// for i in 0..(arr.len() as u32) {
    ///     if i > 0 { assert_eq!(i-1, *buff.newest().unwrap()); }
    ///     buff.push(i);
    ///     assert_eq!(i, *buff.newest().unwrap());
    /// }
    /// ```
    pub fn newest(&self) -> Result<&T, &'static str> {
        //self.get(if self.scribe == 0 { self.size()-1 } else { self.scribe-1 })
        if self.scribe == 0 {
            self.get( self.size()-1 )
        } else {
            let as_ref = unsafe { self.scribe_ptr().offset(-1).as_ref() };
            if as_ref.is_none() {
                Err("received nullptr from ring buffer lookup")
            } else {
                Ok(as_ref.unwrap())
            }
        }
    }


    /// Returns the oldest element in the buffer.
    ///
    /// __NOTE:__ calling `.oldest()` on an empty buffer will result in an error.
    ///
    /// # Examples
    ///
    /// ```
    /// use peregrine::libc::structures::RingBuffer;
    ///
    /// // create a "memory region" to use as the buffer
    /// let arr = [0u32; 16];
    /// let mut buff = RingBuffer::from(&arr);
    ///
    /// // fill and assert
    /// for i in 0..(arr.len() as u32) {
    ///     buff.push(i);
    ///     assert_eq!(0, *buff.oldest().unwrap());
    /// }
    ///
    /// // roll the buffer and do a new assert
    /// buff.push(0xDEADBEEF);
    /// assert_eq!(1, *buff.oldest().unwrap());
    /// ```
    pub fn oldest(&self) -> Result<&T, &'static str> {
        self.get(0)
    }
}

#[cfg(test)]
mod tests {
    pub use super::RingBuffer;

    //
    // testing all insertion operations
    //

    mod insert {
        #[test]
        fn push_one_get_at_zero() {
            let arr = [0usize; 256];
            let mut buff = super::RingBuffer::from(&arr);

            buff.push(0x1234);
            assert_eq!(0x1234, *buff.get(0).expect("could not get index"));
        }

        #[test]
        fn overfill() {
            let arr = [0usize; 256];
            let mut buff = super::RingBuffer::from(&arr);

            for i in 0..10_000 { buff.push(i); }
        }

        #[test]
        fn stepped_insert_and_get() {
            let arr = [0xFFusize; 256];
            for i in 0..10_000 { // index 0 is waited but :shrug:
                let mut buff = super::RingBuffer::from(&arr);
                for p in 0..i {
                    buff.push(p);
                    assert_eq!(arr[p%arr.len()], p, "incorrect value in array");
                    assert_eq!(p, *buff.newest().expect("could not get index"), "incorrect value at step {} and index {}", i, p);
                }
            }
        }
    }


    //
    // testing all fetching operations
    //

    mod lookup {
        pub use super::RingBuffer;

        #[test]
        fn fill_and_get() {
            let arr = [0usize; 256];
            let mut buff = super::RingBuffer::from(&arr);

            for i in 0..arr.len() {
                buff.push(i);
            }

            for i in 0..arr.len() {
                assert_eq!(i, *buff.get(i).expect("failed to get from ring buffer"));
            }
        }

        #[test]
        fn overfill_and_get() {
            let arr = [0usize; 256];
            let mut buff = super::RingBuffer::from(&arr);

            for i in 0..(arr.len() * 2) {
                buff.push(i);
            }

            for i in 0..arr.len() {
                assert_eq!(i+arr.len(), *buff.get(i).expect("failed to get from ring buffer"), "invalid value at index {}", i);
            }
        }

        #[test]
        fn get_at_zero_is_oldest() {
            let arr = [0usize; 256];
            let mut buff = super::RingBuffer::from(&arr);

            for i in 0..100_000 { buff.push(i); }

            assert_eq!(*buff.oldest().unwrap(), *buff.get(0).unwrap());
        }

        #[test]
        fn get_at_size_is_newest() {
            let arr = [0usize; 256];
            let mut buff = super::RingBuffer::from(&arr);

            for i in 0..100_000 { buff.push(i); }

            assert_eq!(*buff.newest().unwrap(), *buff.get(buff.size()-1).unwrap());
        }

        //
        // testing the newest operator
        //

        mod newest {
            pub use super::RingBuffer;

            #[test]
            fn newest() {
                let arr = [0usize; 256];
                let mut buff = super::RingBuffer::from(&arr);

                for i in 0..arr.len() {
                    buff.push(i);
                }

                assert_eq!(arr.len()-1, *buff.newest().expect("error from `newest`"));
            }

            #[test]
            fn newest_on_empty_is_err() {
                let arr = [0usize; 256];
                let buff = super::RingBuffer::from(&arr);

                assert!(buff.newest().is_err());
            }

            #[test]
            fn newest_after_one_push() {
                let arr = [0usize; 256];
                let mut buff = super::RingBuffer::from(&arr);
                buff.push(0x1234);

                assert_eq!(0x1234, *buff.newest().expect("error from `newest`"));
                assert_eq!(*buff.oldest().expect("error from `oldest`"), *buff.newest().expect("error from `newest`"));
            }

            #[test]
            fn newest_on_unrolled_is_last() {
                let arr = [0usize; 256];
                let mut buff = super::RingBuffer::from(&arr);

                for i in 0..(arr.len() / 2) { buff.push(i); }

                assert_eq!((arr.len()/2)-1, *buff.newest().expect("got error retrieving newest"));
            }

            #[test]
            fn newest_on_full_is_last() {
                let arr = [0usize; 256];
                let mut buff = super::RingBuffer::from(&arr);

                for i in 0..arr.len() { buff.push(i); }

                assert_eq!(arr.len()-1, *buff.newest().expect("got error retrieving newest"));
            }

            #[test]
            fn newest_on_rolled_is_last() {
                let arr = [0usize; 256];
                let mut buff = super::RingBuffer::from(&arr);

                for i in 0..(arr.len()*2) { buff.push(i); }

                assert_eq!((arr.len()*2)-1, *buff.newest().expect("got error retrieving newest"));
            }
        }


        //
        // testing the oldest operator
        //

        mod oldest {
            pub use super::RingBuffer;

            #[test]
            fn oldest() {
                let arr = [0usize; 256];
                let mut buff = super::RingBuffer::from(&arr);

                for i in 0..arr.len() {
                    buff.push(i);
                }

                assert_eq!(0, *buff.oldest().expect("error from `oldest`"));
            }

            #[test]
            fn oldest_on_empty_is_err() {
                let arr = [0usize; 256];
                let buff = super::RingBuffer::from(&arr);

                assert!(buff.oldest().is_err());
            }

            #[test]
            fn oldest_after_one_push() {
                let arr = [0usize; 256];
                let mut buff = super::RingBuffer::from(&arr);
                buff.push(0x1234);

                assert_eq!(0x1234, *buff.oldest().expect("error from `oldest`"));
                assert_eq!(*buff.newest().expect("error from `newest`"), *buff.oldest().expect("error from `oldest`"));
            }

            #[test]
            fn oldest_on_unrolled_is_first() {
                let arr = [0usize; 256];
                let mut buff = super::RingBuffer::from(&arr);

                for i in 0..(arr.len() / 2) { buff.push(i); }

                assert_eq!(0, *buff.oldest().expect("got error retrieving oldest"));
            }

            #[test]
            fn oldest_on_full_is_first() {
                let arr = [0usize; 256];
                let mut buff = super::RingBuffer::from(&arr);

                for i in 0..arr.len() { buff.push(i); }

                assert_eq!(0, *buff.oldest().expect("got error retrieving oldest"));
            }

            #[test]
            fn oldest_on_rolled_is_first() {
                let arr = [0usize; 256];
                let mut buff = super::RingBuffer::from(&arr);

                for i in 0..(arr.len()*2) { buff.push(i); }

                assert_eq!(arr.len(), *buff.oldest().expect("got error retrieving oldest"));
            }
        }
    }
}
