pub mod spawner {
    use bevy::{color::palettes::tailwind::RED_500, math::vec2, prelude::*, window::PrimaryWindow};

    use crate::components::Components::*;

    pub fn spawn_asteroids(
        mut commands: Commands,
        asteroids: Query<&Asteroid>,
        window: Query<&Window, With<PrimaryWindow>>,
    ) {
        if asteroids.iter().count() < 1 {
                
                let window = window.single();
                let top_left = Vec3::new(-window.width() / 2.0, window.height() / 2.0, 0.0);

                commands.spawn((
                    Asteroid { health: 1 },
                    Sprite::from_color(RED_500, vec2(50., 50.)),
                    Transform::from_translation(top_left),
                ));
        }
    }
}
