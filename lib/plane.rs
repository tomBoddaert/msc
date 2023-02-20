//! A 2d array-like system

use crate::Pointer;

pub trait Plane {
    type Item;

    fn width(&self) -> usize;
    fn height(&self) -> usize;

    fn get(&self, pointer: Pointer) -> Option<&Self::Item>;
    fn get_mut(&mut self, pointer: Pointer) -> Option<&mut Self::Item>;
}

#[cfg(feature = "std")]
pub use std_planes::*;
#[cfg(feature = "std")]
mod std_planes {
    use super::{Plane, Pointer};

    /// A growable, vector-based [`Plane`] implementation
    pub struct VecPlane<T: Default>(usize, usize, Vec<Vec<T>>, T);

    impl<T: Default> Plane for VecPlane<T> {
        type Item = T;

        fn width(&self) -> usize {
            self.0
        }

        fn height(&self) -> usize {
            self.1
        }

        fn get(&self, pointer: Pointer) -> Option<&Self::Item> {
            if pointer.0 >= self.0 || pointer.1 >= self.1 {
                return None;
            }
            self.2.get(pointer.1).map_or(Some(&self.3), |row| {
                Some(row.get(pointer.0).unwrap_or(&self.3))
            })
        }

        fn get_mut(&mut self, pointer: Pointer) -> Option<&mut Self::Item> {
            if pointer.0 >= self.0 || pointer.1 >= self.1 {
                return None;
            }
            match self.2.get_mut(pointer.1) {
                Some(row) => Some(row.get_mut(pointer.0).unwrap_or(&mut self.3)),
                None => Some(&mut self.3),
            }
        }
    }

    impl<T: Default + Clone> VecPlane<T> {
        #[must_use]
        pub fn new(width: usize, height: usize) -> Self {
            let row = vec![T::default(); width];
            let plane = vec![row; height];

            Self(width, height, plane, T::default())
        }
    }

    impl<T: Default + Clone> From<Vec<Vec<T>>> for VecPlane<T> {
        fn from(mut value: Vec<Vec<T>>) -> Self {
            let width = value
                .iter()
                .fold(0, |acc, row| if row.len() > acc { row.len() } else { acc });

            value
                .iter_mut()
                .for_each(|row| row.extend(vec![T::default(); width - row.len()]));

            Self(width, value.len(), value, T::default())
        }
    }
}

#[allow(clippy::module_name_repetitions)]
/// A constant-sized, array-based [`Plane`] implementation
pub struct ArrayPlane<const WIDTH: usize, const HEIGHT: usize, T: Default>([[T; WIDTH]; HEIGHT], T);

impl<const WIDTH: usize, const HEIGHT: usize, T: Default> Plane for ArrayPlane<WIDTH, HEIGHT, T> {
    type Item = T;

    fn width(&self) -> usize {
        WIDTH
    }

    fn height(&self) -> usize {
        HEIGHT
    }

    fn get(&self, pointer: Pointer) -> Option<&Self::Item> {
        if pointer.0 >= WIDTH || pointer.1 >= HEIGHT {
            return None;
        }
        self.0.get(pointer.1).map_or(Some(&self.1), |row| {
            Some(row.get(pointer.0).unwrap_or(&self.1))
        })
    }

    fn get_mut(&mut self, pointer: Pointer) -> Option<&mut Self::Item> {
        if pointer.0 >= WIDTH || pointer.1 >= HEIGHT {
            return None;
        }
        match self.0.get_mut(pointer.1) {
            Some(row) => Some(row.get_mut(pointer.0).unwrap_or(&mut self.1)),
            None => Some(&mut self.1),
        }
    }
}

impl<const WIDTH: usize, const HEIGHT: usize, T: Default + Copy> ArrayPlane<WIDTH, HEIGHT, T> {
    #[must_use]
    pub fn new() -> Self {
        Self([[T::default(); WIDTH]; HEIGHT], T::default())
    }
}

