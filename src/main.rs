use bevy::prelude::*;
use revolt::plugins::{map_gen::MapPlugin, player::PlayerPlugin};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()));

    // app.add_systems(PreStartup, default_camera);
    app.add_systems(Update, display_num_entities);

    app.add_plugins(MapPlugin);
    app.add_plugins(PlayerPlugin);

    app.run();
}

// fn default_camera(mut commands: Commands) {
//     let mut camera_bundle = Camera2dBundle::default();
//     camera_bundle.transform.translation = Vec3::new(120.0, 240.0, 1.0);
//     camera_bundle.projection.scale = 0.5;
//     commands.spawn(camera_bundle);
// }

fn display_num_entities(query: Query<Entity>) {
    let num_entities = query.iter().count();

    println!("Number of entities: {}", num_entities);
}
