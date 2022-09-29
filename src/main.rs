use bevy::prelude::*;

#[derive(Component, PartialEq)]
enum Direction {
    None,
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.52, 0.8, 0.92)))
        .insert_resource(WindowDescriptor {
            title: "Cat Statues".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(sprite_movement)
        .add_system(keyboard_input)
        .run();
}

fn setup(mut commands: Commands, assets_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteBundle {
            texture: assets_server.load("player.png"),
            transform: Transform {
                translation: Vec3::splat(0.),
                scale: Vec3::splat(2.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Direction::None);
}

fn keyboard_input(kinput: Res<Input<KeyCode>>, mut movement_direction: Query<&mut Direction>) {
    for mut direction in movement_direction.iter_mut() {
        if kinput.just_pressed(KeyCode::A) {
            *direction = Direction::Left;
        }

        if kinput.just_pressed(KeyCode::S) {
            *direction = Direction::Down;
        }

        if kinput.just_pressed(KeyCode::D) {
            *direction = Direction::Right;
        }

        if kinput.just_pressed(KeyCode::W) {
            *direction = Direction::Up;
        }

        if kinput.just_released(KeyCode::W) && *direction == Direction::Up {
            *direction = Direction::None
        }

        if kinput.just_released(KeyCode::A) && *direction == Direction::Left {
            *direction = Direction::None
        }

        if kinput.just_released(KeyCode::S) && *direction == Direction::Down {
            *direction = Direction::None
        }

        if kinput.just_released(KeyCode::D) && *direction == Direction::Right {
            *direction = Direction::None
        }
    }
}

fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
    for (sprite, mut transform) in sprite_position.iter_mut() {
        match *sprite {
            Direction::None => (),
            Direction::Up => transform.translation.y += 250. * time.delta_seconds(),
            Direction::Down => transform.translation.y -= 250. * time.delta_seconds(),
            Direction::Left => transform.translation.x -= 250. * time.delta_seconds(),
            Direction::Right => transform.translation.x += 250. * time.delta_seconds(),
        }
    }
}
