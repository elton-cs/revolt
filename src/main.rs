use bevy::prelude::*;
use revolt::{
    plugins::{
        camera::CenteredCameraPlugin, dojo_systems::SendTransactionsPlugin,
        dojo_to_bevy::map::BevyMapPlugin, map_gen::MapPlugin, player::PlayerPlugin,
        render::DungeonRenderPlugin, ui::UIPlugin,
    },
    tokio::TokioPlugin,
    torii::{call_contracts::CallContractsPlugin, client::ToriiPlugin},
};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()));
    // app.add_systems(Update, display_num_entities);

    app.add_plugins(TokioPlugin);
    app.add_plugins(ToriiPlugin);
    app.add_plugins(BevyMapPlugin);
    app.add_plugins(DungeonRenderPlugin);
    app.add_plugins(CenteredCameraPlugin);
    app.add_plugins(UIPlugin);
    app.add_plugins(SendTransactionsPlugin);
    // app.add_plugins(MapPlugin);
    // app.add_plugins(PlayerPlugin);
    // app.add_plugins(CallContractsPlugin);

    app.run();
}

fn display_num_entities(query: Query<Entity>) {
    let num_entities = query.iter().count();

    info!("Total entities: {}", num_entities);
}
