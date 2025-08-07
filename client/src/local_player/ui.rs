use bevy::prelude::*;

use super::LocalPlayerState;

#[derive(Component)]
pub struct OnFootUi;

pub struct LocalPlayerUiPlugin;

impl Plugin for LocalPlayerUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(LocalPlayerState::OnFoot), spawn_on_foot_ui)
            .add_systems(OnExit(LocalPlayerState::OnFoot), despawn_on_foot_ui);
    }
}

fn spawn_on_foot_ui(mut commands: Commands) {
    let crosshair = (
        Node {
            width: Val::Px(10.0),
            height: Val::Px(10.0),
            ..Default::default()
        },
        BackgroundColor(Color::WHITE),
    );

    commands.spawn((
        OnFootUi,
        Name::new("On Foot UI"),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        children![crosshair],
    ));
}

fn despawn_on_foot_ui(mut commands: Commands, query: Query<Entity, With<OnFootUi>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
