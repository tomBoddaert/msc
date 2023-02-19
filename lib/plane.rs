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
