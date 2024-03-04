mod game_plugins;

use crate::game_plugins::main_game_plugin::GamePlugin;
use bevy::{app::App, DefaultPlugins};

fn main() {
    App::new().add_plugins((DefaultPlugins, GamePlugin)).run();
}
