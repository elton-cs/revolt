use crate::utils::constants::{CHAIN_ID_IN_HEX, STARKNET_RS_JSONRPC_URL};
use bevy::prelude::*;
use starknet::{
    accounts::{ExecutionEncoding, SingleOwnerAccount},
    core::{chain_id, utils::cairo_short_string_to_felt},
    providers::{jsonrpc::HttpTransport, JsonRpcClient, Url},
    signers::{LocalWallet, SigningKey},
};
use starknet_crypto::Felt;
pub struct StarknetRsPlugin;
impl Plugin for StarknetRsPlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Resource)]
pub struct PlayerAccount {
    pub pk: String,
    pub address: String,
}

pub fn build_account(
    player_private_key: &str,
    player_address: &str,
) -> SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet> {
    let provider = JsonRpcClient::new(HttpTransport::new(
        Url::parse(STARKNET_RS_JSONRPC_URL).unwrap(),
    ));
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        Felt::from_hex(player_private_key).unwrap(),
    ));
    let address = Felt::from_hex(player_address).unwrap();

    // let chain_id = cairo_short_string_to_felt("KATANA").unwrap(),
    let chain_id = Felt::from_hex_unchecked(CHAIN_ID_IN_HEX);

    SingleOwnerAccount::new(provider, signer, address, chain_id, ExecutionEncoding::New)
}
