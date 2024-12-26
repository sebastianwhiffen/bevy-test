pub mod Components
{
    use bevy::prelude::Component;

    #[derive(Component)]
    pub struct Asteroid {
        pub health: i32,
    }

    #[derive(Component)]
    pub struct Player {
        pub health: i32,
    }
}