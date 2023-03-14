use std::{
    f32::consts::{FRAC_PI_2, FRAC_PI_4, PI},
    time::Duration,
};

use bevy::{gltf::Gltf, pbr::CascadeShadowConfigBuilder, prelude::*};
use smooth_bevy_cameras::{LookTransform, LookTransformBundle, LookTransformPlugin, Smoother};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LookTransformPlugin)
        .add_systems((
            setup.on_startup(),
            setup_scene_once_loaded,
            update_camera,
            keyboard,
            animate_light_direction,
        ))
        .run();
}

#[derive(Resource)]
struct BruteHandle(Handle<Gltf>);

#[derive(Resource)]
struct BruteAnimations(Vec<Handle<AnimationClip>>);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let size = 6.0;
    let tiles = 8;

    // Insert a resource with the current scene information
    commands.insert_resource(BruteHandle(asset_server.load("brute.glb")));

    // Camera
    commands.spawn((
        LookTransformBundle {
            transform: LookTransform::new(Vec3::default(), Vec3::default(), Vec3::Y),
            smoother: Smoother::new(0.),
        },
        Camera3dBundle { ..default() },
        EnvironmentMapLight {
            diffuse_map: asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"),
            specular_map: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
        },
        FogSettings {
            color: Color::rgba(0.05, 0.05, 0.05, 1.0),
            falloff: FogFalloff::Linear {
                start: size * 2.0,
                end: size * tiles as f32,
            },
            ..default()
        },
    ));

    let mesh = meshes.add(shape::Plane::from_size(size).into());
    let mut ground: StandardMaterial = asset_server.load("ground.jpg").into();
    ground.perceptual_roughness = 1.0;
    let ground = materials.add(ground);
    let mut wall: StandardMaterial = asset_server.load("wall.jpg").into();
    wall.perceptual_roughness = 1.0;
    let wall = materials.add(wall);

    for i in -tiles..=tiles {
        commands.spawn(PbrBundle {
            mesh: mesh.clone(),
            material: wall.clone(),
            transform: Transform::from_xyz(
                i as f32 * size,
                0.5 * size,
                (tiles as f32 + 0.5) * size,
            )
            .with_rotation(Quat::from_rotation_x(-FRAC_PI_2)),
            ..default()
        });
        commands.spawn(PbrBundle {
            mesh: mesh.clone(),
            material: wall.clone(),
            transform: Transform::from_xyz(
                i as f32 * size,
                0.5 * size,
                -(tiles as f32 + 0.5) * size,
            )
            .with_rotation(Quat::from_rotation_x(FRAC_PI_2)),
            ..default()
        });
        commands.spawn(PbrBundle {
            mesh: mesh.clone(),
            material: wall.clone(),
            transform: Transform::from_xyz(
                (tiles as f32 + 0.5) * size,
                0.5 * size,
                i as f32 * size,
            )
            .with_rotation(Quat::from_rotation_z(FRAC_PI_2)),
            ..default()
        });
        commands.spawn(PbrBundle {
            mesh: mesh.clone(),
            material: wall.clone(),
            transform: Transform::from_xyz(
                -(tiles as f32 + 0.5) * size,
                0.5 * size,
                i as f32 * size,
            )
            .with_rotation(Quat::from_rotation_z(-FRAC_PI_2)),
            ..default()
        });
        commands.spawn(PbrBundle {
            mesh: mesh.clone(),
            material: wall.clone(),
            transform: Transform::from_xyz(
                i as f32 * size,
                1.5 * size,
                (tiles as f32 + 0.5) * size,
            )
            .with_rotation(Quat::from_rotation_x(-FRAC_PI_2)),
            ..default()
        });
        commands.spawn(PbrBundle {
            mesh: mesh.clone(),
            material: wall.clone(),
            transform: Transform::from_xyz(
                i as f32 * size,
                1.5 * size,
                -(tiles as f32 + 0.5) * size,
            )
            .with_rotation(Quat::from_rotation_x(FRAC_PI_2)),
            ..default()
        });
        commands.spawn(PbrBundle {
            mesh: mesh.clone(),
            material: wall.clone(),
            transform: Transform::from_xyz(
                (tiles as f32 + 0.5) * size,
                1.5 * size,
                i as f32 * size,
            )
            .with_rotation(Quat::from_rotation_z(FRAC_PI_2)),
            ..default()
        });
        commands.spawn(PbrBundle {
            mesh: mesh.clone(),
            material: wall.clone(),
            transform: Transform::from_xyz(
                -(tiles as f32 + 0.5) * size,
                1.5 * size,
                i as f32 * size,
            )
            .with_rotation(Quat::from_rotation_z(-FRAC_PI_2)),
            ..default()
        });

        for j in -tiles..=tiles {
            commands.spawn(PbrBundle {
                mesh: mesh.clone(),
                material: ground.clone(),
                transform: Transform::from_xyz(i as f32 * size, 0.0, j as f32 * size),
                ..default()
            });
        }
    }

    // Light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, 1.0, -PI / 4.)),
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: size * tiles as f32 * 2.0 / 20.0,
            maximum_distance: size * tiles as f32 * 2.0,
            ..default()
        }
        .into(),
        ..default()
    });
}

