use bevy::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, generate_map);
    }
}

#[derive(Resource)]
pub struct MapResource{
    matrix: [[u32;9];9],  
}

pub fn generate_map(mut commands: Commands) {
    let map_matrix = [
        [1,1,1,1,1,1,1,1,1],
        [1,1,1,0,0,0,1,1,1],
        [1,1,0,0,0,0,0,1,1],
        [1,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,1],
        [1,1,0,0,0,0,0,1,1],
        [1,1,1,0,0,0,1,1,1],
        [1,1,1,1,1,1,1,1,1],
    ];

    commands.insert_resource(MapResource {matrix: map_matrix});

}
