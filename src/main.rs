use bevy::math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume};
use bevy::prelude::*;


#[derive(Component)]
struct Player;

#[derive(Component)]
struct Collider;

#[derive(Component)]
struct Projectile;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Resource)]
struct ProjectileTimer(Timer);

#[derive(Component)]
struct Enemy;

const PLAYER_SCALE: Vec3 = Vec3::new(2.0, 2.0, 2.0);
const PLAYER_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
const PLAYER_STARTING_POSITION: Vec3 = Vec3::new(0.0, -300.0, 1.0);
const PLAYER_SPEED: f32 = 100.0;
const ENEMY_STARTING_POSITION: Vec3 = Vec3::new(0.0, 20.0, 1.0);
const ENEMY_SCALE: Vec3 = Vec3::new(2.0, 2.0, 2.0);
//const PROJECTILE_SIZE: Vec3 = Vec3::splat(3.0);
//const PROJECTILE_COLOR: Color = Color::rgb(0.95, 0.95, 0.95);
const INITIAL_PROJECTILE_DIRECTION: Vec2 = Vec2::new(0.5, 0.5);
const PROJECTILE_SPEED: f32 = 400.0;
const PROJECTILE_COOLDOWN_SECONDS: f32 = 0.3;
const TOP_OF_SCREEN: f32 = 350.0;
const TIME_STEP: f32 = 1.0 / 60.0;

fn setup_game(
    mut commands: Commands,
    mut gizmo_config_store: ResMut<GizmoConfigStore>,
    asset_server: Res<AssetServer>,
) {
    //gizmo_config_store.config_mut::<AabbGizmoConfigGroup>().1.draw_all ^= true;

    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("player_ship.png"),
            transform: Transform {
                translation: PLAYER_STARTING_POSITION,
                scale: PLAYER_SCALE,
                ..default()
            },
            sprite: Sprite {
                ..default()
            },
            ..default()
        },
        Player,
        Collider,
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("enemy_ship.png"),
            transform: Transform {
                translation: ENEMY_STARTING_POSITION,
                scale: ENEMY_SCALE,
                ..default()
            },
            sprite: Sprite {
                flip_y: true,
                ..default()
            },
            ..default()
        },
        Enemy,
        Collider,
    ));
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
    time: Res<Time>,
    mut projectile_timer: ResMut<ProjectileTimer>,
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&Transform, With<Player>>,
) {
    let player_transform = query.single_mut();
    if keyboard_input.pressed(KeyCode::Space) {
        if projectile_timer.0.tick(time.delta()).finished() {
            projectile_timer.0.reset();

            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load("player_projectile.png"),
                    transform: Transform {
                        translation: player_transform.translation,
                        ..default()
                    },
                    sprite: Sprite {
                        ..default()
                    },
                    ..default()
                },
                Projectile,
                Velocity(INITIAL_PROJECTILE_DIRECTION.normalize() * PROJECTILE_SPEED),
            ));
        }
    }
}

fn move_projectiles(mut query: Query<&mut Transform, With<Projectile>>) {
    for mut collider_transform in &mut query {
        let new_projectile_position = collider_transform.translation.y + 250.0 * TIME_STEP;
        collider_transform.translation.y = new_projectile_position;
    }
}

fn destroy_projectiles(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Projectile>>,
) {
    for (collider_entity, collider_transform) in &query {
        if collider_transform.translation.y > TOP_OF_SCREEN {
            commands.entity(collider_entity).despawn();
        }
    }
}

fn check_for_collisions(
    mut commands: Commands,
    mut gizmos: Gizmos,
    projectiles_query: Query<(Entity, &Transform), With<Projectile>>,
    collider_query: Query<(Entity, &Transform, Option<&Enemy>), With<Collider>>,
) {
    for (projectile_entity, projectile_transform) in &projectiles_query {
        for (collider_entity, collider_transform, enemy_check) in &collider_query {
            let projectile_box = Aabb2d::new(
                projectile_transform.translation.truncate(),
                projectile_transform.scale.truncate() / 2.0,
            );

            gizmos.rect_2d(projectile_box.center(), 0.0, projectile_box.half_size()*2.0, Color::RED);

            let collider_box = Aabb2d::new(
                collider_transform.translation.truncate(),
                collider_transform.scale.truncate() / 2.0,
            );
            gizmos.rect_2d(collider_box.center(), 0.0, collider_box.half_size()*2.0, Color::PURPLE);

            let collision = projectile_box.intersects(&collider_box);
            if collision {
                if enemy_check.is_some() {
                    println!("Collided!");
                    // Enemy is destroyed
                    commands.entity(collider_entity).despawn();
                    // Projectile disappears too? Prevents "cutting through" a line of enemies all at once
                    commands.entity(projectile_entity).despawn();
                }
            }
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ProjectileTimer(Timer::from_seconds(PROJECTILE_COOLDOWN_SECONDS, TimerMode::Once)))
        .add_systems(Startup, setup_game)
        .add_systems(Update, check_for_collisions)
        .add_systems(FixedUpdate, (
            move_player,
            shoot_projectile,
            move_projectiles,
            destroy_projectiles
        ).before(check_for_collisions))
        .insert_resource(Time::<Fixed>::from_seconds(TIME_STEP as f64))
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}
