use bevy::prelude::*;
use config::ConfigPlugin;
use game_of_life::GameOfLifePlugin;

mod basics;
mod config;
mod game_of_life;

fn main() {
    let mut app = App::new();

    app.add_plugins(ConfigPlugin);
    basics::setup_basics(&mut app);
    app.add_plugins(GameOfLifePlugin);

    app.run();
}
