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
                min_threads: 0,
                max_threads: 100,
                percent: 0.0,
            },
            async_compute: TaskPoolThreadAssignmentPolicy {
                min_threads: 0,
                max_threads: 100,
                percent: 0.0,
            },
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

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
}
