use bevy::prelude::*;
use revolt::plugins::ONE;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_systems(Update, print_one);
    app.run();
}

fn print_one() {
    info!("{ONE}");
}
