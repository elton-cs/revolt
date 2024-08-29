use crate::{
    plugins::{dojo_systems::account::build_account, ui::game_menu::CreateGameEvent},
    tokio::TokioRuntime,
    utils::constants::{GAME_SYSTEM_CONTRACT_ADDRESS, GAME_SYSTEM_SELECTORS, P1_ADDRESS, P1_PK},
};
use bevy::prelude::*;
use starknet::{
    accounts::{Account, Call},
    core::utils::get_selector_from_name,
};
use starknet_crypto::Felt;

use super::account::PlayerAccount;

pub struct CreateGame;
impl Plugin for CreateGame {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, send_create_game_transaction);
    }
}

fn send_create_game_transaction(
    mut events: EventReader<CreateGameEvent>,
    tokio: Res<TokioRuntime>,
) {
    for _ in events.read() {
        let actions_contract_address = Felt::from_hex(GAME_SYSTEM_CONTRACT_ADDRESS).unwrap();
        let selector = get_selector_from_name(GAME_SYSTEM_SELECTORS[0]).unwrap();
        let player_name = Felt::from_dec_str("1").unwrap();
        let calldata = vec![player_name];

        tokio.runtime.spawn(async move {
            let account = build_account(P1_PK, P1_ADDRESS);
            let result = account
                .execute_v1(vec![Call {
                    to: actions_contract_address,
                    selector,
                    calldata,
                }])
                .send()
                .await;

            info!("SENT A CREATE GAME TRANSACTION: {:?}", result);
        });
    }
}
