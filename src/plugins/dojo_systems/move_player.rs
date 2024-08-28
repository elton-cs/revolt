use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use starknet::{
    accounts::{Account, Call},
    core::utils::get_selector_from_name,
};
use starknet_crypto::Felt;

use crate::{
    tokio::TokioRuntime,
    utils::constants::{GAME_SYSTEM_CONTRACT_ADDRESS, GAME_SYSTEM_SELECTORS},
};

use super::account::BurnerWalletAccount;

pub struct MovePlayer;
impl Plugin for MovePlayer {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            send_move_transaction.run_if(input_just_pressed(KeyCode::ArrowDown)),
        );
    }
}

fn send_move_transaction(account_res: ResMut<BurnerWalletAccount>, tokio: Res<TokioRuntime>) {
    let actions_contract_address = Felt::from_hex(GAME_SYSTEM_CONTRACT_ADDRESS).unwrap();
    let selector = get_selector_from_name(GAME_SYSTEM_SELECTORS[2]).unwrap();

    // need to provide:
    // game_id: u32,
    // direction: u8,
    let game_id = Felt::from_dec_str("1").unwrap();
    let direction = Felt::from_dec_str("2").unwrap();
    let calldata = vec![game_id, direction];

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

        info!("SENT A TRANSACTION: {:?}", result);
    });
}
