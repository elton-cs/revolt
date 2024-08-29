use super::account::PlayerAccount;
use crate::{
    plugins::{dojo_systems::account::build_account, ui::game_menu::JoinGameEvent},
    tokio::TokioRuntime,
    utils::constants::{GAME_SYSTEM_CONTRACT_ADDRESS, GAME_SYSTEM_SELECTORS, P2_ADDRESS, P2_PK},
};
use bevy::prelude::*;
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

fn send_join_transaction(tokio: Res<TokioRuntime>, mut events: EventReader<JoinGameEvent>) {
    for _ in events.read() {
        let actions_contract_address = Felt::from_hex(GAME_SYSTEM_CONTRACT_ADDRESS).unwrap();
        let selector = get_selector_from_name(GAME_SYSTEM_SELECTORS[1]).unwrap();

        let player_name = Felt::from_dec_str("2").unwrap();
        let game_id = Felt::from_dec_str("1").unwrap();
        let calldata = vec![player_name, game_id];

        tokio.runtime.spawn(async move {
            let account = build_account(P2_PK, P2_ADDRESS);
            let result = account
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
