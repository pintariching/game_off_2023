use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_player, test_model_load))
            .add_systems(FixedUpdate, (update_player, update_target));
    }
}

#[derive(Component)]
struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    torso: TorsoBundle,
    bodyparts: BodyParts,
}

#[derive(Component)]
struct BodyParts {
    left_shoulder: Entity,
    left_forearm: Entity,
    right_shoulder: Entity,
    right_forearm: Entity,
    left_upper_leg: Entity,
    left_lower_leg: Entity,
    right_upper_leg: Entity,
    right_lower_leg: Entity,
}

#[derive(Bundle)]
struct BodyPart {
    mesh: PbrBundle,
    rigid_body: RigidBody,
    collider: Collider,
    gravity: GravityScale,
}

#[derive(Bundle)]
struct TargetBundle {
    target: Target,
    mesh: PbrBundle,
    raycast: RayCaster,
}

#[derive(Component)]
struct Target {
    player: Entity,
    offset: Vec3,
}

impl BodyPart {
    fn new(
        meshes: &mut ResMut<Assets<Mesh>>,
        material: Handle<StandardMaterial>,
        position: Vec3,
        radius: f32,
        height: f32,
    ) -> Self {
        Self {
            mesh: PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Capsule {
                    radius,
                    depth: height,
                    ..default()
                })),
                material,
                transform: Transform::from_translation(position),
                ..default()
            },
            rigid_body: RigidBody::Dynamic,
            collider: Collider::capsule(height, radius),
            gravity: GravityScale(1.),
        }
    }
}

#[derive(Component)]
struct Torso;

#[derive(Bundle)]
struct TorsoBundle {
    torso: Torso,
    mesh: PbrBundle,
    rigid_body: RigidBody,
    collider: Collider,
    gravity: GravityScale,
    locked_axes: LockedAxes,
    linear_damping: LinearDamping,
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let material = materials.add(Color::TURQUOISE.into());

    let torso_bundle = TorsoBundle {
        torso: Torso,
        mesh: PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule {
                radius: 0.2,
                depth: 0.5,
                ..default()
            })),
            material: material.clone(),
            transform: Transform::from_xyz(0., 1.5, 0.),
            ..default()
        },
        rigid_body: RigidBody::Dynamic,
        collider: Collider::capsule(0.5, 0.2),
        gravity: GravityScale(0.),
        locked_axes: LockedAxes::new()
            .lock_rotation_x()
            .lock_rotation_y()
            .lock_rotation_z()
            .lock_translation_y(),
        linear_damping: LinearDamping(5.),
    };

    let left_shoulder = commands
        .spawn(BodyPart::new(
            &mut meshes,
            material.clone(),
            Vec3::new(0.4, 1.7, 0.),
            0.1,
            0.3,
        ))
        .id();

    let left_forearm = commands
        .spawn(BodyPart::new(
            &mut meshes,
            material.clone(),
            Vec3::new(0.4, 1.1, 0.),
            0.1,
            0.2,
        ))
        .id();

    let right_shoulder = commands
        .spawn(BodyPart::new(
            &mut meshes,
            material.clone(),
            Vec3::new(-0.4, 1.7, 0.),
            0.1,
            0.3,
        ))
        .id();

    let right_forearm = commands
        .spawn(BodyPart::new(
            &mut meshes,
            material.clone(),
            Vec3::new(-0.4, 1.1, 0.),
            0.1,
            0.2,
        ))
        .id();

    let left_upper_leg = commands
        .spawn(BodyPart::new(
            &mut meshes,
            material.clone(),
            Vec3::new(0.2, 0.6, 0.),
            0.13,
            0.3,
        ))
        .id();

    let left_lower_leg = commands
        .spawn(BodyPart::new(
            &mut meshes,
            material.clone(),
            Vec3::new(0.2, -0.1, 0.),
            0.13,
            0.3,
        ))
        .id();

    let right_upper_leg = commands
        .spawn(BodyPart::new(
            &mut meshes,
            material.clone(),
            Vec3::new(-0.2, 0.6, 0.),
            0.13,
            0.3,
        ))
        .id();

    let right_lower_leg = commands
        .spawn(BodyPart::new(
            &mut meshes,
            material.clone(),
            Vec3::new(-0.2, -0.1, 0.),
            0.13,
            0.3,
        ))
        .id();

    let player = commands
        .spawn((PlayerBundle {
            player: Player,
            torso: torso_bundle,
            bodyparts: BodyParts {
                left_shoulder,
                left_forearm,
                right_shoulder,
                right_forearm,
                left_upper_leg,
                left_lower_leg,
                right_upper_leg,
                right_lower_leg,
            },
        },))
        .id();

    commands.spawn(TargetBundle {
        target: Target {
            player,
            offset: Vec3::new(0., 0., 0.5),
        },
        mesh: PbrBundle {
            mesh: meshes.add(
                shape::UVSphere {
                    radius: 0.2,
                    ..default()
                }
                .into(),
            ),
            material: materials.add((Color::RED).into()),
            transform: Transform::from_xyz(0., 0.5, 0.),
            ..default()
        },
        raycast: RayCaster::new(Vec3::ZERO, Vec3::NEG_Y).with_query_filter(
            SpatialQueryFilter::new().without_entities([
                left_shoulder,
                left_forearm,
                right_shoulder,
                right_forearm,
                left_upper_leg,
                left_lower_leg,
                right_upper_leg,
                right_lower_leg,
            ]),
        ),
    });

    commands.spawn(
        SphericalJoint::new(player, left_shoulder)
            .with_local_anchor_1(Vec3::new(0.4, 0.5, 0.))
            .with_local_anchor_2(Vec3::new(0., 0.2, 0.)),
    );

    commands.spawn(
        RevoluteJoint::new(left_shoulder, left_forearm)
            .with_local_anchor_1(Vec3::new(0., -0.3, 0.))
            .with_local_anchor_2(Vec3::new(0., 0.25, 0.)),
    );

    commands.spawn(
        SphericalJoint::new(player, right_shoulder)
            .with_local_anchor_1(Vec3::new(-0.4, 0.5, 0.))
            .with_local_anchor_2(Vec3::new(0., 0.2, 0.)),
    );

    commands.spawn(
        RevoluteJoint::new(right_shoulder, right_forearm)
            .with_local_anchor_1(Vec3::new(0., -0.3, 0.))
            .with_local_anchor_2(Vec3::new(0., 0.25, 0.)),
    );

    commands.spawn(
        SphericalJoint::new(player, left_upper_leg)
            .with_local_anchor_1(Vec3::new(0.25, -0.4, 0.))
            .with_local_anchor_2(Vec3::new(0., 0.4, 0.)),
    );

    commands.spawn(
        RevoluteJoint::new(left_upper_leg, left_lower_leg)
            .with_local_anchor_1(Vec3::new(0., -0.35, 0.))
            .with_local_anchor_2(Vec3::new(0., 0.25, 0.)),
    );

    commands.spawn(
        SphericalJoint::new(player, right_upper_leg)
            .with_local_anchor_1(Vec3::new(-0.25, -0.4, 0.))
            .with_local_anchor_2(Vec3::new(0., 0.4, 0.)),
    );

    commands.spawn(
        RevoluteJoint::new(right_upper_leg, right_lower_leg)
            .with_local_anchor_1(Vec3::new(0., -0.35, 0.))
            .with_local_anchor_2(Vec3::new(0., 0.25, 0.)),
    );
}

