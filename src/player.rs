use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

const PLAYER_SHOULDER_HALF_WIDTH: f32 = 0.4;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(FixedUpdate, update_player);
    }
}

#[derive(Component)]
struct Player;

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player_material = materials.add(Color::TURQUOISE.into());

    let torso = commands
        .spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Capsule {
                    radius: 0.2,
                    depth: 0.5,
                    ..default()
                })),
                material: player_material.clone(),
                transform: Transform::from_xyz(0., 1.5, 0.),
                ..default()
            },
            Player,
            RigidBody::Static,
            Collider::capsule(0.5, 0.2),
        ))
        .id();

    let left_shoulder = commands
        .spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Capsule {
                    radius: 0.1,
                    depth: 0.3,
                    ..default()
                })),
                material: player_material.clone(),
                transform: Transform::from_xyz(PLAYER_SHOULDER_HALF_WIDTH, 1.7, 0.),
                ..default()
            },
            RigidBody::Dynamic,
            Collider::capsule(0.3, 0.1),
        ))
        .id();

    commands.spawn(
        SphericalJoint::new(torso, left_shoulder)
            .with_local_anchor_1(Vec3::new(PLAYER_SHOULDER_HALF_WIDTH, 0.5, 0.))
            .with_local_anchor_2(Vec3::new(0., 0.2, 0.)),
    );

    let left_forearm = commands
        .spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Capsule {
                    radius: 0.1,
                    depth: 0.2,
                    ..default()
                })),
                material: player_material.clone(),
                transform: Transform::from_xyz(PLAYER_SHOULDER_HALF_WIDTH, 1., 0.),
                ..default()
            },
            RigidBody::Dynamic,
            Collider::capsule(0.2, 0.1),
        ))
        .id();

    commands.spawn(
        SphericalJoint::new(left_shoulder, left_forearm)
            .with_local_anchor_1(Vec3::new(0., -0.3, 0.))
            .with_local_anchor_2(Vec3::new(0., 0.25, 0.)),
    );

    let right_shoulder = commands
        .spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Capsule {
                    radius: 0.1,
                    depth: 0.3,
                    ..default()
                })),
                material: player_material.clone(),
                transform: Transform::from_xyz(-PLAYER_SHOULDER_HALF_WIDTH, 1.7, 0.),
                ..default()
            },
            RigidBody::Dynamic,
            Collider::capsule(0.3, 0.1),
        ))
        .id();

    commands.spawn(
        SphericalJoint::new(torso, right_shoulder)
            .with_local_anchor_1(Vec3::new(-PLAYER_SHOULDER_HALF_WIDTH, 0.5, 0.))
            .with_local_anchor_2(Vec3::new(0., 0.2, 0.)),
    );

    let right_forearm = commands
        .spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Capsule {
                    radius: 0.1,
                    depth: 0.2,
                    ..default()
                })),
                material: player_material.clone(),
                transform: Transform::from_xyz(-PLAYER_SHOULDER_HALF_WIDTH, 1., 0.),
                ..default()
            },
            RigidBody::Dynamic,
            Collider::capsule(0.2, 0.1),
        ))
        .id();

    commands.spawn(
        SphericalJoint::new(right_shoulder, right_forearm)
            .with_local_anchor_1(Vec3::new(0., -0.3, 0.))
            .with_local_anchor_2(Vec3::new(0., 0.25, 0.)),
    );
}

const PLAYER_SPEED: f32 = 1.;

fn update_player(
    inputs: Res<Input<KeyCode>>,
    mut position_query: Query<&mut Position, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut position) = position_query.get_single_mut() {
        if inputs.pressed(KeyCode::W) {
            position.0.z -= PLAYER_SPEED * time.delta_seconds();
        }

        if inputs.pressed(KeyCode::S) {
            position.0.z += PLAYER_SPEED * time.delta_seconds();
        }

        if inputs.pressed(KeyCode::A) {
            position.0.x -= PLAYER_SPEED * time.delta_seconds();
        }

        if inputs.pressed(KeyCode::D) {
            position.0.x += PLAYER_SPEED * time.delta_seconds();
        }
    }
}
