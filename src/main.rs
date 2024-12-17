use bevy::prelude::*;
use bevy_networking_turbulence::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

mod game;
mod networking;
mod pieces;
mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(NetworkingPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(UIPlugin)
        .run();
}