// Once the scene is loaded, start the animation
fn setup_scene_once_loaded(
    mut commands: Commands,
    brute: Res<BruteHandle>,
    gltfs: Res<Assets<Gltf>>,
    mut player: Query<&mut AnimationPlayer>,
    mut spawned: Local<bool>,
    mut animated: Local<bool>,
    animations: Option<Res<BruteAnimations>>,
) {
    if !*spawned {
        if let Some(brute) = gltfs.get(&brute.0) {
            commands.spawn(SceneBundle {
                scene: brute.scenes[0].clone_weak(),
                ..default()
            });
            commands.insert_resource(BruteAnimations(brute.animations.clone()));
            *spawned = true;
        }
    }
    if *spawned && !*animated {
        if let Ok(mut player) = player.get_single_mut() {
            player
                .play(animations.as_ref().unwrap().0[Animations::Idle as usize].clone_weak())
                .repeat();
            *animated = true;
        }
    }
}

fn update_camera(
    players: Query<(&Transform, &GlobalTransform), With<AnimationPlayer>>,
    mut cameras: Query<&mut LookTransform>,
) {
    for (t, gt) in &players {
        for mut look_transform in &mut cameras {
            let translation = gt.translation();
            look_transform.target = translation + Vec3::Y;
            look_transform.eye = translation - t.up() * 4.0 + Vec3::Y * 2.0;
        }
    }
}

#[derive(Component)]
struct Character;

#[derive(Copy, Clone)]
enum Animations {
    Idle = 1,
    RunForward = 4,
    RunBackward = 3,
    TurnLeft = 5,
    TurnRight = 6,
    Attack = 2,
    Jump = 0,
}

#[derive(Default)]
struct RootMotionRotation {
    timer: Timer,
    is_left: bool,
}

