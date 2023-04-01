use bevy::prelude::*;
use std::{
    f32::consts::PI,
    ops::{Add, BitXor, Sub},
};

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
    pub flip: bool,
}
impl TriangleCoord {
    pub fn to_world_pos(&self, primitive: Triangles) -> Transform {
        let xyz = self.to_vec3(&primitive);
        let mut t = Transform::from_xyz(xyz.x, xyz.y, xyz.z);
        match primitive.alignment {
            GridAlign::XY => t.rotate_z(PI * (self.flip as i32 + self.r) as f32),
            GridAlign::XZ => t.rotate_y(PI * (self.flip as i32 + self.r) as f32),
        }
        t
    }

    pub fn to_vec3(&self, primitive: &Triangles) -> Vec3 {
        let ab = primitive.width();
        let height = primitive.height();
        let height_adjust = 2.0 * primitive.size - height;
        let odd_row = (self.r % 2) != 0; // if odd rows, flip triangles
        let x = 0.5 * ab * (self.q + self.flip as i32) as f32; // equals (self.q as f32 / 2.0) * ab + (0.5 * ab * self.flip as i8 as f32)
        let xyz = match primitive.alignment {
            GridAlign::XY => Vec3::new(
                x,
                self.r as f32 * height + (height_adjust * self.flip.bitxor(odd_row) as i8 as f32),
                primitive.layer,
            ),
            GridAlign::XZ => Vec3::new(
                x,
                primitive.layer,
                self.r as f32 * height - (height_adjust * self.flip.bitxor(odd_row) as i8 as f32),
            ),
        };
        xyz
    }

    pub fn new(q: i32, r: i32, flip: bool) -> TriangleCoord {
        TriangleCoord { q, r, flip }
    }

    pub fn new_from_world_pos(pos: Vec3, primitive: &Triangles) -> TriangleCoord {
        let height = primitive.height();
        let height_adj = 2.0 * primitive.size - height;
        let ab = primitive.width();
        // finding Q
        let q = ((2.0 * pos.x) / ab).round() as i32;
        let flip = is_flip(&primitive, &pos, q);

        // finding R
        let odd_row = match primitive.alignment {
            GridAlign::XY => is_odd(pos.y, height),
            GridAlign::XZ => is_odd(pos.z, height),
        };

        let r_adjust = height_adj * flip.bitxor(odd_row) as i8 as f32;
        let r = match primitive.alignment {
            GridAlign::XY => (pos.y - r_adjust / height).round() as i32,
            GridAlign::XZ => (pos.z + r_adjust / height).round() as i32,
        };
        TriangleCoord::new(q, r, flip)
    }
}

fn is_odd(y: f32, heigth: f32) -> bool {
    ((y / heigth).floor() % 2.0).abs() != 0.0
}

fn is_flip(primitive: &Triangles, pos: &Vec3, q: i32) -> bool {
    let coord = TriangleCoord::new(q, 0, false);
    let vec = coord.to_vec3(primitive);
    let dist = pos.distance_squared(vec);

    let coord_flip = TriangleCoord::new(q - 1, 0, true);
    let vec_flip = coord_flip.to_vec3(primitive);
    let dist_flip = pos.distance_squared(vec_flip);
    dist_flip < dist
}

impl Coords for TriangleCoord {
    const ZERO: Self = TriangleCoord {
        q: 0,
        r: 0,
        flip: false,
    };

    fn distance(&self, other: &Self) -> u32 {
        let dist = *other - *self;
        (dist.q.abs() + dist.r.abs()) as u32 * 2 + dist.flip as u32
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
        self.q == other.q && self.r == other.r && self.flip == other.flip
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
