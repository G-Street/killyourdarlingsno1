use crate::state::GameState;
use avian2d::prelude::*;
use bevy::prelude::*;

pub struct TitlePlugin;

impl Plugin for TitlePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(OnEnter(GameState::Title), (spawn_title, pause_physics))
            .add_systems(OnEnter(GameState::Playing), unpause_physics)
            .add_systems(Update, title_system.run_if(in_state(GameState::Title)));
    }
}

fn pause_physics(mut physics_time: ResMut<Time<Physics>>) {
    physics_time.pause();
}

fn unpause_physics(mut physics_time: ResMut<Time<Physics>>) {
    physics_time.unpause();
}

#[derive(Component)]
pub struct TitleOverlay;

fn spawn_title(mut commands: Commands) {
    commands
        .spawn((
            TitleOverlay,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.0),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Feather Falling IV"),
                TextFont {
                    font_size: 64.0,
                    ..default()
                },
                Node {
                    margin: UiRect::bottom(Val::Px(50.0)),
                    ..default()
                },
            ));
            parent.spawn((
                Text::new("start falling"),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                // TODO: border to make it obvious that it's a button?
                Button,
            ));
        });
}

fn title_system(
    mut commands: Commands,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    overlay_query: Query<Entity, With<TitleOverlay>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            for entity in &overlay_query {
                commands.entity(entity).despawn();
            }
            next_state.set(GameState::Playing);
        }
    }
}
