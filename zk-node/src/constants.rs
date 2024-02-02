// SQL files
pub const NOIR_DB_NAME: &str = "noir.db";
pub const CIRCOM_DB_NAME: &str = "circom.db";
// Circuit directories
pub const NOIR: &str = "noir";
pub const CIRCOM: &str = "circom";
// Nix
pub const CIRCUITS_ENV_PATH: &str = "CIRCUITS_PATH";
// Response messages and errors
pub const PROOF_ACCEPTED_RESPONSE: &str = "Proof accepted!";
pub const PROOF_REJECTED_RESPONSE: &str = "Proof rejected!";
pub const PROOF_STORED_RESPONSE: &str = "Proof was stored!";
pub const FAILED_TO_READ_RESPONSE_BODY: &str = "Failed to read response body!";
pub const FAILED_TO_GOSSIP: &str = "Failed to gossip Proof!";
pub const RECEIVED_RESPONSE: &str = "Received response from Peer!";
// I/O error messages
pub const FAILED_TO_STORE_PROOF: &str = "Failed to store proof!";
pub const MISSING_CIRCUITS_PATH: &str = "Missing required environment variable: CIRCUITS_PATH!";
// Hello world message for testing
pub const PING_SERVER_RESPONSE: &str = "Hello, Prover!";

// rocket config
use std::path::PathBuf;
pub struct RocketCfg{
    pub path_to_db: PathBuf,
    pub peers: Vec<(String, u16)>
}