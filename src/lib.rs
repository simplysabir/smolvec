use std::alloc::{self, Layout};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;
use std::mem::{self, MaybeUninit};
use std::ops::{Deref, DerefMut};
use std::ptr;

const INLINE_CAPACITY: usize = 16;

pub struct SmolVec<T> {
    len: usize,
    data: Data<T>,
}

enum Data<T> {
    Inline(MaybeUninit<[T; INLINE_CAPACITY]>),
    Heap { ptr: *mut T, capacity: usize },
}

impl<T> SmolVec<T> {
    pub fn new() -> Self {
        SmolVec {
            len: 0,
            data: Data::Inline(MaybeUninit::uninit()),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        if capacity <= INLINE_CAPACITY {
            Self::new()
        } else {
            let layout = Layout::array::<T>(capacity).unwrap();
            let ptr = unsafe { alloc::alloc(layout) as *mut T };
            SmolVec {
                len: 0,
                data: Data::Heap { ptr, capacity },
            }
        }
    }

    pub fn push(&mut self, value: T) {
        if self.len == self.capacity() {
            self.grow();
        }
        unsafe {
            let ptr = self.as_mut_ptr().add(self.len);
            ptr::write(ptr, value);
        }
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        } else {
            self.len -= 1;
            unsafe {
                let ptr = self.as_mut_ptr().add(self.len);
                Some(ptr::read(ptr))
            }
        }
    }

    pub fn clear(&mut self) {
        while self.pop().is_some() {}
    }

    pub fn capacity(&self) -> usize {
        match &self.data {
            Data::Inline(_) => INLINE_CAPACITY,
            Data::Heap { capacity, .. } => *capacity,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn grow(&mut self) {
        let new_capacity = if self.capacity() == 0 {
            INLINE_CAPACITY
        } else {
            self.capacity() * 2
        };

        let new_layout = Layout::array::<T>(new_capacity).unwrap();
        let new_ptr = unsafe { alloc::alloc(new_layout) as *mut T };

        let old_ptr = self.as_ptr();
        unsafe {
            ptr::copy_nonoverlapping(old_ptr, new_ptr, self.len);
        }

        if let Data::Heap { ptr, capacity } = self.data {
            unsafe {
                let old_layout = Layout::array::<T>(capacity).unwrap();
                alloc::dealloc(ptr as *mut u8, old_layout);
            }
        }

        self.data = Data::Heap {
            ptr: new_ptr,
            capacity: new_capacity,
        };
    }

    fn as_ptr(&self) -> *const T {
        match &self.data {
            Data::Inline(ref arr) => arr.as_ptr() as *const T,
            Data::Heap { ptr, .. } => *ptr as *const T,
        }
    }

    fn as_mut_ptr(&mut self) -> *mut T {
        match self.data {
            Data::Inline(ref mut arr) => arr.as_mut_ptr() as *mut T,
            Data::Heap { ptr, .. } => ptr,
        }
    }
}