fn keyboard(
    keyboard: Res<Input<KeyCode>>,
    mut player: Query<&mut AnimationPlayer>,
    mut player_transform: Query<&mut Transform, With<AnimationPlayer>>,
    animations: Option<Res<BruteAnimations>>,
    animation_clips: Res<Assets<AnimationClip>>,
    (time, mut timer, mut rotation): (Res<Time>, Local<Timer>, Local<RootMotionRotation>),
    (mut something_pressed, mut current_direction): (Local<bool>, Local<Direction>),
) {
    if keyboard.just_released(KeyCode::Up)
        || keyboard.just_released(KeyCode::Z)
        || keyboard.just_released(KeyCode::W)
    {
        player
            .single_mut()
            .play_with_transition(
                animations.as_ref().unwrap().0[Animations::Idle as usize].clone(),
                Duration::from_secs_f32(0.25),
            )
            .repeat();
        *something_pressed = false;
        return;
    }
    if keyboard.just_released(KeyCode::Down) || keyboard.just_released(KeyCode::S) {
        player
            .single_mut()
            .play_with_transition(
                animations.as_ref().unwrap().0[Animations::Idle as usize].clone(),
                Duration::from_secs_f32(0.25),
            )
            .repeat();
        *something_pressed = false;
        return;
    }
    if *something_pressed {
        return;
    }

    let transition_delay = 1.0;

    if timer.tick(time.delta()).just_finished() {
        let Ok(mut player) = player.get_single_mut() else {return;};
        rotation.timer.unpause();
        player
            .play_with_transition(
                animations.as_ref().unwrap().0[Animations::Idle as usize].clone(),
                Duration::from_secs_f32(transition_delay),
            )
            .repeat();
        return;
    }
    if !timer.finished() {
        return;
    }

    rotation.timer.tick(time.delta());
    if !rotation.timer.finished() && !rotation.timer.paused() {
        let mut transform = player_transform.single_mut();
        // println!("{:?}", entity);
        // eprintln!("{:?}", transform.rotation);

        if rotation.is_left {
            transform.rotate(Quat::from_rotation_y(
                FRAC_PI_2 * time.delta_seconds() / rotation.timer.duration().as_secs_f32(),
            ));
        } else {
            transform.rotate(Quat::from_rotation_y(
                -FRAC_PI_2 * time.delta_seconds() / rotation.timer.duration().as_secs_f32(),
            ));
        }
        return;
    }
    if rotation.timer.just_finished() {
        let Ok(mut transform) = player_transform.get_single_mut() else {return;};

        transform.rotation = Quat::from_rotation_y(match *current_direction {
            Direction::North => 0.0,
            Direction::West => FRAC_PI_2,
            Direction::South => PI,
            Direction::Est => 3.0 * FRAC_PI_2,
        }) * Quat::from_rotation_x(FRAC_PI_2);
    }

    if keyboard.just_pressed(KeyCode::Up)
        || keyboard.just_pressed(KeyCode::Z)
        || keyboard.just_pressed(KeyCode::W)
    {
        player
            .single_mut()
            .play_with_transition(
                animations.as_ref().unwrap().0[Animations::RunForward as usize].clone(),
                Duration::from_secs_f32(0.25),
            )
            .enable_root_motion(RootMotion::on_bone(EntityPath {
                parts: vec![Name::new("Armature"), Name::new("mixamorig:Hips")],
            }))
            .repeat();
        *something_pressed = true;
        return;
    }

    if keyboard.just_pressed(KeyCode::Down) || keyboard.just_pressed(KeyCode::S) {
        player
            .single_mut()
            .play_with_transition(
                animations.as_ref().unwrap().0[Animations::RunBackward as usize].clone(),
                Duration::from_secs_f32(0.25),
            )
            .enable_root_motion(RootMotion::on_bone(EntityPath {
                parts: vec![Name::new("Armature"), Name::new("mixamorig:Hips")],
            }))
            .repeat();
        *something_pressed = true;
        return;
    }

    if keyboard.just_pressed(KeyCode::Left)
        || keyboard.just_pressed(KeyCode::A)
        || keyboard.just_pressed(KeyCode::Q)
    {
        player.single_mut().play_with_transition(
            animations.as_ref().unwrap().0[Animations::TurnLeft as usize].clone(),
            Duration::from_secs_f32(0.5),
        );
        let clip = animation_clips
            .get(&animations.as_ref().unwrap().0[Animations::TurnLeft as usize])
            .unwrap();
        *timer = Timer::new(
            Duration::from_secs_f32(clip.duration() - transition_delay),
            TimerMode::Once,
        );
        let mut rot_timer = Timer::new(Duration::from_secs_f32(transition_delay), TimerMode::Once);
        rot_timer.pause();
        *rotation = RootMotionRotation {
            timer: rot_timer,
            is_left: true,
        };
        *current_direction = match *current_direction {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::Est,
            Direction::Est => Direction::North,
        };
        return;
    }
    if keyboard.just_pressed(KeyCode::Right) || keyboard.just_pressed(KeyCode::D) {
        player.single_mut().play_with_transition(
            animations.as_ref().unwrap().0[Animations::TurnRight as usize].clone(),
            Duration::from_secs_f32(0.5),
        );
        let clip = animation_clips
            .get(&animations.as_ref().unwrap().0[Animations::TurnRight as usize])
            .unwrap();
        *timer = Timer::new(
            Duration::from_secs_f32(clip.duration() - transition_delay),
            TimerMode::Once,
        );
        let mut rot_timer = Timer::new(Duration::from_secs_f32(transition_delay), TimerMode::Once);
        rot_timer.pause();
        *rotation = RootMotionRotation {
            timer: rot_timer,
            is_left: false,
        };
        *current_direction = match *current_direction {
            Direction::North => Direction::Est,
            Direction::West => Direction::North,
            Direction::South => Direction::West,
            Direction::Est => Direction::South,
        };
        return;
    }

    if keyboard.just_pressed(KeyCode::Space) {
        player.single_mut().play_with_transition(
            animations.as_ref().unwrap().0[Animations::Attack as usize].clone(),
            Duration::from_secs_f32(0.5),
        );
        let clip = animation_clips
            .get(&animations.as_ref().unwrap().0[Animations::Attack as usize])
            .unwrap();
        *timer = Timer::new(
            Duration::from_secs_f32(clip.duration() - transition_delay),
            TimerMode::Once,
        );
        return;
    }
    if keyboard.just_pressed(KeyCode::Return) {
        player.single_mut().play_with_transition(
            animations.as_ref().unwrap().0[Animations::Jump as usize].clone(),
            Duration::from_secs_f32(0.5),
        );
        let clip = animation_clips
            .get(&animations.as_ref().unwrap().0[Animations::Jump as usize])
            .unwrap();
        *timer = Timer::new(
            Duration::from_secs_f32(clip.duration() - transition_delay),
            TimerMode::Once,
        );
        #[allow(clippy::needless_return)]
        return;
    }
}

fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
) {
    for mut transform in &mut query {
        transform.rotation = Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            time.elapsed_seconds() * PI / 50.0,
            -FRAC_PI_4,
        );
    }
}

#[derive(Default)]
enum Direction {
    #[default]
    North,
    West,
    South,
    Est,
}
