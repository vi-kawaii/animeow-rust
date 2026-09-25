use bevy::prelude::*;

/// Абстракция ввода. Заполняется системой read_input каждый кадр.
/// Игровые системы читают это, а не ButtonInput<KeyCode> напрямую —
/// так потом легко подменить (геймпад, ремап, тесты).
#[derive(Resource, Default)]
pub struct InputState {
    /// Направление движения в локальных осях: x — вправо, y — вперёд.
    /// Уже нормализовано, если не ноль.
    pub move_dir: Vec2,
    pub sprint: bool,
    pub jump_pressed: bool,
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputState>()
            .add_systems(Update, read_input);
    }
}

fn read_input(keys: Res<ButtonInput<KeyCode>>, mut state: ResMut<InputState>) {
    let mut dir = Vec2::ZERO;

    if keys.pressed(KeyCode::KeyW) {
        dir.y += 1.0;
    }
    if keys.pressed(KeyCode::KeyS) {
        dir.y -= 1.0;
    }
    if keys.pressed(KeyCode::KeyA) {
        dir.x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) {
        dir.x += 1.0;
    }

    state.move_dir = if dir == Vec2::ZERO {
        Vec2::ZERO
    } else {
        dir.normalize()
    };
    state.sprint = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);
    state.jump_pressed = keys.just_pressed(KeyCode::Space);
}