const PLAYER_SPEED: f32 = 20.;

fn update_player(
    inputs: Res<Input<KeyCode>>,
    mut torso_query: Query<(&mut Position, &mut LinearVelocity), With<Torso>>,
    time: Res<Time>,
) {
    if let Ok((mut position, mut velocity)) = torso_query.get_single_mut() {
        if inputs.pressed(KeyCode::W) {
            velocity.0.z -= PLAYER_SPEED * time.delta_seconds();
        }

        if inputs.pressed(KeyCode::S) {
            velocity.0.z += PLAYER_SPEED * time.delta_seconds();
        }

        if inputs.pressed(KeyCode::A) {
            velocity.0.x -= PLAYER_SPEED * time.delta_seconds();
        }

        if inputs.pressed(KeyCode::D) {
            velocity.0.x += PLAYER_SPEED * time.delta_seconds();
        }

        if inputs.pressed(KeyCode::ShiftLeft) {
            position.0.y += 0.5 * time.delta_seconds();
        }

        if inputs.pressed(KeyCode::ControlLeft) {
            position.0.y -= 0.5 * time.delta_seconds();
        }
    }
}

fn update_target(
    mut target_query: Query<(&mut RayCaster, &RayHits, &Target, &mut Transform)>,
    player_query: Query<&Position, With<Player>>,
) {
    for (mut ray, hits, target, mut transform) in target_query.iter_mut() {
        let hit = if hits.is_empty() {
            continue;
        } else {
            hits.as_slice().first().unwrap()
        };

        let point = ray.global_origin() + ray.direction * hit.time_of_impact;

        let player_position = player_query.get(target.player).unwrap();
        let mut new_pos = player_position.0 + target.offset;
        new_pos.y = point.y;

        transform.translation = new_pos;

        ray.origin = player_position.0 + target.offset;
    }

    // if let (Ok((ray, hits)), Ok(mut transform)) =
    //     (raycast_query.get_single(), target_query.get_single_mut())
    // {
    //     for hit in hits.iter() {
    //         let point = ray.global_origin() + ray.global_direction() * hit.time_of_impact;
    //         transform.translation = point;
    //     }
    // }
}

fn test_model_load(mut commands: Commands, assets: Res<AssetServer>) {
    let mesh = PbrBundle {
        mesh: assets.load("test.gltf#Mesh0/Primitive0"),
        transform: Transform::from_xyz(1., 0., 0.5),
        ..default()
    };

    commands.spawn((mesh, AsyncCollider(ComputedCollider::TriMesh)));
}
