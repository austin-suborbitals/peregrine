use ::libc::math::uceil;
use ::libc::memory::IOVec;

pub struct SlabAllocator {
    blocks: IOVec,
    bitmap: ::libc::structures::Bitmap,
    block_size: usize,
    num_blocks: usize,
}
impl SlabAllocator {
    /// Instantiates a new slab allocator managing the region defined by the `IOVec`.
    ///
    /// A bitmap is used to track used blocks, thus `buffer_length / block_size` will not always give you the proper
    /// block count. If the buffer has enough room for the bitmap, but not another block, this dead space will be
    /// used for the bitmap.
    pub fn new(iov: IOVec, block_size: usize) -> SlabAllocator {
        let num_blocks = iov.size / block_size; // we do not ceil this... extra space is detected later
        let num_bitmap_bytes = uceil(num_blocks, 8);
        let mut num_bitmap_blocks = uceil(num_bitmap_bytes, block_size);

        let mut dead_bitmap = false;
        if num_bitmap_blocks == 1 && num_bitmap_bytes < (iov.size % block_size) {
            dead_bitmap = true;
            num_bitmap_blocks = 0;
        }

        let (blks, bmap) = match dead_bitmap {
            true => {
                (
                    IOVec{
                        ptr: iov.ptr,
                        size: iov.size - (iov.size % block_size)    // trim the "fat" we use for the bmap
                    },
                    IOVec{
                        ptr: unsafe { iov.ptr.offset( (iov.size - (iov.size % block_size)) as isize) },
                        size: iov.size % block_size
                    }
                )
            }
            false => {
                (
                    IOVec{
                        ptr: unsafe { iov.ptr.offset((num_bitmap_blocks*block_size) as isize) },
                        size: (num_blocks - num_bitmap_blocks) * block_size,
                    },
                    IOVec{
                        ptr: iov.ptr,
                        size: num_bitmap_bytes  // because we pass to the Bitmap::from_iov(), do not let it have the whole region
                    }
                )
            }
        };


        unsafe { ::libc::memory::memset(iov.ptr as *mut u8, 0, iov.size); }
        SlabAllocator{
            blocks: blks,
            bitmap: ::libc::structures::Bitmap::from_iov(bmap),
            block_size: block_size,
            num_blocks: num_blocks - num_bitmap_blocks,
        }
    }

    /// Allocate the given number of blocks. The returned IOVec should be "given back" to the allocator when freed.
    ///
    /// Errors occur from bounds checking, internal failures, or unavailability of memory.
    pub fn alloc(&mut self, block_count: usize) -> Result<IOVec, &'static str> {
        if block_count > self.bitmap.free() { return Err("not enough free blocks (without checking continuity)"); }
        if block_count > self.num_blocks { return Err("requested block count is more than exist in this heap"); }

        let blocks = self.bitmap.bounded_find_and_set(block_count, self.num_blocks);
        if blocks.is_err() { return Err(blocks.err().unwrap()); }

        Ok(IOVec{
            ptr: unsafe { self.blocks.ptr.offset( (blocks.ok().unwrap() * self.block_size) as isize) },
            size: block_count*self.block_size,
        })
    }

    /// Free the given region of allocated memory. The input ideally should come from a previous `::alloc()`.
    ///
    /// Errors occur from bounds checking, freeing unallocated/unmanaged memory, or unaligned `IOVec` pointer and/or size.
    ///
    /// __NOTE:__ if `Bitmap::checked_clear(...)` returns an error, the error is immediately returned. This is being worked on
    /// but can lead to things not being freed. The solution will likely be returning the index with the error -- but we cannot
    /// return a `Vec` of errors for instance.
    pub fn free(&mut self, iov: IOVec) -> Result<(), &'static str> {
        let offset = (iov.ptr as usize) - (self.blocks.ptr as usize);
        if offset % self.block_size != 0 {
            return Err("iov ptr is not aligned to a block");
        }
        let first_block = offset / self.block_size;

        if iov.size % self.block_size != 0 {
            return Err("iov size is not aligned to a block");
        }
        let num_blocks = iov.size / self.block_size;
        if num_blocks > self.bitmap.used() {
            return Err("attempting to free more blocks than are used");
        }
        if num_blocks > self.num_blocks {
            return Err("attempting to free more blocks than are in this heap"); // TODO: kind of a duplicate
        }

        for i in 0..num_blocks {
            let check = self.bitmap.checked_clear(first_block+i); // we "naturally" limit the "unaligned" bitmap here
            if check.is_err() {
                return Err(check.err().unwrap()); // TODO: what to do with the things we skip on early exit?
            }
        }

        Ok(())
    }

    /// Get the total number of managed blocks.
    pub fn blocks(&self) -> usize { self.num_blocks }
    /// Get the number of blocks available to the manager.
    pub fn free_blocks(&self) -> usize { self.bitmap.free() - (self.bitmap.count() - self.num_blocks) }
    /// Get the number of free blocks in the manager.
    pub fn used_blocks(&self) -> usize { self.bitmap.used() }
}


#[cfg(test)]
mod test {
    mod sanity {
        use ::libc::memory::IOVec;

        #[test]
        pub fn unaligned_free_errors() {
            let buff = [0u8; 4200];
            let mut mman = super::super::SlabAllocator::new(IOVec{ptr:&buff[0], size:buff.len()}, 1024);

            // unaligned ptr
            assert!(mman.free(IOVec{ptr:&buff[3], size:1024}).is_err());

            // unaligned size
            assert!(mman.free(IOVec{ptr:&buff[0], size:7}).is_err());

            // unaligned all
            assert!(mman.free(IOVec{ptr:&buff[9], size:16}).is_err());
        }

