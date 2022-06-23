use std::ops::{Index, IndexMut};

pub struct Vec4 {
    pub r: i32,
    pub g: i32,
    pub b: i32,
    pub a: i32,
}

impl Vec4 {
    pub fn to_rgb_string(&self) -> String {
        format!("{}, {}, {}", self.r, self.g, self.b)
    }
}

impl Index<usize> for Vec4 {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.r,
            1 => &self.g,
            2 => &self.b,
            3 => &self.a,
            _ => panic!("Internal error, index is out of bounds"),
        }
    }
}

impl IndexMut<usize> for Vec4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.r,
            1 => &mut self.g,
            2 => &mut self.b,
            3 => &mut self.a,
            _ => panic!("Internal error, index is out of bounds"),
        }
    }
}

impl Default for Vec4 {
    fn default() -> Self {
        Vec4 {
            r: 0,
            g: 0,
            b: 0,
            a: 1,
        }
    }
}