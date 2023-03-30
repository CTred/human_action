use bevy::{prelude::*, utils::HashMap};
use coordinates::{Coords, TriangleCoord};
use primitives::*;

use self::coordinates::{HexCoord, SquareCoord};

pub mod coordinates;
pub mod primitives;
#[derive(Resource)]
pub struct GridMap<K: Coords>(HashMap<K, Entity>);

#[derive(Resource)]
pub struct GridConfig<T: GridPrimitive>(pub T);
impl GridConfig<Triangles> {
    pub fn to_mesh(&self) -> Mesh {
        self.0.to_mesh()
    }
    pub fn to_grid(&self, width: u32, height: u32) -> (Mesh, Vec<TriangleCoord>) {
        let mesh = self.to_mesh();
        let mut coordinates = Vec::with_capacity(width as usize * height as usize);
        for i in 0..width {
            for j in 0..height {
                coordinates.push(TriangleCoord {
                    q: i as i32,
                    r: j as i32,
                    side: 0,
                });
                coordinates.push(TriangleCoord {
                    q: i as i32,
                    r: j as i32,
                    side: 1,
                });
            }
        }
        (mesh, coordinates)
    }
}
impl GridConfig<Squares> {
    pub fn to_mesh(&self) -> Mesh {
        self.0.to_mesh()
    }
    pub fn to_grid(&self, width: u32, height: u32) -> (Mesh, Vec<SquareCoord>) {
        let mesh = self.to_mesh();
        let mut coordinates = Vec::with_capacity(width as usize * height as usize);
        for i in 0..width {
            for j in 0..height {
                coordinates.push(SquareCoord {
                    q: i as i32,
                    r: j as i32,
                });
                coordinates.push(SquareCoord {
                    q: i as i32,
                    r: j as i32,
                });
            }
        }
        (mesh, coordinates)
    }
}
impl GridConfig<Hexes> {
    pub fn to_mesh(&self) -> Mesh {
        self.0.to_mesh()
    }
    pub fn to_grid(&self, width: u32, height: u32) -> (Mesh, Vec<HexCoord>) {
        todo!()
    }
}

#[derive(Clone)]
pub struct GridPlugin<T: GridPrimitive>(pub T);
impl GridPlugin<Triangles> {
    pub fn new(primitive: Triangles) -> GridPlugin<Triangles> {
        GridPlugin(primitive)
    }
}
impl GridPlugin<Squares> {
    pub fn new(primitive: Squares) -> GridPlugin<Squares> {
        GridPlugin(primitive)
    }
}
impl GridPlugin<Hexes> {
    pub fn new(primitive: Hexes) -> GridPlugin<Hexes> {
        GridPlugin(primitive)
    }
}
impl Plugin for GridPlugin<Triangles> {
    fn build(&self, app: &mut App) {
        let resource = GridMap::<TriangleCoord>(HashMap::new());
        let object = self.0.clone();
        app.insert_resource(resource);
        app.insert_resource(GridConfig(object));
    }
}
impl Plugin for GridPlugin<Squares> {
    fn build(&self, app: &mut App) {
        let resource = GridMap::<SquareCoord>(HashMap::new());
        let object = self.0.clone();
        app.insert_resource(resource);
        app.insert_resource(GridConfig(object));
    }
}
impl Plugin for GridPlugin<Hexes> {
    fn build(&self, app: &mut App) {
        let resource = GridMap::<HexCoord>(HashMap::new());
        let object = self.0.clone();
        app.insert_resource(resource);
        app.insert_resource(GridConfig(object));
    }
}
