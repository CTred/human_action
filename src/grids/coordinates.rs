use bevy::prelude::*;
use std::ops::{Add, Sub};

use crate::GridAlign;

use super::primitives::{GridPrimitive, Squares, Triangles};

/// Coordinate System Trait
pub trait Coords<T = Self>
where
    T: Sized,
{
    // The coordinate at Origin
    const ZERO: T;

    // Manhattam distance between coordinates A and B in absolute terms
    fn distance(&self, other: &T) -> u32;

    // Distance from &self to Origin
    fn magnitude(&self) -> u32 {
        self.distance(&Self::ZERO)
    }

    fn neighbours(&self) -> Vec<T>;

    fn scalar_multiply(&self, scalar: i32) -> T;
}

/// TRIANGLE COORDINATES
#[derive(Clone, Copy, Debug, Hash, Default)]
pub enum TriangleNeighbours {
    Strict,
    #[default]
    Expanded,
}

#[derive(Clone, Copy, Debug, Hash, Default)]
pub struct TriangleCoord {
    pub q: i32,
    pub r: i32,
    pub side: i8,
}
impl TriangleCoord {
    pub fn to_world_pos(&self, primitive: Triangles) -> Vec3 {
        let ab = primitive.size * 30_f32.to_radians().cos() * 2.0;
        let height = (3.0_f32.sqrt() / 2.0) * ab;
        let offset = (self.r % 2) as f32 * (0.5 * ab); // if even rows, shift triangles half size to the left
        match primitive.alignment {
            GridAlign::XY => {
                return Vec3::new(
                    self.q as f32 * ab - offset,
                    self.r as f32 * height,
                    primitive.layer,
                )
            }
            GridAlign::XZ => {
                return Vec3::new(
                    self.q as f32 * ab - offset,
                    primitive.layer,
                    self.r as f32 * height,
                )
            }
        }
    }
}

impl Coords for TriangleCoord {
    const ZERO: Self = TriangleCoord {
        q: 0,
        r: 0,
        side: 0,
    };

    fn distance(&self, other: &Self) -> u32 {
        let dist = *other - *self;
        (dist.q.abs() + dist.r.abs()) as u32 * 2 + dist.side.abs() as u32
    }

    fn neighbours(&self) -> Vec<Self> {
        todo!()
    }

    fn scalar_multiply(&self, scalar: i32) -> Self {
        todo!()
    }
}

impl Eq for TriangleCoord {}
impl PartialEq for TriangleCoord {
    fn eq(&self, other: &Self) -> bool {
        self.q == other.q && self.r == other.r && self.side == other.side
    }
}
impl Add for TriangleCoord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}
impl Sub for TriangleCoord {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

/// SQUARE COORDINATES
#[derive(Clone, Copy, Debug, Hash, Default)]
pub struct SquareCoord {
    pub q: i32,
    pub r: i32,
}

impl SquareCoord {
    pub fn to_world_pos(&self, primitive: Squares) -> Vec3 {
        let size = (2.0 * primitive.size.powi(2)).sqrt();
        let height = size.sin();
        match primitive.alignment {
            GridAlign::XY => {
                return Vec3::new(
                    self.q as f32 * size,
                    self.r as f32 * height,
                    primitive.layer,
                )
            }
            GridAlign::XZ => {
                return Vec3::new(
                    self.q as f32 * size,
                    primitive.layer,
                    self.r as f32 * height,
                )
            }
        }
    }
}
impl Coords for SquareCoord {
    const ZERO: Self = SquareCoord { q: 0, r: 0 };

    fn distance(&self, other: &Self) -> u32 {
        let dist = *other - *self;
        (dist.q.abs() + dist.r.abs()) as u32
    }

    fn neighbours(&self) -> Vec<Self> {
        todo!()
    }

    fn scalar_multiply(&self, scalar: i32) -> Self {
        SquareCoord {
            q: self.q * scalar,
            r: self.r * scalar,
        }
    }
}

impl Eq for SquareCoord {}
impl PartialEq for SquareCoord {
    fn eq(&self, other: &Self) -> bool {
        self.q == other.q && self.r == other.r
    }
}
impl Add for SquareCoord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            q: self.q + rhs.q,
            r: self.r + rhs.r,
        }
    }
}
impl Sub for SquareCoord {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            q: self.q - rhs.q,
            r: self.r - rhs.r,
        }
    }
}

/// HEXAGON COORDINATES

#[derive(Clone, Copy, Debug, Hash)]
pub enum HexOrientation {
    PointyUp,
    FlatUp,
}

#[derive(Clone, Copy, Debug, Hash, Default)]
pub struct HexCoord {
    pub q: i32,
    pub r: i32,
}
impl HexCoord {
    pub fn to_world_pos(&self, size: f32, orientation: &HexOrientation) {
        todo!()
    }
}

impl Coords for HexCoord {
    const ZERO: Self = Self { q: 0, r: 0 };

    fn distance(&self, other: &Self) -> u32 {
        let s = -self.q - self.r;
        let other_s = -other.q - other.r;
        let dist = *other - *self;
        (dist.q.abs() + dist.r.abs() + (other_s - s).abs()) as u32 / 2
    }

    fn neighbours(&self) -> Vec<Self> {
        let offsets = [(1, 0), (1, -1), (0, -1), (-1, 0), (-1, 1), (0, 1)];
        let mut n = Vec::with_capacity(6);
        for (q, r) in &offsets {
            n.push(HexCoord {
                q: self.q + q,
                r: self.r + r,
            })
        }
        n
    }

    fn scalar_multiply(&self, scalar: i32) -> Self {
        HexCoord {
            q: self.q * scalar,
            r: self.r * scalar,
        }
    }
}

impl Eq for HexCoord {}
impl PartialEq for HexCoord {
    fn eq(&self, other: &Self) -> bool {
        self.q == other.q && self.r == other.r
    }
}
impl Add for HexCoord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            q: self.q + rhs.q,
            r: self.r + rhs.r,
        }
    }
}
impl Sub for HexCoord {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            q: self.q - rhs.q,
            r: self.r - rhs.r,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        todo!();
    }
}
