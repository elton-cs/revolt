use crate::utils::constants::{
    LOCAL_WALLET_PRIVATE_KEY, PLAYER_CONTRACT_ADDRESS, STARKNET_RS_JSONRPC_URL,
};
use bevy::prelude::*;
use starknet::{
    accounts::{ExecutionEncoding, SingleOwnerAccount},
    core::utils::cairo_short_string_to_felt,
    providers::{jsonrpc::HttpTransport, JsonRpcClient, Url},
    signers::{LocalWallet, SigningKey},
};
use starknet_crypto::Felt;

pub struct StarknetRsPlugin;
impl Plugin for StarknetRsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_starknet_contract_caller);
    }
}

#[derive(Resource)]
pub struct BurnerWalletAccount(pub SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet>);

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
    commands.insert_resource(BurnerWalletAccount(account));
}
