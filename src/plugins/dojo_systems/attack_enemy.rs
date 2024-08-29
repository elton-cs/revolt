use crate::{
    plugins::dojo_systems::account::build_account,
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

pub struct AttackEnemy;
impl Plugin for AttackEnemy {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            send_attack_transaction.run_if(in_state(GameStates::InGame)),
        );
    }
}

fn send_attack_transaction(
    tokio: Res<TokioRuntime>,
    mut evr_kbd: EventReader<KeyboardInput>,
    player_account: Res<PlayerAccount>,
) {
    let mut should_execute = false;

    for ev in evr_kbd.read() {
        // We don't care about key releases, only key presses
        if ev.state == ButtonState::Released {
            continue;
        }
        match &ev.logical_key {
            Key::Space => {
                should_execute = true;
            }
            _ => {}
        }
    }

    if should_execute {
        let pk = player_account.pk.clone();
        let address = player_account.address.clone();

        let actions_contract_address = Felt::from_hex(GAME_SYSTEM_CONTRACT_ADDRESS).unwrap();
        let selector = get_selector_from_name(GAME_SYSTEM_SELECTORS[3]).unwrap();

        let game_id = Felt::from_dec_str("1").unwrap();
        let calldata = vec![game_id];

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

            info!("SENT AN ATTACK TRANSACTION: {:?}", result);
        });
    }

    // if should_execute {
    //     let actions_contract_address = Felt::from_hex(GAME_SYSTEM_CONTRACT_ADDRESS).unwrap();
    //     let selector = get_selector_from_name(GAME_SYSTEM_SELECTORS[3]).unwrap();
    //     let game_id = Felt::from_dec_str("1").unwrap();

    //     let calldata = vec![game_id];

    //     tokio.runtime.block_on(async move {
    //         let result = account_res
    //             .0
    //             .execute_v1(vec![Call {
    //                 to: actions_contract_address,
    //                 selector,
    //                 calldata,
    //             }])
    //             .send()
    //             .await;

    //         info!("SENT A ATTACK TRANSACTION: {:?}", result);
    //     });
    // }
}
