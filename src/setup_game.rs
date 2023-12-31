use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

pub struct SetupGamePlugin;

impl Plugin for SetupGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane::from_size(8.0))),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            transform: Transform::from_xyz(0., -1., 0.),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(8.0, 0.005, 8.0),
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Box::new(2., 1., 2.).into()),
            material: materials.add(Color::CYAN.into()),
            transform: Transform::from_xyz(2., 0., 1.),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(2., 1., 2.),
    ));

    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 2000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-4.0, 6.5, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
