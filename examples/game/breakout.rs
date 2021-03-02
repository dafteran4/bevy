use bevy::{
    prelude::*,
    render::pass::ClearColor,
    sprite::collide_aabb::{collide, Collision},
};

/// An implementation of the classic game "Breakout" 
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_event::<CollisionEvent>()
        .insert_resource(Scoreboard { score: 0 })
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_startup_system(setup.system())
        .add_system(paddle_movement_system.system())
        .add_system(ball_collision_system.system())
        .add_system(wall_collision_system.system())
        .add_system(collision_event_system.system())
        .add_system(ball_movement_system.system())
        .add_system(scoreboard_system.system())
        .run();
}

struct Paddle {
    speed: f32,
}

struct Ball {
    velocity: Vec3,
}

struct Scoreboard {
    score: usize,
}

enum Collider {
    Scorable,
    Paddle,
}

enum Wall {
    Left,
    Right,
    Top,
    Bottom,
}

struct CollisionEvent(Collision);

fn setup(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Add the game's entities to our world
    commands
        // cameras
        .spawn(OrthographicCameraBundle::new_2d())
        .spawn(UiCameraBundle::default())
        // paddle
        .spawn(SpriteBundle {
            material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
            transform: Transform::from_xyz(0.0, -215.0, 0.0),
            sprite: Sprite::new(Vec2::new(120.0, 30.0)),
            ..Default::default()
        })
        .with(Paddle { speed: 500.0 })
        .with(Collider::Paddle)
        // ball
        .spawn(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 0.5, 0.5).into()),
            transform: Transform::from_xyz(0.0, -50.0, 1.0),
            sprite: Sprite::new(Vec2::new(30.0, 30.0)),
            ..Default::default()
        })
        .with(Ball {
            velocity: 400.0 * Vec3::new(0.5, -0.5, 0.0).normalize(),
        })
        // scoreboard
        .spawn(TextBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Score: ".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.5, 0.5, 1.0),
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(1.0, 0.5, 0.5),
                        },
                    },
                ],
                ..Default::default()
            },
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(5.0),
                    left: Val::Px(5.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        });

    // Add walls
    let wall_material = materials.add(Color::rgb(0.8, 0.8, 0.8).into());
    let wall_thickness = 10.0;
    let bounds = Vec2::new(900.0, 600.0);

    commands
        // left
        .spawn(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_xyz(-bounds.x / 2.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
            ..Default::default()
        })
        .with(Wall::Left)
        // right
        .spawn(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_xyz(bounds.x / 2.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
            ..Default::default()
        })
        .with(Wall::Right)
        // bottom
        .spawn(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_xyz(0.0, -bounds.y / 2.0, 0.0),
            sprite: Sprite::new(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
            ..Default::default()
        })
        .with(Wall::Bottom)
        // top
        .spawn(SpriteBundle {
            material: wall_material,
            transform: Transform::from_xyz(0.0, bounds.y / 2.0, 0.0),
            sprite: Sprite::new(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
            ..Default::default()
        })
        .with(Wall::Top);

    // Add bricks
    let brick_rows = 4;
    let brick_columns = 5;
    let brick_spacing = 20.0;
    let brick_size = Vec2::new(150.0, 30.0);
    let bricks_width = brick_columns as f32 * (brick_size.x + brick_spacing) - brick_spacing;
    // center the bricks and move them up a bit
    let bricks_offset = Vec3::new(-(bricks_width - brick_size.x) / 2.0, 100.0, 0.0);
    let brick_material = materials.add(Color::rgb(0.5, 0.5, 1.0).into());
    for row in 0..brick_rows {
        let y_position = row as f32 * (brick_size.y + brick_spacing);
        for column in 0..brick_columns {
            let brick_position = Vec3::new(
                column as f32 * (brick_size.x + brick_spacing),
                y_position,
                0.0,
            ) + bricks_offset;
            commands
                // brick
                .spawn(SpriteBundle {
                    material: brick_material.clone(),
                    sprite: Sprite::new(brick_size),
                    transform: Transform::from_translation(brick_position),
                    ..Default::default()
                })
                .with(Collider::Scorable);
        }
    }
}

