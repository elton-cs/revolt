use crate::{
    tokio::TokioRuntime,
    utils::constants::{
        LOCAL_WALLET_PRIVATE_KEY, PLAYER_CONTRACT_ADDRESS, STARKNET_RS_JSONRPC_URL,
    },
};

use super::client::setup_torii_client;
use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use starknet::{
    accounts::{Account, Call, ExecutionEncoding, SingleOwnerAccount},
    core::utils::{cairo_short_string_to_felt, get_selector_from_name},
    providers::{jsonrpc::HttpTransport, JsonRpcClient, Url},
    signers::{LocalWallet, SigningKey},
};
use starknet_crypto::Felt;

pub struct CallContractsPlugin;
impl Plugin for CallContractsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            setup_starknet_contract_caller.after(setup_torii_client),
        );
        app.add_systems(
            Update,
            send_simple_transaction.run_if(input_just_pressed(KeyCode::Enter)),
        );
        // app.add_systems(Update, update_entities);
    }
}

#[derive(Resource)]
struct StarknetAccount(SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet>);

fn setup_starknet_contract_caller(mut commands: Commands) {
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
    commands.insert_resource(StarknetAccount(account));
}

fn send_simple_transaction(account_res: ResMut<StarknetAccount>, tokio: Res<TokioRuntime>) {
    let actions_contract_address =
        Felt::from_hex("0x34a3bf116ba899adcc24e885548dcd981aa96c0aeac9ccf551429fd0c6f91cf")
            .unwrap();

    let selector = get_selector_from_name("create_game").unwrap();

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
