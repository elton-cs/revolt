use super::account::BurnerWalletAccount;
use crate::{
    tokio::TokioRuntime,
    utils::constants::{GAME_SYSTEM_CONTRACT_ADDRESS, GAME_SYSTEM_SELECTORS},
};
use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use starknet::{
    accounts::{Account, Call},
    core::utils::get_selector_from_name,
};
use starknet_crypto::Felt;

pub struct CreateGame;
impl Plugin for CreateGame {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            send_create_game_transaction.run_if(input_just_pressed(KeyCode::Enter)),
        );
    }
}

fn send_create_game_transaction(
    account_res: ResMut<BurnerWalletAccount>,
    tokio: Res<TokioRuntime>,
) {
    let actions_contract_address = Felt::from_hex(GAME_SYSTEM_CONTRACT_ADDRESS).unwrap();
    let selector = get_selector_from_name(GAME_SYSTEM_SELECTORS[0]).unwrap();

    tokio.runtime.block_on(async move {
        let result = account_res
            .0
            .execute_v1(vec![Call {
                to: actions_contract_address,
                selector,
                calldata: vec![Felt::from(0)],
            }])
            .send()
            .await;

        info!("SENT A TRANSACTION: {:?}", result);
    });
}
