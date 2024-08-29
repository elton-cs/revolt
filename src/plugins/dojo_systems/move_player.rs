use crate::{
    plugins::{dojo_systems::account::build_account, dojo_to_bevy::game::GameModel},
    states::GameStates,
    tokio::TokioRuntime,
    utils::constants::{GAME_SYSTEM_CONTRACT_ADDRESS, GAME_SYSTEM_SELECTORS},
};
use bevy::{
    input::{
        keyboard::{Key, KeyboardInput},
        ButtonState,
    },
    prelude::*,
};
use starknet::{
    accounts::{Account, Call},
    core::utils::get_selector_from_name,
};
use starknet_crypto::Felt;

use super::account::PlayerAccount;

pub struct MovePlayer;
impl Plugin for MovePlayer {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            send_move_transaction.run_if(in_state(GameStates::InGame)),
        );
    }
}

fn send_move_transaction(
    tokio: Res<TokioRuntime>,
    mut evr_kbd: EventReader<KeyboardInput>,
    player_account: Res<PlayerAccount>,
    query_game: Query<&GameModel>,
) {
    let mut should_execute = false;
    let mut direction = 0;

    for ev in evr_kbd.read() {
        // We don't care about key releases, only key presses
        if ev.state == ButtonState::Released {
            continue;
        }
        match &ev.logical_key {
            Key::ArrowUp => {
                direction = 1;
                should_execute = true;
            }
            Key::ArrowDown => {
                direction = 2;
                should_execute = true;
            }
            Key::ArrowLeft => {
                direction = 3;
                should_execute = true;
            }
            Key::ArrowRight => {
                direction = 4;
                should_execute = true;
            }
            _ => {}
        }
    }

    if should_execute {
        let pk = player_account.pk.clone();
        let address = player_account.address.clone();

        let actions_contract_address = Felt::from_hex(GAME_SYSTEM_CONTRACT_ADDRESS).unwrap();
        let selector = get_selector_from_name(GAME_SYSTEM_SELECTORS[2]).unwrap();

        // let game_id = Felt::from_dec_str("1").unwrap();
        let game_id = query_game.single().id;
        let game_id = Felt::from_dec_str(game_id.to_string().as_str()).unwrap();
        let direction = Felt::from_dec_str(direction.to_string().as_str()).unwrap();
        let calldata = vec![game_id, direction];

        tokio.runtime.spawn(async move {
            let account = build_account(pk.as_str(), address.as_str());
            let result = account
                .execute_v1(vec![Call {
                    to: actions_contract_address,
                    selector,
                    calldata,
                }])
                .send()
                .await;

            info!("SENT A MOVE({:?}) TRANSACTION: {:?}", direction, result);
        });
    }
}
