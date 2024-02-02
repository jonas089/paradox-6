use serde_derive::{Serialize, Deserialize};
use crate::constants::*;

#[derive(Serialize, Deserialize)]
pub struct NodeResponse{
    pub errors: Option<NodeError>,
    pub status_code: u32,
    pub hash_identifier: Option<String>,
    pub message: ResponseMessage
}
#[derive(Serialize, Deserialize)]
pub enum NodeError{
    DuplicateError,
    GossipError,
    StorageError
}

impl NodeError{
    pub fn get_status_code(self) -> u32{
        match self{
            Self::DuplicateError => 0u32,
            Self::GossipError => 1u32,
            Self::StorageError => 2u32
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum ResponseMessage{
    ProofReceived,
    ProofAccepted,
    ProofRejected
}

impl ResponseMessage{
    pub fn explain(&self){
        // explain a response message
        todo!("Add explaination!");
    }
}

#[derive(Serialize, Deserialize)]
pub struct EventMessage{
    pub msg: String,
    pub peer: Option<String>,
    pub hash: Option<String>,
    pub detail: Option<String>
}
impl EventMessage{
    pub fn text(self) -> String{
        serde_json::to_string(&self).unwrap()
    }
}

#[derive(Serialize, Deserialize)]
pub struct Logger{
    pub message: DebugMessage
}
impl Logger{
    pub fn print(self, peer: Option<&String>, hash: Option<&String>, detail: Option<&String>){
        println!("[INFO] {:?}", self.message.text(peer, hash, detail));
    }
    pub fn eprint(self, peer: Option<&String>, hash:Option<&String>, detail: Option<&String>){
        eprintln!("[ERROR] {:?}", self.message.text(peer, hash, detail));
    }
}

#[derive(Serialize, Deserialize)]
pub enum DebugMessage{
    ProofAccepted,
    ProofRejected,
    FailedToReadResponseBody,
    FailedToGossip,
    ProofStored,

    PeerResponse
}

impl DebugMessage{
    pub fn text(self, peer: Option<&String>, hash: Option<&String>, detail: Option<&String>) -> String{
        match self{
            Self::ProofAccepted => EventMessage{
                msg: PROOF_ACCEPTED_RESPONSE.to_string(),
                peer: peer.cloned(),
                hash: hash.cloned(),
                detail: detail.cloned(),
            }.text(),
            Self::ProofRejected => EventMessage{
                msg: PROOF_REJECTED_RESPONSE.to_string(),
                peer: peer.cloned(),
                hash: hash.cloned(),
                detail: detail.cloned(),
            }.text(),
            Self::FailedToReadResponseBody => EventMessage{
                msg: FAILED_TO_READ_RESPONSE_BODY.to_string(),
                peer: peer.cloned(),
                hash: hash.cloned(),
                detail: detail.cloned(),
            }.text(),
            Self::FailedToGossip => EventMessage{
                msg: FAILED_TO_GOSSIP.to_string(),
                peer: peer.cloned(),
                hash: hash.cloned(),
                detail: detail.cloned(),
            }.text(),
            Self::ProofStored => EventMessage{
                msg: PROOF_STORED_RESPONSE.to_string(),
                peer: peer.cloned(),
                hash: hash.cloned(),
                detail: detail.cloned()
            }.text(),
            Self::PeerResponse => EventMessage{
                msg: RECEIVED_RESPONSE.to_string(),
                peer: peer.cloned(),
                hash: hash.cloned(),
                detail: detail.cloned()
            }.text()
        }
    }
}