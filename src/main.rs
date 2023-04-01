use bevy::prelude::*;
use smooth_bevy_cameras::{
    controllers::orbit::{OrbitCameraBundle, OrbitCameraController, OrbitCameraPlugin},
    LookTransformPlugin,
};

mod grids;
use grids::{coordinates::TriangleNeighbours, primitives::*, GridConfig, GridPlugin};

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .insert_resource(MouseWorldPos::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(LookTransformPlugin)
        .add_plugin(OrbitCameraPlugin::default())
        .add_plugin(GridPlugin::<Triangles>::new(Triangles {
            size: 0.1,
            alignment: GridAlign::XY,
            neighbors: TriangleNeighbours::Expanded,
            layer: 1.0,
        }))
        // .add_plugin(GridPlugin::<Squares>::new(Squares {
        //     size: 1.0,
        //     alignment: GridAlign::XZ,
        //     layer: 2.0,
        // }))
        .add_startup_system(setup)
        .add_system(mouse_to_world_pos)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    grid: Res<GridConfig<Triangles>>,
) {
    commands
        .spawn(Camera3dBundle::default())
        .insert(OrbitCameraBundle::new(
            OrbitCameraController::default(),
            Vec3::new(-2.0, 5.0, 5.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::Y,
        ))
        .insert(MainCamera);

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(
    //         shape::Plane {
    //             size: 10.,
    //             subdivisions: 4,
    //         }
    //         .into(),
    //     ),
    //     material: materials.add(StandardMaterial {
    //         base_color: Color::GRAY,
    //         ..default()
    //     }),
    //     transform: Transform::from_xyz(0.0, 0.0, 0.0),
    //     ..default()
    // });

    let (mesh, coords) = grid.to_grid(10, 10);
    let handle = meshes.add(mesh);
    for c in coords {
        let transform = c.to_world_pos(grid.0);
        commands.spawn(PbrBundle {
            mesh: handle.clone(),
            material: materials.add(StandardMaterial {
                base_color: Color::GREEN,
                ..default()
            }),
            transform,
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

#[derive(Component)]
pub struct MainCamera;

#[derive(Default, Resource)]
pub struct MouseWorldPos(pub Vec2);

pub fn mouse_to_world_pos(
    window: Query<&Window>,
    camera_query: Query<(&Transform, &Camera), With<MainCamera>>,
    mut mouse_pos: ResMut<MouseWorldPos>,
) {
    let window = window.get_single().unwrap();
    if let Some(cursor) = window.cursor_position() {
        mouse_pos.0 = cursor_to_world(cursor, &camera_query, window);
        dbg!(mouse_pos.0);
    }
}

fn cursor_to_world(
    cursor: Vec2,
    camera_query: &Query<(&Transform, &Camera), With<MainCamera>>,
    window: &Window,
) -> Vec2 {
    let (transform, camera) = camera_query.single();

    let screen_size = Vec2::new(window.width() as f32, window.height() as f32);
    let camera_position = transform.compute_matrix();
    let projection_matrix = camera.projection_matrix();

    // Normalized device coordinate cursor position from (-1, -1, -1) to (1, 1, 1)
    let cursor_ndc = (cursor / screen_size) * 2.0 - Vec2::from([1.0, 1.0]);
    // let cursor_pos_ndc_near = cursor_ndc.extend(-1.0);
    let cursor_pos_ndc_far = cursor_ndc.extend(1.0);

    let ndc_to_world = camera_position * projection_matrix.inverse();
    // let cursor_pos_near = ndc_to_world.project_point3(cursor_pos_ndc_near);
    let cursor_pos_far = ndc_to_world.project_point3(cursor_pos_ndc_far);
    // let ray_direction = cursor_pos_far - cursor_pos_near;

    cursor_pos_far.truncate()
}
