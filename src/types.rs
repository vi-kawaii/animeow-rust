use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Vehicle;

/// Корень TPS-камеры. Родитель `Camera3d`.
/// Позиция — «плечо» игрока, поворот — yaw/pitch от мыши.
/// Коллизия и сглаживание — в `core/camera.rs` (Этап 2).
#[derive(Component)]
pub struct CameraRig {
    /// Горизонтальный угол (радианы)
    pub yaw: f32,
    /// Вертикальный угол (радианы), ограничен pitch_min..pitch_max
    pub pitch: f32,
    /// Дистанция от «плеча» до камеры (м)
    pub distance: f32,
    /// Минимальный pitch (взгляд вниз)
    pub pitch_min: f32,
    /// Максимальный pitch (взгляд вверх)
    pub pitch_max: f32,
}

impl Default for CameraRig {
    fn default() -> Self {
        Self {
            yaw: 0.0,
            pitch: -0.3,
            distance: 6.0,
            pitch_min: -1.4,
            pitch_max: 1.4,
        }
    }
}

#[derive(Resource, Default)]
pub struct Score {
    pub value: u32,
}

#[derive(Resource)]
pub struct GameSettings {
    /// Скорость ходьбы (м/с)
    pub walk_speed: f32,
    /// Скорость бега (м/с)
    pub run_speed: f32,
    /// Скорость спринта (м/с)
    pub sprint_speed: f32,
    /// Начальная скорость прыжка (м/с)
    pub jump_velocity: f32,
    /// Гравитация (м/с²)
    pub gravity: f32,
    /// Чувствительность мыши
    pub mouse_sensitivity: f32,
    /// Скорость транспорта (м/с)
    pub vehicle_speed: f32,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            walk_speed: 4.0,
            run_speed: 7.0,
            sprint_speed: 10.0,
            jump_velocity: 6.0,
            gravity: -20.0,
            mouse_sensitivity: 0.003,
            vehicle_speed: 25.0,
        }
    }
}
