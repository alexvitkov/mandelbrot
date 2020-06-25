use std::ops::{Add,Sub,Mul,Div};

#[derive(Debug, Copy, Clone)]
pub struct V2<T> {
    pub x: T,
    pub y: T
}

impl<T> From<(T,T)> for V2<T> {
    fn from(item: (T,T)) -> V2<T> {
        let (x, y)  = item;
        Self { x: x, y: y }
    }
}


impl<T> Add<Self> for &V2<T> where T: Copy + Add<T,Output=T> {
    type Output = V2<T>;
    fn add(self, rhs: Self) -> V2<T> {
        V2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    } 
}

impl<T> Sub<Self> for &V2<T> where T: Copy + Sub<T,Output=T> {
    type Output = V2<T>;
    fn sub(self, rhs: Self) -> V2<T> {
        V2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    } 
}

impl<T> Mul<Self> for &V2<T> where T: Copy + Mul<T,Output=T> {
    type Output = V2<T>;
    fn mul(self, rhs: Self) -> V2<T> {
        V2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y
        }
    } 
}

impl<T> Div<Self> for &V2<T> where T: Copy + Div<T,Output=T> {
    type Output = V2<T>;
    fn div(self, rhs: Self) -> V2<T> {
        V2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y
        }
    } 
}

pub type V2f = V2<f32>;
pub type V2u = V2<usize>;

impl From<&V2u> for V2f {
    fn from(item: &V2u) -> Self {
        V2f {
            x: item.x as f32,
            y: item.y as f32,
        }
    }
}

