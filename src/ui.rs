use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui)
            .add_systems(Update, update_ui);
    }
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Setup UI elements
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },
        ..default()
    });
}

fn update_ui(
    game_state: Res<State<GameState>>,
    mut ui_query: Query<&mut Text, With<UiText>>,
) {
    // Update UI based on game state
}