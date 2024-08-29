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

pub struct MovePlayer;
impl Plugin for MovePlayer {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            send_move_transaction.run_if(in_state(GameStates::InGame)),
        );
    }
}

fn send_move_transaction(tokio: Res<TokioRuntime>, mut evr_kbd: EventReader<KeyboardInput>) {
    let mut direction = 10;

    for ev in evr_kbd.read() {
        // We don't care about key releases, only key presses
        if ev.state == ButtonState::Released {
            continue;
        }
        match &ev.logical_key {
            Key::ArrowUp => {
                direction = 1;
            }
            Key::ArrowDown => {
                direction = 2;
            }
            Key::ArrowLeft => {
                direction = 3;
            }
            Key::ArrowRight => {
                direction = 4;
            }
            _ => {}
        }
    }

    if direction < 5 {
        let actions_contract_address = Felt::from_hex(GAME_SYSTEM_CONTRACT_ADDRESS).unwrap();
        let selector = get_selector_from_name(GAME_SYSTEM_SELECTORS[2]).unwrap();

        let game_id = Felt::from_dec_str("1").unwrap();
        let direction = Felt::from_dec_str(direction.to_string().as_str()).unwrap();
        let calldata = vec![game_id, direction];

        tokio.runtime.spawn(async move {
            let account = build_account();
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
