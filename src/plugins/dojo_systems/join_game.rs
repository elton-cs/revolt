use super::account::BurnerWalletAccount;
use crate::{
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

pub struct JoinGame;
impl Plugin for JoinGame {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, send_join_transaction);
    }
}

fn send_join_transaction(
    account_res: ResMut<BurnerWalletAccount>,
    tokio: Res<TokioRuntime>,
    mut evr_kbd: EventReader<KeyboardInput>,
) {
    let mut should_execute = false;

    for ev in evr_kbd.read() {
        if ev.state == ButtonState::Released {
            continue;
        }
        match &ev.logical_key {
            Key::Shift => {
                should_execute = true;
            }
            _ => {}
        }
    }

    if should_execute {
        let actions_contract_address = Felt::from_hex(GAME_SYSTEM_CONTRACT_ADDRESS).unwrap();
        let selector = get_selector_from_name(GAME_SYSTEM_SELECTORS[1]).unwrap();
        let game_id = Felt::from_dec_str("1").unwrap();

        let calldata = vec![game_id];

        tokio.runtime.block_on(async move {
            let result = account_res
                .0
                .execute_v1(vec![Call {
                    to: actions_contract_address,
                    selector,
                    calldata,
                }])
                .send()
                .await;

            info!(
                "JOINING AN EXISTING GAME AT GAME_ID({:?}): {:?}",
                game_id, result
            );
        });
    }
}
