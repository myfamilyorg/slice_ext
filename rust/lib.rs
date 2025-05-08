#![no_std]

extern crate errors;
extern crate result;

use core::ptr::copy;
use errors::*;
use result::Result;

pub fn slice_copy<T: Copy>(src: &[T], dst: &mut [T], len: usize) -> Result<()> {
    if dst.len() < len || src.len() < len {
        err!(OutOfBounds)
    } else {
        unsafe { copy(src.as_ptr(), dst.as_mut_ptr(), len) }
        Ok(())
    }
}

pub trait SliceExt<T: Copy> {
    fn slice_copy(&mut self, src: &[T]) -> Result<()>;
}

impl<T: Copy> SliceExt<T> for [T] {
    fn slice_copy(&mut self, src: &[T]) -> Result<()> {
        if self.len() != src.len() {
            return err!(OutOfBounds);
        }
        slice_copy(src, self, src.len())
    }
}
