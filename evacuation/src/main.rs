use bevy::prelude::*;
use config::ConfigPlugin;
use evacuation::EvacuationPlugin;

mod basics;
mod config;
mod evacuation;

fn main() {
    let mut app = App::new();

    app.add_plugins(ConfigPlugin);
    basics::setup_basics(&mut app);
    app.add_plugins(EvacuationPlugin);

    app.run();
}
