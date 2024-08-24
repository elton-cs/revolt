use bevy::prelude::*;
use revolt::utils::map_gen::MapPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()));
    app.add_systems(PreStartup, default_camera);
    app.add_plugins(MapPlugin);
    app.run();
}

fn default_camera(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scale = 0.2;
    commands.spawn(camera_bundle);
}
