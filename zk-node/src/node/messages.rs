use serde_derive::{Serialize, Deserialize};

#[derive(FromForm)]
pub struct Message {
    pub content: String,
}

#[derive(FromForm, Default, Clone, Serialize, Deserialize)]
pub struct ProofMessage {
    pub proof_serialized: String,
    pub signatures_serialized: Option<String>
}