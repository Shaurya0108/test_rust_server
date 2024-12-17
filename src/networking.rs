use bevy::prelude::*;
use bevy_networking_turbulence::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum NetworkMessage {
    MovePiece { from: (u8, u8), to: (u8, u8) },
    GameState(GameState),
}

pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(NetworkingPlugin::default())
            .add_systems(Update, handle_network_messages);
    }
}

fn handle_network_messages(
    mut network_events: EventReader<NetworkEvent>,
    mut game_state: ResMut<State<GameState>>,
    mut board: Query<&mut Board>,
) {
    for event in network_events.iter() {
        match event {
            NetworkEvent::Message(_, message) => {
                if let Ok(network_message) = bincode::deserialize(message) {
                    match network_message {
                        NetworkMessage::MovePiece { from, to } => {
                            if let Ok(mut board) = board.get_single_mut() {
                                move_piece(&mut board, from, to);
                            }
                        }
                        NetworkMessage::GameState(new_state) => {
                            game_state.set(new_state).unwrap();
                        }
                    }
                }
            }
            _ => {}
        }
    }
}