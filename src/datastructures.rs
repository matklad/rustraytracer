use std::iter;
use std::ops::{Index, IndexMut};

pub type Idx = [u32; 2];

#[derive(Debug)]
pub struct Matrix<T> {
    shape: Idx,
    data: Vec<Vec<T>>
}

impl<T: Copy> Matrix<T> {
    pub fn fill(shape: Idx, value: T) -> Matrix<T> {
        let data = (0..shape[0]).map(|_| {
            iter::repeat(value).take(shape[1] as usize)
                .collect::<Vec<_>>()
        }).collect::<Vec<_>>();

        Matrix {
            shape: shape,
            data: data
        }
    }

    pub fn iter<'a>(&'a self) -> Box<Iterator<Item=(Idx, T)> + 'a> {
        Box::new((0..self.height()).flat_map(move |y| {
            (0..self.width()).map(move |x| ([x, y], self[[x, y]]))
        }))
    }
}

impl<T> Matrix<T> {
    pub fn width(&self) -> u32 {
        self.shape[0]
    }

    pub fn height(&self) -> u32 {
        self.shape[1]
    }
}

impl<T> Index<Idx> for Matrix<T> {
    type Output = T;

    fn index(&self, index: Idx) -> &T {
        let (x, y) = (index[0], index[1]);
        assert!(x < self.width() && y < self.height());
        &self.data[x as usize][y as usize]
    }
}

impl<T> IndexMut<Idx> for Matrix<T> {
    fn index_mut(&mut self, index: Idx) -> &mut T {
        let (x, y) = (index[0], index[1]);
        assert!(x < self.width() && y < self.height());
        &mut self.data[x as usize][y as usize]
    }
}