fn paddle_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Paddle, &mut Transform)>,
) {
    for (paddle, mut transform) in query.iter_mut() {
        let mut direction = 0.0;
        if keyboard_input.pressed(KeyCode::Left) {
            direction -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            direction += 1.0;
        }

        let translation = &mut transform.translation;
        // move the paddle horizontally
        translation.x += time.delta_seconds() * direction * paddle.speed;
        // bound the paddle within the walls
        translation.x = translation.x.min(380.0).max(-380.0);
    }
}

fn ball_movement_system(time: Res<Time>, mut ball_query: Query<(&Ball, &mut Transform)>) {
    // clamp the timestep to stop the ball from escaping when the game starts
    let delta_seconds = f32::min(0.2, time.delta_seconds());

    for (ball, mut transform) in ball_query.iter_mut() {
        transform.translation += ball.velocity * delta_seconds;
    }
}

fn scoreboard_system(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
    for mut text in query.iter_mut() {
        text.sections[1].value = scoreboard.score.to_string();
    }
}

fn ball_collision_system(
    commands: &mut Commands,
    mut scoreboard: ResMut<Scoreboard>,
    mut ball_query: Query<(&Transform, &Sprite), With<Ball>>,
    collider_query: Query<(Entity, &Collider, &Transform, &Sprite)>,
    mut collision_events: ResMut<Events<CollisionEvent>>,
) {
    for (ball_transform, ball_sprite) in ball_query.iter_mut() {
        // check collision with scorables
        for (collider_entity, collider, transform, sprite) in collider_query.iter() {
            let collision = collide(
                ball_transform.translation,
                ball_sprite.size,
                transform.translation,
                sprite.size,
            );

            if let Some(collision) = collision {
                // scorable colliders should be despawned and increment the scoreboard on collision
                if let Collider::Scorable = *collider {
                    scoreboard.score += 1;
                    commands.despawn(collider_entity);
                }

                collision_events.send(CollisionEvent(collision));
            }
        }
    }
}

fn wall_collision_system(
    mut ball_query: Query<(&Transform, &Sprite), With<Ball>>,
    mut walls_query: Query<(&Transform, &Sprite, &Wall)>,
    mut collision_events: ResMut<Events<CollisionEvent>>,
) {
    for (ball_transform, ball_sprite) in ball_query.iter_mut() {
        let ball_pos = ball_transform.translation;
        let ball_min = ball_pos.truncate() - ball_sprite.size / 2.0;
        let ball_max = ball_pos.truncate() + ball_sprite.size / 2.0;
            
        for (wall_transform, wall_sprite, wall) in walls_query.iter_mut() { 
            let ball_pos = wall_transform.translation;   
            let wall_min = ball_pos.truncate() - wall_sprite.size / 2.0;
            let wall_max = ball_pos.truncate() + wall_sprite.size / 2.0;

            match wall {
                Wall::Left => {
                    if ball_min.x < wall_max.x {
                        collision_events.send(CollisionEvent(Collision::Right));
                    }
                },
                Wall::Right => {
                    if ball_max.x > wall_min.x {
                        collision_events.send(CollisionEvent(Collision::Left));
                    }
                },
                Wall::Bottom => {
                    if ball_min.y < wall_max.y {
                        collision_events.send(CollisionEvent(Collision::Top));
                    }
                },
                Wall::Top => {
                    if ball_max.y > wall_min.y {
                        collision_events.send(CollisionEvent(Collision::Bottom));
                    }
                },
            }
        }
    }
}

fn collision_event_system(
    mut events: EventReader<CollisionEvent>, 
    mut ball_query: Query<&mut Ball>,
) {
    for event in events.iter() {
        for mut ball in ball_query.iter_mut() {
            let velocity = &mut ball.velocity;

            let mut reflect_x = false;
            let mut reflect_y = false;

            // only reflect if the ball's velocity is going in the opposite direction of the collision
            match event.0 {
                Collision::Left => reflect_x = velocity.x > 0.0,
                Collision::Right => reflect_x = velocity.x < 0.0,
                Collision::Top => reflect_y = velocity.y < 0.0,
                Collision::Bottom => reflect_y = velocity.y > 0.0,
            }

            // reflect velocity on the x-axis if we hit something on the x-axis
            if reflect_x {
                velocity.x = -velocity.x;
            }

            // reflect velocity on the y-axis if we hit something on the y-axis
            if reflect_y {
                velocity.y = -velocity.y;
            }
        }
    }
}
