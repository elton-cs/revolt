use bevy::prelude::*;
use map::DojoMapModels;
pub mod map;
pub mod type_extractors;

pub struct DojoManualBindgenPlugin;
impl Plugin for DojoManualBindgenPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DojoMapModels);
    }
}
