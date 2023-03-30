use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

use super::coordinates::{HexOrientation, TriangleNeighbours};

// TRAIT
pub trait GridPrimitive {
    fn to_grid(&self, width: u32, height: u32) -> (Mesh, Vec<Vec3>);
    fn to_mesh(&self) -> Mesh;
}

// PRIMITIVES: TRIANGLE
#[derive(Copy, Clone, Debug)]
pub struct Triangles {
    pub size: f32,
    pub alignment: GridAlign,
    pub neighbors: TriangleNeighbours,
    pub layer: f32,
}
impl GridPrimitive for Triangles {
    fn to_mesh(&self) -> Mesh {
        let mut vectors = Vec::with_capacity(3);
        let indices = vec![0, 1, 3, 0, 3, 2];

        for i in 0..3 {
            let vec3d_pos = corner_pos(i, 120.0, 90.0, self.size, &self.alignment);
            vectors.push([vec3d_pos.x, vec3d_pos.y, vec3d_pos.z]);
        }
        let vec1 = vectors.get(1).unwrap();
        let vec2 = vectors.get(2).unwrap();
        vectors.push([
            (vec1[0] + vec2[0]) / 2.0,
            (vec1[1] + vec2[1]) / 2.0,
            (vec1[2] + vec2[2]) / 2.0,
        ]); // adds a middle point between vertices 1 and 2.

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vectors);
        match &self.alignment {
            GridAlign::XY => {
                mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, 1.0]; 4])
            }
            GridAlign::XZ => {
                mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 1.0, 0.0]; 4])
            }
        };
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh
    }
    fn to_grid(&self, width: u32, height: u32) -> (Mesh, Vec<Vec3>) {
        todo!()
    }
}

// PRIMITIVES: SQUARES
#[derive(Clone, Copy, Debug)]
pub struct Squares {
    pub size: f32,
    pub alignment: GridAlign,
    pub layer: f32,
}

impl GridPrimitive for Squares {
    fn to_grid(&self, width: u32, height: u32) -> (Mesh, Vec<Vec3>) {
        todo!()
    }

    fn to_mesh(&self) -> Mesh {
        let mut vectors = Vec::with_capacity(4);
        let indices = vec![0, 1, 2, 0, 2, 3];
        for i in 0..4 {
            let vec3d_pos = corner_pos(i, 90.0, 45.0, self.size, &self.alignment);
            vectors.push([vec3d_pos.x, vec3d_pos.y, vec3d_pos.z]);
        }
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vectors);
        match self.alignment {
            GridAlign::XY => {
                mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, 1.0]; 4])
            }
            GridAlign::XZ => {
                mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 1.0, 0.0]; 4])
            }
        };
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh
    }
}

// PRIMITIVES: HEXAGON
#[derive(Clone, Debug)]
pub struct Hexes {
    pub size: f32,
    pub alignment: GridAlign,
    pub orientation: HexOrientation,
    pub layer: f32,
}

impl GridPrimitive for Hexes {
    fn to_mesh(&self) -> Mesh {
        let mut vectors = Vec::with_capacity(8);
        vectors.push([0.0, 0.0, 0.0]);
        let mut indices = Vec::new();
        let offset = match self.orientation {
            HexOrientation::PointyUp => 30.0,
            HexOrientation::FlatUp => 0.0,
        };
        for i in 0..6 {
            let vec3d_pos = corner_pos(i, 60.0, offset, self.size, &self.alignment);
            dbg!(&vec3d_pos);
            vectors.push([vec3d_pos.x, vec3d_pos.y, vec3d_pos.z]);
            indices.push(0);
            indices.push(i as u32 + 1);
            if i < 5 {
                indices.push(i as u32 + 2);
            } else {
                indices.push(1);
            }
        }
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vectors);
        match self.alignment {
            GridAlign::XY => {
                mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, 1.0]; 7])
            }
            GridAlign::XZ => {
                mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 1.0, 0.0]; 7])
            }
        };
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh
    }

    fn to_grid(&self, width: u32, height: u32) -> (Mesh, Vec<Vec3>) {
        todo!()
    }
}

#[derive(Copy, Clone, Debug)]
pub enum GridAlign {
    XY,
    XZ,
}

// generic funcions
fn corner_pos(i: usize, angle: f32, offset: f32, size: f32, grid_align: &GridAlign) -> Vec3 {
    let angle = angle.to_radians() * i as f32 + offset.to_radians();
    let (sin, cos) = angle.sin_cos();
    match grid_align {
        GridAlign::XY => Vec3 {
            x: size * cos,
            y: size * sin,
            z: 0.0,
        },
        GridAlign::XZ => Vec3 {
            x: size * cos,
            y: 0.0,
            z: -size * sin,
        },
    }
}
