use async_channel::{bounded, Receiver, Sender};
use bevy::prelude::*;

use crate::{
    plugins::{dojo_systems::account::PlayerAccount, dojo_to_bevy::player::PlayerModel},
    states::GameStates,
    utils::constants::{ATTACK_Z_HEIGHT, TILE_SCALE, TILE_SIZE},
};
pub struct AttackPlugin;
impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        // app.add_event::<TriggerAttackAnimation>();
        // app.add_systems(Update, check_if_player_frozen);
        app.add_systems(Startup, setup_attack_resource);
        app.add_systems(
            Update,
            (animate_attack)
                .chain()
                .run_if(in_state(GameStates::InGame)),
        );
    }
}

#[derive(Resource)]
pub struct AttackResource {
    rx: Receiver<u8>,
    pub tx: Sender<u8>,
}

fn setup_attack_resource(mut commands: Commands) {
    let (tx, rx) = bounded(16);
    commands.insert_resource(AttackResource { rx, tx });
}

fn animate_attack(
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut commands: Commands,
    query_players: Query<&PlayerModel>,
    attack_resource: Res<AttackResource>,
) {
    let texture2: Handle<Image> =
        asset_server.load("cooked_by_hpmnk/GroundPoundV2_Sheet_1X13f_48x.png");
    let map_layout = TextureAtlasLayout::from_grid(UVec2::splat(48), 13, 1, None, None);
    let texture_atlas_layout_handle = texture_atlas_layouts.add(map_layout);

    match attack_resource.rx.try_recv() {
        Ok(_) => {
            for player in query_players.iter() {
                let (x, y) = (player.pos_x, player.pos_y);
                let (x, y) = (x as f32 * TILE_SIZE, y as f32 * TILE_SIZE * -1.0);

                let mut transform = Transform::from_translation(Vec3::new(x, y, ATTACK_Z_HEIGHT));
                transform.scale = TILE_SCALE;

                let texture_atlas = TextureAtlas {
                    layout: texture_atlas_layout_handle.clone(),
                    index: 3,
                };

                let sprite_bundle = SpriteBundle {
                    transform,
                    texture: texture2.clone(),
                    ..default()
                };

                let id = commands.spawn((texture_atlas, sprite_bundle)).id();
            }
        }
        Err(err) => {
            if err != async_channel::TryRecvError::Empty {
                error!("Error sending attack via mpsc {:?}", err);
            }
        }
    }
}
