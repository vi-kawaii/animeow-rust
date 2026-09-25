use crate::states::GameState;
use bevy::prelude::*;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), setup_loading)
            .add_systems(Update, finish_loading.run_if(in_state(GameState::Loading)))
            .add_systems(OnExit(GameState::Loading), cleanup_loading);
    }
}

#[derive(Resource)]
struct LoadingTimer(Timer);

#[derive(Component)]
struct LoadingUi;

fn setup_loading(mut commands: Commands) {
    commands.insert_resource(LoadingTimer(Timer::from_seconds(5.0, TimerMode::Once)));

    commands
        .spawn((
            LoadingUi,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Loading..."),
                TextFont {
                    font_size: FontSize::Px(48.0),
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

fn finish_loading(
    time: Res<Time>,
    mut timer: ResMut<LoadingTimer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        next_state.set(GameState::InGame);
    }
}

fn cleanup_loading(mut commands: Commands, query: Query<Entity, With<LoadingUi>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
    commands.remove_resource::<LoadingTimer>();
}
