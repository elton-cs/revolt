use crate::torii::client::TempDojoEntityWrapper;
use bevy::prelude::*;

pub struct BevyMapPlugin;
impl Plugin for BevyMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, convert_to_bevy);
        app.add_systems(Update, number_of_converted);
    }
}

#[derive(Component)]
pub struct Tile {
    pub map_id: u32,
    pub pos_x: u8,
    pub pos_y: u8,
    // pub value: TileValue,
    pub value: u8,
}

// pub enum TileValue {
//     None,
//     Wall,
//     Path,
// }

fn convert_to_bevy(query: Query<(Entity, &TempDojoEntityWrapper)>, mut commands: Commands) {
    for (id, dojo_wrapper) in query.iter() {
        let dojo_entity = &dojo_wrapper.dojo_entity;

        let has_model = dojo_entity.models.len() > 0;
        let is_tile = has_model && dojo_entity.models[0].name == "revolt-Tile";

        if is_tile {
            let map_id = dojo_entity.models[0].children[0]
                .ty
                .as_primitive()
                .unwrap()
                .as_u32()
                .unwrap()
                .clone();
            let pos_x = dojo_entity.models[0].children[1]
                .ty
                .as_primitive()
                .unwrap()
                .as_u8()
                .unwrap()
                .clone();
            let pos_y = dojo_entity.models[0].children[2]
                .ty
                .as_primitive()
                .unwrap()
                .as_u8()
                .unwrap()
                .clone();
            let value = dojo_entity.models[0].children[3]
                .ty
                .as_enum()
                .unwrap()
                .option
                .unwrap()
                .clone();

            commands.spawn(Tile {
                map_id,
                pos_x,
                pos_y,
                value,
            });
            commands.entity(id).despawn();
        }
    }
}

fn number_of_converted(query: Query<&Tile>) {
    let value = query.iter().count();
    info!("Number of converted tiles: {}", value);
}
