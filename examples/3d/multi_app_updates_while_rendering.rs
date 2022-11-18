use bevy::{
    app::AllowMultipleAppStepsWhileRendering,
    input::mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    prelude::*,
    window::PresentMode,
    winit::{UpdateMode, WinitSettings},
};

fn main() {
    App::new()

        .add_plugins(DefaultPlugins)
        
        .add_startup_system(set_reactive_winit)
        .add_startup_system(allow_multi_app_steps)
        .add_startup_system(setup_simple_scene)

        .add_system(print_mouse_events_system)
        .add_system(toggle_vsync)
        .add_system(toggle_multi_app_steps)
        .add_system(frame_update)

        .run();
}


fn set_reactive_winit(mut winit_settings: ResMut<WinitSettings>) {
    let max_wait = std::time::Duration::from_millis(2);
    winit_settings.focused_mode = UpdateMode::Reactive { max_wait };
    winit_settings.unfocused_mode = UpdateMode::Reactive { max_wait };
}

fn allow_multi_app_steps(mut commands: Commands) {
    commands.insert_resource(AllowMultipleAppStepsWhileRendering(true));
}

fn toggle_vsync(input: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
    if input.just_pressed(KeyCode::V) {
        let window = windows.primary_mut();

        window.set_present_mode(if matches!(window.present_mode(), PresentMode::AutoVsync) {
            PresentMode::AutoNoVsync
        } else {
            PresentMode::AutoVsync
        });
        info!("PRESENT_MODE: {:?}", window.present_mode());
    }
}

fn toggle_multi_app_steps(input: Res<Input<KeyCode>>, mut m: ResMut<AllowMultipleAppStepsWhileRendering>) {
    if input.just_pressed(KeyCode::T) {
        m.0 = !m.0;
    }
}

fn frame_update(time: Res<Time>) {
    info!("frame update: {:?}", time.delta());
}

fn print_mouse_events_system(
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
) {
    for event in mouse_button_input_events.iter() {
        info!("{:?}", event);
    }

    for event in mouse_motion_events.iter() {
        info!("{:?}", event);
    }

    for event in mouse_wheel_events.iter() {
        info!("{:?}", event);
    }
}

/// set up a simple 3D scene
fn setup_simple_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

