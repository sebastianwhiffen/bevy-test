pub mod movement {
    use crate::components::Components::*;
    use bevy::prelude::*;

    pub fn player_movement(
        keys: Res<ButtonInput<KeyCode>>,
        mut sprite_pos: Query<&mut Transform, With<Player>>,
    ) {
        for mut transform in &mut sprite_pos {
            let keys_down = keys.get_pressed();
            for key in keys_down {
                match key {
                    KeyCode::KeyW => transform.translation.y += 1.,
                    KeyCode::KeyS => transform.translation.y -= 1.,
                    KeyCode::KeyA => transform.translation.x -= 1.,
                    KeyCode::KeyD => transform.translation.x += 1.,
                    _ => {}
                }
            }
        }
    }

    pub fn asteroid_movement(
        mut params: ParamSet<(
            Query<&mut Transform, With<Asteroid>>,
            Query<&mut Transform, With<Player>>,
        )>,
    ) {
        let player_translation = params.p1().single().translation;

        for mut pos in params.p0().iter_mut() {
            pos.translation = move_closer(pos.translation, player_translation, 1.);
        }
    }

    pub fn is_hit_system(
        mut params: ParamSet<(
            Query<&mut Transform, With<Asteroid>>,
            Query<&mut Transform, With<Player>>,
        )>,
    ) {
        let asteroid_positions: Vec<_> = params.p0().iter().map(|transform| transform.translation).collect();
        let player_positions: Vec<_> = params.p1().iter().map(|transform| transform.translation).collect();
    
        for asteroid_pos in asteroid_positions {
            for player_pos in &player_positions {
                if asteroid_pos.distance(*player_pos) < 10.0 {
                    println!("Game Over! You were hit by the red square");
                    std::process::exit(0);
                }
            }
        }
    }

    pub fn move_closer(current_pos: Vec3, target_pos: Vec3, step_size: f32) -> Vec3 {
        let direction = target_pos - current_pos;
        let distance = direction.length();

        if distance <= step_size {
            target_pos
        } else {
            current_pos + direction.normalize() * step_size
        }
    }
}
