use bevy::math::Vec3;

pub const STARKNET_RS_JSONRPC_URL: &str = "http://localhost:5050/";
pub const LOCAL_WALLET_PRIVATE_KEY: &str =
    "0x2bbf4f9fd0bbb2e60b0316c1fe0b76cf7a4d0198bd493ced9b8df2a3a24d68a";
pub const PLAYER_CONTRACT_ADDRESS: &str =
    "0xb3ff441a68610b30fd5e2abbf3a1548eb6ba6f3559f2862bf2dc757e5828ca";

// pub const STARKNET_RS_JSONRPC_URL: &str = "https://starknet-sepolia.public.blastapi.io/rpc/v0_7";
// pub const LOCAL_WALLET_PRIVATE_KEY: &str = "YOUR_PRIVATE_KEY_IN_HEX_HERE";
// pub const PLAYER_CONTRACT_ADDRESS: &str = "YOUR_ACCOUNT_CONTRACT_ADDRESS_IN_HEX_HERE";

pub const TILE_SIZE: f32 = 8.0;
pub const TILE_SCALE: Vec3 = Vec3::splat(0.5);
pub const PLAYER_SCALE: Vec3 = Vec3::splat(0.25);
pub const PLAYER_CAMERA_SCALE: f32 = 0.1;