        #[test]
        pub fn free_unused_error() {
            let buff = [0u8; 4200];
            let mut mman = super::super::SlabAllocator::new(IOVec{ptr:&buff[0], size:buff.len()}, 1024);
            assert!(mman.free(IOVec{ptr:&buff[0], size:1024}).is_err());
        }

        #[test]
        pub fn alloc_too_many_error() {
            let buff = [0u8; 4200];
            let mut mman = super::super::SlabAllocator::new(IOVec{ptr:&buff[0], size:buff.len()}, 1024);
            assert!(mman.alloc(5).is_err());
        }

        #[test]
        pub fn alloc_bounded() {
            let buff = [0u8; 5125]; // enough for bitmap but not 8-byte aligned
            let mut mman = super::super::SlabAllocator::new(IOVec{ptr:&buff[0], size:buff.len()}, 1024);

            for i in 0..5 {
                let result = mman.alloc(1);
                assert!(result.is_ok(), "could not allocate block {} out of {}: {:?}", i, mman.blocks(), result);
            }
            assert!(mman.alloc(1).is_err());
        }
    }


    mod dead_bitmap {
        use ::libc::memory::IOVec;

        #[test]
        fn no_blocks_removed() {
            let buff = [0u8; 4200];
            let mman = super::super::SlabAllocator::new(IOVec{ptr:&buff[0], size:buff.len()}, 1024);
            assert_eq!(4, mman.blocks());
            assert_eq!(4, mman.free_blocks());
            assert_eq!(0, mman.used_blocks());
        }

        #[test]
        fn alloc_all_and_free() {
            let buff = [0u8; 4200];
            let mut mman = super::super::SlabAllocator::new(IOVec{ptr:&buff[0], size:buff.len()}, 1024);

            let mut blocks = [IOVec{ptr:0 as *const u8, size:0}; 4];
            for i in 0..mman.blocks() {
                let blk = mman.alloc(1).expect("could not allocate one block");
                unsafe { ::libc::memory::memset(blk.as_mut(), 0xA0 + (i as u8), blk.size); }
                assert_eq!(i+1, mman.used_blocks(), "incorrect used block count after allocation");
                blocks[i] = blk;
            }

            assert_eq!(4, mman.used_blocks(), "incorrect used block count after allocating all");
            assert_eq!(0, mman.free_blocks(), "incorrect free block count after allocating all");

            // TODO: add a block size getter
            for i in 0..(mman.blocks() * 1024) { assert_eq!(0xA0 + ((i / 1024) as u8), buff[i]); }

            for i in 0..blocks.len() {
                mman.free(blocks[i].clone()).expect("could not free block");
            }
            assert_eq!(0, mman.used_blocks(), "incorrect used block count after freeing all");
            assert_eq!(4, mman.free_blocks(), "incorrect free block count after freeing all");
        }
    }


    mod block_bitmap {
        use ::libc::memory::IOVec;

        #[test]
        fn round_bitmap_up() {
            let buff = [0u8; 64];
            let mman = super::super::SlabAllocator::new(IOVec{ptr:&buff[0], size:buff.len()}, 16);
            assert_eq!(3, mman.blocks());
            assert_eq!(3, mman.free_blocks());
            assert_eq!(0, mman.used_blocks());
        }


        #[test]
        fn multiblock_bitmap() {
            let buff = [0u8; 1024]; // 1024 bytes @ 8byte blocks == 128 blocks == 16 bytes of bitmap == 2 blocks
            let mman = super::super::SlabAllocator::new(IOVec{ptr:&buff[0], size:buff.len()}, 8);
            assert_eq!(126, mman.blocks());
            assert_eq!(mman.blocks(), mman.free_blocks());
            assert_eq!(0, mman.used_blocks());
        }

        #[test]
        fn alloc_all_and_free() {
            let buff = [0u8; 512]; // 512 bytes @ 4byte blocks == 128 blocks == 16 bitmap bytes
            let mut mman = super::super::SlabAllocator::new(IOVec{ptr:&buff[0], size:buff.len()}, 4);

            let mut blocks = [IOVec{ptr:0 as *const u8, size:0}; 124]; // only 124 data blocks
            for i in 0..mman.blocks() {
                let blk = mman.alloc(1).expect("could not allocate one block");
                unsafe { ::libc::memory::memset(blk.as_mut(), 0x30 + (i as u8), blk.size); }
                assert_eq!(i+1, mman.used_blocks(), "incorrect used block count after allocation");
                blocks[i] = blk;
            }

            assert_eq!(124, mman.used_blocks(), "incorrect used block count after allocating all");
            assert_eq!(0, mman.free_blocks(), "incorrect free block count after allocating all");

            // TODO: add a block size getter
            for i in 16..(mman.blocks() * 4) { // start at 16 to skip the bitmap
                assert_eq!(0x30 + (((i-16) / 4) as u8), buff[i],    // -16 due to bitmap [0, 16]
                    "wrong value at region[{}][{}]", (i / 4), (i % 4));
            }

            for i in 0..blocks.len() {
                mman.free(blocks[i].clone()).expect("could not free block");
            }
            assert_eq!(0, mman.used_blocks(), "incorrect used block count after freeing all");
            assert_eq!(124, mman.free_blocks(), "incorrect free block count after freeing all");
        }
    }
}