impl<const WIDTH: usize, const HEIGHT: usize, T: Default + Copy> From<[[T; WIDTH]; HEIGHT]>
    for ArrayPlane<WIDTH, HEIGHT, T>
{
    fn from(value: [[T; WIDTH]; HEIGHT]) -> Self {
        Self(value, T::default())
    }
}

impl<const WIDTH: usize, const HEIGHT: usize, T: Default + Copy> Default
    for ArrayPlane<WIDTH, HEIGHT, T>
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use crate::plane::ArrayPlane;

    use super::{Plane, VecPlane};

    macro_rules! plane_ops {
        ( $plane:ident, set $pointer:expr => $value:literal ) => {
            *$plane.get_mut($pointer).unwrap() = $value
        };
        ( $plane:ident, get $pointer:expr => None ) => {
            assert!(matches!($plane.get($pointer), None))
        };
        ( $plane:ident, get $pointer:expr => $value:literal ) => {
            assert!(matches!($plane.get($pointer), Some($value)))
        };
    }

    macro_rules! plane_tests {
        ( $name:ident, $type:path => ($( $arg:expr ),*), $( get $pointer:expr => $value:tt ),* , ) => {
            #[test]
            fn $name() {
                let plane = <$type>::new($( $arg ),*);

                $( plane_ops!(plane, get $pointer => $value) );* ;
            }
        };
        ( $name:ident, $type:path => ($( $arg:expr ),*), $( $op:tt $pointer:expr => $value:tt ),* , ) => {
            #[test]
            fn $name() {
                let mut plane = <$type>::new($( $arg ),*);

                $( plane_ops!(plane, $op $pointer => $value) );* ;
            }
        };
    }

    plane_tests!(vec_empty, VecPlane<i8> => (4, 4),
        get (0, 0) => 0,
    );
    plane_tests!(vec_set_get, VecPlane<i8> => (4, 4),
        set (0, 0) => 5,
        get (0, 0) => 5,
    );
    plane_tests!(vec_set2_get, VecPlane<i8> => (4, 4),
        set (0, 0) => 5,
        set (1, 1) => 5,
        get (0, 0) => 5,
    );
    plane_tests!(vec_set2_get2, VecPlane<i8> => (4, 4),
        set (0, 0) => 5,
        set (1, 1) => 5,
        get (0, 0) => 5,
        get (1, 1) => 5,
    );
    plane_tests!(vec_set_get2, VecPlane<i8> => (4, 4),
        set (0, 0) => 5,
        get (0, 0) => 5,
        get (1, 1) => 0,
    );
    plane_tests!(vec_get_out_of_range, VecPlane<i8> => (4, 4),
        get (4, 0) => None,
        get (5, 0) => None,
        get (0, 4) => None,
        get (0, 5) => None,
        get (4, 4,) => None,
        get (5, 5) => None,
    );

    plane_tests!(array_empty, ArrayPlane<4, 4, i8> => (),
        get (0, 0) => 0,
    );
    plane_tests!(array_set_get, ArrayPlane<4, 4, i8> => (),
        set (0, 0) => 5,
        get (0, 0) => 5,
    );
    plane_tests!(array_set2_get, ArrayPlane<4, 4, i8> => (),
        set (0, 0) => 5,
        set (1, 1) => 5,
        get (0, 0) => 5,
    );
    plane_tests!(array_set2_get2, ArrayPlane<4, 4, i8> => (),
        set (0, 0) => 5,
        set (1, 1) => 5,
        get (0, 0) => 5,
        get (1, 1) => 5,
    );
    plane_tests!(array_set_get2, ArrayPlane<4, 4, i8> => (),
        set (0, 0) => 5,
        get (0, 0) => 5,
        get (1, 1) => 0,
    );
    plane_tests!(array_get_out_of_range, ArrayPlane<4, 4, i8> => (),
        get (4, 0) => None,
        get (5, 0) => None,
        get (0, 4) => None,
        get (0, 5) => None,
        get (4, 4,) => None,
        get (5, 5) => None,
    );
}
