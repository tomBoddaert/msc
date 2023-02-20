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
    /// A growable, vector-based [`Stack`] implementation
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
/// A constant-sized, vector-based [`Stack`] implementation
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

#[cfg(test)]
mod test {
    use crate::stack::ArrayStack;

    use super::{Stack, VecStack};

    macro_rules! stack_ops {
        ( $stack:ident, pop None ) => {
            assert!(matches!($stack.pop(), None))
        };
        ( $stack:ident, pop $value:expr ) => {
            assert!(matches!($stack.pop(), Some($value)))
        };
        ( $stack:ident, push $value:expr ) => {
            $stack.push($value)
        };
    }

    macro_rules! stack_tests {
        ( $name:ident, $type:path, $( $op:tt $value:tt ),* , ) => {
            #[test]
            fn $name() {
                let mut stack = <$type>::new();

                $( stack_ops!(stack, $op $value) );* ;
            }
        };
    }

    stack_tests!(vec_empty, VecStack<i8>,
        pop None,
    );
    stack_tests!(vec_push_pop, VecStack<i8>,
        push 5,
        pop 5,
    );
    stack_tests!(vec_push2_pop, VecStack<i8>,
        push 5,
        push 10,
        pop 10,
    );
    stack_tests!(vec_push2_pop2, VecStack<i8>,
        push 5,
        push 10,
        pop 10,
        pop 5,
    );
    stack_tests!(vec_push_pop2, VecStack<i8>,
        push 5,
        pop 5,
        pop None,
    );

    stack_tests!(array_empty, ArrayStack<3, i8>,
        pop None,
    );
    stack_tests!(array_push_pop, ArrayStack<3, i8>,
        push 5,
        pop 5,
    );
    stack_tests!(array_push2_pop, ArrayStack<3, i8>,
        push 5,
        push 10,
        pop 10,
    );
    stack_tests!(array_push2_pop2, ArrayStack<3, i8>,
        push 5,
        push 10,
        pop 10,
        pop 5,
    );
    stack_tests!(array_push_pop2, ArrayStack<3, i8>,
        push 5,
        pop 5,
        pop None,
    );
    stack_tests!(array_overflow, ArrayStack<3, i8>,
        push 1,
        push 2,
        push 3,
        push 4,
        pop 4,
        pop 3,
        pop 2,
        pop None,
    );
}

#[cfg(test)]
pub mod test_stacks {
    use core::{marker::PhantomData, mem};
    use std::thread::panicking;

    use super::Stack;

    pub struct FakeStack<N>(PhantomData<N>);

    impl<N> Stack for FakeStack<N> {
        type Item = N;

        fn push(&mut self, _: Self::Item) {
            panic!("Attempted to push to a fake stack!");
        }

        fn pop(&mut self) -> Option<Self::Item> {
            panic!("Attempted to pop from a fake stack!");
        }
    }

    impl<N> FakeStack<N> {
        pub const fn new() -> Self {
            Self(PhantomData)
        }
    }

    pub struct SinglePushStack<N>(Option<N>);

    impl<N> Stack for SinglePushStack<N> {
        type Item = N;

        fn push(&mut self, item: Self::Item) {
            if matches!(self.0, Some(_)) {
                panic!("Attempted to push twice to a single-push stack!");
            }
            self.0 = Some(item);
        }

        fn pop(&mut self) -> Option<Self::Item> {
            panic!("Attempted to pop from a single-push stack!");
        }
    }

    impl<N> Drop for SinglePushStack<N> {
        fn drop(&mut self) {
            if matches!(self.0, None) && !panicking() {
                panic!("Did not push to a single-push stack!");
            }
        }
    }

    impl<N> SinglePushStack<N> {
        pub const fn new() -> Self {
            Self(None)
        }

        pub const fn get(&self) -> &Option<N> {
            &self.0
        }
    }

    pub struct SinglePopStack<N>(Option<N>);

    impl<N> Stack for SinglePopStack<N> {
        type Item = N;

        fn push(&mut self, _: Self::Item) {
            panic!("Attempted to push to a single-pop stack!");
        }

        fn pop(&mut self) -> Option<Self::Item> {
            if matches!(self.0, None) {
                panic!("Attempted to pop twice from a single-pop stack!");
            }
            mem::replace(&mut self.0, None)
        }
    }

    impl<N> Drop for SinglePopStack<N> {
        fn drop(&mut self) {
            if matches!(self.0, Some(_)) && !panicking() {
                panic!("Did not pop from a single-pop stack!");
            }
        }
    }

    impl<N> SinglePopStack<N> {
        pub const fn new(value: N) -> Self {
            Self(Some(value))
        }
    }

    pub struct TestVecStack<N>(Vec<N>);

    impl<N> Stack for TestVecStack<N> {
        type Item = N;

        fn push(&mut self, item: Self::Item) {
            self.0.push(item);
        }

        fn pop(&mut self) -> Option<Self::Item> {
            self.0.pop()
        }
    }

    impl<N> TestVecStack<N> {
        pub const fn new() -> Self {
            Self(Vec::new())
        }

        pub fn destructure(self) -> Vec<N> {
            self.0
        }
    }
}
