use std::mem::{self, MaybeUninit};
use std::ptr;
use std::ops::{Deref, DerefMut};
use std::alloc::{self, Layout};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;

const INITIAL_CAPACITY: usize = 16;

pub struct SmolVec<T> {
    len: usize,
    data: Data<T>,
}

enum Data<T> {
    Inline(MaybeUninit<[T; INITIAL_CAPACITY]>),
    Heap {
        ptr: *mut T,
        capacity: usize,
    },
}