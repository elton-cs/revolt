use crate::{
    plugins::ui::game_menu::CreateGameEvent, tokio::TokioRuntime, utils::constants::{GAME_SYSTEM_CONTRACT_ADDRESS, GAME_SYSTEM_SELECTORS, LOCAL_WALLET_PRIVATE_KEY, PLAYER_CONTRACT_ADDRESS, STARKNET_RS_JSONRPC_URL}
};
use bevy::prelude::*;
use starknet::{
    accounts::{Account, Call, ExecutionEncoding, SingleOwnerAccount},
    core::utils::{cairo_short_string_to_felt, get_selector_from_name}, providers::{jsonrpc::HttpTransport, JsonRpcClient, Url}, signers::{LocalWallet, SigningKey},
};
use starknet_crypto::Felt;

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

        let provider = JsonRpcClient::new(HttpTransport::new(
            Url::parse(STARKNET_RS_JSONRPC_URL).unwrap(),
        ));
        let signer = LocalWallet::from(SigningKey::from_secret_scalar(
            Felt::from_hex(LOCAL_WALLET_PRIVATE_KEY).unwrap(),
        ));
        let address = Felt::from_hex(PLAYER_CONTRACT_ADDRESS).unwrap();

        let account: SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet> =
        SingleOwnerAccount::new(
            provider,
            signer,
            address,
            cairo_short_string_to_felt("KATANA").unwrap(),
            ExecutionEncoding::New,
        );

        let actions_contract_address = Felt::from_hex(GAME_SYSTEM_CONTRACT_ADDRESS).unwrap();
        let selector = get_selector_from_name(GAME_SYSTEM_SELECTORS[0]).unwrap();

        tokio.runtime.block_on(async move {
            let result = account
                .execute_v1(vec![Call {
                    to: actions_contract_address,
                    selector,
                    calldata: vec![Felt::from(0)],
                }])
                .send()
                .await;

            info!("SENT A CREATE GAME TRANSACTION: {:?}", result);
        });
    }
}
