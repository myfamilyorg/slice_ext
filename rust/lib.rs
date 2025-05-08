#![no_std]

extern crate errors;
extern crate result;

use core::ptr::copy;
use core::slice::{from_raw_parts, from_raw_parts_mut};
use errors::prelude::*;
use result::Result;

pub fn slice_copy<T: Copy>(src: &[T], dst: &mut [T], len: usize) -> Result<()> {
    if dst.len() < len || src.len() < len {
        err!(OutOfBounds)
    } else {
        unsafe { copy(src.as_ptr(), dst.as_mut_ptr(), len) }
        Ok(())
    }
}

fn subslice<N>(n: &[N], off: usize, len: usize) -> Result<&[N]> {
    if off > n.len() || len.checked_add(off).map_or(true, |end| end > n.len()) {
        err!(OutOfBounds)
    } else {
        Ok(unsafe { from_raw_parts(n.as_ptr().add(off), len) })
    }
}

fn subslice_mut<N>(n: &mut [N], off: usize, len: usize) -> Result<&mut [N]> {
    if off > n.len() || len.checked_add(off).map_or(true, |end| end > n.len()) {
        err!(OutOfBounds)
    } else {
        Ok(unsafe { from_raw_parts_mut(n.as_mut_ptr().add(off), len) })
    }
}

pub trait SliceExt<T: Copy> {
    fn slice_copy(&mut self, src: &[T]) -> Result<()>;
    fn subslice(&self, offset: usize, len: usize) -> Result<&[T]>;
    fn subslice_mut(&mut self, off: usize, len: usize) -> Result<&mut [T]>;
}

impl<T: Copy> SliceExt<T> for [T] {
    fn slice_copy(&mut self, src: &[T]) -> Result<()> {
        if self.len() != src.len() {
            return err!(OutOfBounds);
        }
        slice_copy(src, self, src.len())
    }

    fn subslice(&self, offset: usize, len: usize) -> Result<&[T]> {
        subslice(self, offset, len)
    }

    fn subslice_mut(&mut self, offset: usize, len: usize) -> Result<&mut [T]> {
        subslice_mut(self, offset, len)
    }
}
