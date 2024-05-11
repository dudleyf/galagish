use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Collider;

#[derive(Component)]
struct Projectile;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);


const PADDLE_SIZE: Vec3 = Vec3::new(120.0, 20.0, 0.0);
const PADDLE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
const TIME_STEP: f32 = 1.0 / 60.0;
const PLAYER_SPEED: f32 = 100.0;
const PROJECTILE_SIZE: Vec3 = Vec3::splat(3.0);
const PROJECTILE_COLOR: Color = Color::rgb(0.95, 0.95, 0.95);
const INITIAL_PROJECTILE_DIRECTION: Vec2 = Vec2::new(0.5, 0.5);
const PROJECTILE_SPEED: f32 = 400.0;
const PLAYER_STARTING_POSITION: Vec3 = Vec3::new(0.0, -300.0, 1.0);

fn setup_game(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: PLAYER_STARTING_POSITION,
                scale: PADDLE_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: PADDLE_COLOR,
                ..default()
            },
            ..default()
        },
        Player,
        Collider,
    ));
    commands.spawn(Camera2dBundle::default());
}


fn move_player(keyboard_input: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Transform, With<Player>>) {
    let mut paddle_transform = query.single_mut();
    let mut direction = 0.0;

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        println!("[KEYBOARD] Pressed left");
        direction -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        println!("[KEYBOARD] Pressed right");
        direction += 1.0;
    }

    let new_paddle_position = paddle_transform.translation.x + direction * PLAYER_SPEED * TIME_STEP;
    paddle_transform.translation.x = new_paddle_position;
}


fn shoot_projectile(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&Transform, With<Player>>,
) {
    let player_transform = query.single_mut();
    if keyboard_input.pressed(KeyCode::Space) {
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(Circle::default()).into(),
                material: materials.add(ColorMaterial::from(PROJECTILE_COLOR)),
                transform: Transform::from_translation(player_transform.translation)
                    .with_scale(PROJECTILE_SIZE),
                ..default()
            },
            Projectile,
            Velocity(INITIAL_PROJECTILE_DIRECTION.normalize() * PROJECTILE_SPEED),
            ));
    }
}

fn move_projectiles(mut query: Query<&mut Transform, With<Projectile>>) {
    for mut collider_transform in &mut query {
        let new_projectile_position = collider_transform.translation.y + 250.0 * TIME_STEP;
        collider_transform.translation.y = new_projectile_position;
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_game)
        .add_systems(Update, (move_player, shoot_projectile, move_projectiles))
        .run();
}
