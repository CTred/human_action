use bevy::prelude::*;

mod grids;
use grids::{coordinates::TriangleNeighbours, primitives::*, GridConfig, GridPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(GridPlugin::<Triangles>::new(Triangles {
            size: 0.1,
            alignment: GridAlign::XZ,
            neighbors: TriangleNeighbours::Expanded,
            layer: 1.0,
        }))
        // .add_plugin(GridPlugin::<Squares>::new(Squares {
        //     size: 1.0,
        //     alignment: GridAlign::XZ,
        //     layer: 2.0,
        // }))
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    grid: Res<GridConfig<Triangles>>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 3.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(
            shape::Plane {
                size: 10.,
                subdivisions: 4,
            }
            .into(),
        ),
        material: materials.add(StandardMaterial {
            base_color: Color::GRAY,
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

    let (mesh, coords) = grid.to_grid(100, 100);
    let handle = meshes.add(mesh);
    for c in coords {
        let pos = c.to_world_pos(grid.0);
        commands.spawn(PbrBundle {
            mesh: handle.clone(),
            material: materials.add(StandardMaterial {
                base_color: Color::GREEN,
                ..default()
            }),
            transform: Transform::from_xyz(pos.x, pos.y, pos.z),
            ..default()
        });
    }
    // for coord in coordinates.iter() {
    //     commands.spawn(PbrBundle {
    //         mesh: meshes.add(mesh),
    //         material: materials.add(StandardMaterial {
    //             base_color: Color::GREEN,
    //             ..default()
    //         }),
    //         transform: coord,
    //         ..default()
    //     });
    // }
}
