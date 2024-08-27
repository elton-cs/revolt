use bevy::math::Vec3;

// Starknet.rs contstants for sending transactions
pub const STARKNET_RS_JSONRPC_URL: &str = "http://localhost:5050/";
pub const LOCAL_WALLET_PRIVATE_KEY: &str =
    "0x2bbf4f9fd0bbb2e60b0316c1fe0b76cf7a4d0198bd493ced9b8df2a3a24d68a";
pub const PLAYER_CONTRACT_ADDRESS: &str =
    "0xb3ff441a68610b30fd5e2abbf3a1548eb6ba6f3559f2862bf2dc757e5828ca";

// Torii contstants for receiving entity updates
pub const TORII_URL: &str = "http://localhost:8080";
pub const TORII_RPC_URL: &str = "http://localhost:5050";
pub const TORII_RELAY_URL: &str = "/ip4/127.0.0.1/tcp/9090";
pub const TORII_WORLD_CONTRACT: &str =
    "0x5d97c46d046f442f125b6cc83057e97ee6e848c4921126acd8ae9d17b55b369";
pub const EXISTING_ENTITY_QUERY_LIMIT: u32 = 500;

// Bevy constants
pub const TILE_SIZE: f32 = 8.0;
pub const TILE_SCALE: Vec3 = Vec3::splat(0.5);
pub const PLAYER_SCALE: Vec3 = Vec3::splat(0.25);
pub const PLAYER_CAMERA_SCALE: f32 = 0.1;
