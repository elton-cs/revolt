use bevy::math::Vec3;

// Starknet.rs contstants for sending transactions
pub const STARKNET_RS_JSONRPC_URL: &str = "http://localhost:5050/";
pub const LOCAL_WALLET_PRIVATE_KEY: &str =
    "0x2bbf4f9fd0bbb2e60b0316c1fe0b76cf7a4d0198bd493ced9b8df2a3a24d68a";
pub const PLAYER_CONTRACT_ADDRESS: &str =
    "0xb3ff441a68610b30fd5e2abbf3a1548eb6ba6f3559f2862bf2dc757e5828ca";

pub const GAME_SYSTEM_CONTRACT_ADDRESS: &str =
    "0x34a3bf116ba899adcc24e885548dcd981aa96c0aeac9ccf551429fd0c6f91cf";
pub const GAME_SYSTEM_SELECTORS: [&str; 2] = ["create_game", "join_game"];

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

// Contract "constants"
pub const MAP_SIZE: (u8, u8) = (35, 21);

// Map rendering constants
pub const GROUND_Z_HEIGHT: f32 = 0.0;
pub const WALL_Z_HEIGHT: f32 = 1.0;
pub const PLAYER_Z_HEIGHT: f32 = 2.0;

pub const GROUND_TEXTURE_INDEX: usize = 0;
pub const WALL_TEXTURE_INDEX: usize = 1;

// UI constants
pub const BUTTONS_Z_HEIGHT: f32 = 3.0;
pub const BUTTONS_SCALE: Vec3 = Vec3::splat(0.2);

pub const CREATE_BUTTON_LOCATION: Vec3 = Vec3::new(40.0, -40.0, BUTTONS_Z_HEIGHT);
pub const JOIN_BUTTON_LOCATION: Vec3 = Vec3::new(200.0, -40.0, BUTTONS_Z_HEIGHT);
