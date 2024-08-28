use bevy::prelude::*;
use map::DojoMapModels;
use player::DojoPlayerModel;
pub mod map;
pub mod player;
pub mod type_extractors;

pub struct DojoManualBindgenPlugin;
impl Plugin for DojoManualBindgenPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DojoMapModels);
        app.add_plugins(DojoPlayerModel);
    }
}
