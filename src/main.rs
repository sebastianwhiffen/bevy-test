pub mod movement;
pub mod spawner;
pub mod components;


use bevy::{color::palettes::tailwind::BLUE_100, math::vec2, prelude::*};
use components::Components::Player;
use movement::movement::{asteroid_movement, is_hit_system, player_movement};
use spawner::spawner::spawn_asteroids;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (player_movement, asteroid_movement, spawn_asteroids, is_hit_system))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Player{health: 1},
        Sprite::from_color(BLUE_100, vec2(100., 100.)),
        Transform::from_xyz(100., 0., 0.),
    ));
}

