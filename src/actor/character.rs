use bevy::prelude::{Component, Vec3};

/// Линейная скорость персонажа (м/с).
/// Используется кинематическим контроллером (следующий заход).
#[derive(Component, Default)]
pub struct Velocity(pub Vec3);

/// Базовые скорости для locomotion. Пока берём из GameSettings,
/// но позже можно будет иметь разные для игрока / NPC / ботов.
#[derive(Component)]
pub struct MovementStats {
    pub walk_speed: f32,
    pub run_speed: f32,
    pub sprint_speed: f32,
}

impl Default for MovementStats {
    fn default() -> Self {
        Self {
            walk_speed: 4.0,
            run_speed: 7.0,
            sprint_speed: 10.0,
        }
    }
}

/// Здоровье. Компонент общий для игрока, NPC, ботов.
#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Default for Health {
    fn default() -> Self {
        Self {
            current: 100.0,
            max: 100.0,
        }
    }
}
