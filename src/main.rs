use bevy::prelude::*;
use revolt::{
    debug_tools::DebugPlugin,
    plugins::{
        camera::CenteredCameraPlugin, dojo_systems::SendTransactionsPlugin,
        dojo_to_bevy::RevoltModelsPlugin, render::DungeonRenderPlugin, stats::StatsPlugin,
        ui::UIPlugin,
    },
    states::GameStatesPlugin,
    tokio::TokioPlugin,
    torii::client::ToriiPlugin,
};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()));
    app.add_plugins(GameStatesPlugin);
    app.add_plugins(TokioPlugin);
    app.add_plugins(ToriiPlugin);
    app.add_plugins(CenteredCameraPlugin);
    app.add_plugins(RevoltModelsPlugin);
    app.add_plugins(DungeonRenderPlugin);
    app.add_plugins(SendTransactionsPlugin);
    app.add_plugins(UIPlugin);
    app.add_plugins(DebugPlugin);
    app.add_plugins(StatsPlugin);

    app.run();
}
