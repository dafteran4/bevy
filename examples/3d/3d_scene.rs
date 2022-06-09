//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::{core::TaskPoolThreadAssignmentPolicy, prelude::*};

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            present_mode: bevy::window::PresentMode::Immediate,
            ..default()
        })
        .insert_resource(DefaultTaskPoolOptions {
            max_total_threads: 1,
            min_total_threads: 1,
            compute: TaskPoolThreadAssignmentPolicy {
                min_threads: 12,
                max_threads: 100,
                percent: 0.0,
            },
            io: TaskPoolThreadAssignmentPolicy {
                min_threads: 3,
                max_threads: 100,
                percent: 0.0,
            },
            async_compute: TaskPoolThreadAssignmentPolicy {
                min_threads: 3,
                max_threads: 100,
                percent: 0.0,
            },
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(sys)
        .run();
}

#[derive(Component, Copy, Clone)]
struct Position(Vec3);

#[derive(Component, Copy, Clone)]
struct Rotation(Vec3);

#[derive(Component, Copy, Clone)]
struct Velocity(Vec3);

#[derive(Component, Copy, Clone)]
struct Transform2(Mat4);

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn_batch((0..1000).map(|_| {
        (
            Transform2(Mat4::from_axis_angle(Vec3::X, 1.2)),
            Position(Vec3::X),
            Rotation(Vec3::X),
            Velocity(Vec3::X),
        )
    }));
}

fn sys(mut query: Query<(&mut Position, &mut Transform2)>) {
    query.par_for_each_mut(10, |(mut pos, mut mat)| {
        for _ in 0..100 {
            mat.0 = mat.0.inverse();
        }

        pos.0 = mat.0.transform_vector3(pos.0);
    });
}
