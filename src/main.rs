use bevy::prelude::*;
use revolt::{
    plugins::{
        camera::CenteredCameraPlugin, dojo_systems::SendTransactionsPlugin, dojo_to_bevy::RevoltModelsPlugin, render::DungeonRenderPlugin, ui::UIPlugin
    }, states::GameStatesPlugin, tokio::TokioPlugin, torii::client::ToriiPlugin
};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()));
    // app.add_systems(Update, display_num_entities);

    app.add_plugins(GameStatesPlugin);
    app.add_plugins(TokioPlugin);
    app.add_plugins(ToriiPlugin);
    app.add_plugins(CenteredCameraPlugin);
    app.add_plugins(RevoltModelsPlugin);
    app.add_plugins(DungeonRenderPlugin);
    app.add_plugins(SendTransactionsPlugin);
    // app.add_plugins(WorldInspectorPlugin::new());
    app.add_plugins(UIPlugin);
    // app.add_plugins(GameMenuPlugin);

    app.run();
}

// fn display_num_entities(query: Query<Entity>) {
//     let num_entities = query.iter().count();
//     info!("Total entities: {}", num_entities);
// }
