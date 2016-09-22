extern crate core;

use ::libc::memory::IOVec;

pub struct RingBuffer<T: Sized> {
    mem: *const T,
    num_bytes: usize,
    scribe: usize,
    has_rolled: bool,
}
impl<T> RingBuffer<T> {
    pub fn new(mem: IOVec) -> RingBuffer<T> {
        RingBuffer{
            mem: mem.convert_mut_ptr::<T>(),
            num_bytes: mem.size,
            scribe: 0,
            has_rolled: false,
        }
    }

    pub fn from(mem: &[T]) -> RingBuffer<T> {
        RingBuffer{
            mem: &mem[0] as *const T,
            num_bytes: mem.len() * core::mem::size_of::<T>(),
            scribe: 0,
            has_rolled: false,
        }
    }

    pub fn size(&self) -> usize {
        self.num_bytes / core::mem::size_of::<T>()
    }

    // NOTE: offset to isize, effectively limits the actual max size of the ring buffer
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

    // NOTE: scribe to isize, effectively limits the actual max size of the ring buffer
    fn scribe_ptr(&self) -> *mut T {
        unsafe { self.mem.offset( self.scribe as isize ) as *mut T }
    }

    pub fn push(&mut self, val: T) {
        unsafe { *(self.scribe_ptr()) = val; }
        self.scribe += 1;
        if self.scribe == self.size() {
            self.scribe = 0;
            self.has_rolled = true; // no sense if loading, checking, storing
        }
    }

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
