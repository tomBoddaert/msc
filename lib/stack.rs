//! A stack system

pub trait Stack {
    type Item;

    fn push(&mut self, item: Self::Item);
    fn pop(&mut self) -> Option<Self::Item>;
}

use core::ops::Rem;

#[cfg(feature = "std")]
pub use std_stacks::*;
#[cfg(feature = "std")]
mod std_stacks {
    use super::Stack;

    #[derive(Clone, Default)]
    pub struct VecStack<T: Default>(Vec<T>);

    impl<T: Default> Stack for VecStack<T> {
        type Item = T;

        fn push(&mut self, item: Self::Item) {
            self.0.push(item);
        }

        fn pop(&mut self) -> Option<Self::Item> {
            self.0.pop()
        }
    }

    impl<T: Default> VecStack<T> {
        pub fn extend(&mut self, stack: Vec<T>) {
            self.0.extend(stack);
        }
    }

    impl<T: Default> VecStack<T> {
        #[must_use]
        pub const fn new() -> Self {
            Self(Vec::new())
        }
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Copy)]
pub struct ArrayStack<const CAPACITY: usize, T: Default + Copy>([Option<T>; CAPACITY], usize);

impl<const CAPACITY: usize, T: Default + Copy> Stack for ArrayStack<CAPACITY, T> {
    type Item = T;

    fn push(&mut self, item: Self::Item) {
        self.0[self.1] = Some(item);
        self.1 = self.1.wrapping_add(1).rem(CAPACITY);
    }

    fn pop(&mut self) -> Option<Self::Item> {
        self.1 = CAPACITY.wrapping_add(self.1).wrapping_sub(1).rem(CAPACITY);
        let output = self.0[self.1];
        self.0[self.1] = None;
        output
    }
}

impl<const CAPACITY: usize, T: Default + Copy> ArrayStack<CAPACITY, T> {
    #[must_use]
    pub const fn new() -> Self {
        Self([None; CAPACITY], 0)
    }
}

impl<const CAPACITY: usize, T: Default + Copy> Default for ArrayStack<CAPACITY, T> {
    fn default() -> Self {
        Self::new()
    }
}
