use bevy::prelude::*;
use revolt::{
    plugins::{
        dojo_to_bevy::map::BevyMapPlugin, map_gen::MapPlugin, player::PlayerPlugin,
        render::DungeonRenderPlugin,
    },
    tokio::TokioPlugin,
    torii::{call_contracts::CallContractsPlugin, client::ToriiPlugin},
};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()));

    app.add_systems(PreStartup, default_camera);
    app.add_systems(Update, display_num_entities);

    app.add_plugins(TokioPlugin);
    app.add_plugins(ToriiPlugin);
    app.add_plugins(BevyMapPlugin);
    app.add_plugins(DungeonRenderPlugin);
    // app.add_plugins(MapPlugin);
    // app.add_plugins(PlayerPlugin);
    // app.add_plugins(CallContractsPlugin);

    app.run();
}

fn default_camera(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.transform.translation = Vec3::new(160.0, 80.0, 0.0);
    camera_bundle.projection.scale = 0.3;
    commands.spawn(camera_bundle);
}

fn display_num_entities(query: Query<Entity>) {
    let num_entities = query.iter().count();

    info!("Total entities: {}", num_entities);
}
