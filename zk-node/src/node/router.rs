use rocket::http::Status;
use rocket::form::Form;
use rocket::response::status::Custom;
use rocket::{get, post};
use std::path::PathBuf;
use crate::node::messages::ProofMessage;
use crate::types::Proof;
use serde_json::to_string;
use crate::storage::storer::{Storage, UniversalStorage};
use crate::node::receiver::{Thread, NoirThread, CircomThread};
use crate::node::response::{NodeResponse, NodeError, ResponseMessage};
use crate::constants::{CIRCOM_DB_NAME, NOIR_DB_NAME, PING_SERVER_RESPONSE, RocketCfg};

#[allow(unreachable_patterns)]
#[post("/proof", data = "<message_form>")]
pub async fn proof_receiver(cfg: &rocket::State<RocketCfg>, message_form: Form<ProofMessage>) -> Result<Custom<String>, Status>{
    let proof_serialized: String = message_form.proof_serialized.clone();
    let proof: Proof = serde_json::from_str(&proof_serialized).unwrap();
    let path_to_db: PathBuf = cfg.path_to_db.clone();
    let peers: Vec<(String, u16)> = cfg.peers.clone();

    // process proofs for different backends
    match &proof {
        // Handle a noir proof for any circuit
        Proof::Noir(proof_unwrapped) => {
            let storer: Storage = Storage {
                path: path_to_db.join(NOIR_DB_NAME),
            };
            let maybe_entry: Option<String> = storer.fetch_by_unique_id(&proof_unwrapped.hash).unwrap();
            // early revert if proof already exists
            if maybe_entry.is_some(){
                return Ok(Custom(
                    Status::Ok,
                    serde_json::to_string(&NodeResponse{
                        errors: Some(NodeError::DuplicateError),
                        status_code: NodeError::DuplicateError.get_status_code(),
                        hash_identifier: Some(proof_unwrapped.clone().hash),
                        message: ResponseMessage::ProofRejected
                    }).unwrap()));
            }
            else{
                // construct the thread to verify and gossip the message
                let thread = Thread{
                    path_to_db,
                    peers,
                    proof: proof.clone(),
                    proof_serialized
                };
                // spawn the thread
                NoirThread::spawn(&thread, storer).await;
            }
        },

        Proof::Circom(proof_unwrapped) => {
            let storer: Storage = Storage { 
                path:  path_to_db.join(CIRCOM_DB_NAME)
            };
            let maybe_entry: Option<String> = storer.fetch_by_unique_id(&proof_unwrapped.hash).unwrap();
            // early revert if proof already exists
            if maybe_entry.is_some(){
                return Ok(Custom(
                    Status::Ok,
                    serde_json::to_string(&NodeResponse{
                        errors: Some(NodeError::DuplicateError),
                        status_code: NodeError::DuplicateError.get_status_code(),
                        hash_identifier: Some(proof_unwrapped.clone().hash),
                        message: ResponseMessage::ProofRejected
                    }).unwrap()));
            }
            else{
                // construct the thread to verify and gossip the message
                let thread = Thread{
                    path_to_db,
                    peers,
                    proof: proof.clone(),
                    proof_serialized
                };
                // spawn the thread
                CircomThread::spawn(&thread, storer).await;
            }
        }
        // tbd: handle proofs for other backends
        _ => todo!("Support more backends!")
    };
    Ok(Custom(
        Status::Ok,
        serde_json::to_string(&NodeResponse{
            errors: None,
            status_code: 200,
            hash_identifier: 
            Some(
                match &proof {
                    Proof::Noir(noir) => noir.clone().hash,
                    Proof::Circom(circom) => circom.clone().hash
                }
            ),
            message: ResponseMessage::ProofAccepted
        }).unwrap()))
}

#[get("/qnoir?<id>")]
pub fn noir_query(cfg: &rocket::State<RocketCfg>, id: Option<String>) -> String {
    let noir_storage = Storage {
        path: cfg.path_to_db.join(NOIR_DB_NAME),
    };
    // a few unhandled unwraps that can cause exceptions -> improve error handling
    let noir_proof: String = noir_storage
        .fetch_by_unique_id(&id.unwrap())
        .unwrap()
        .unwrap();
    to_string(&noir_proof).unwrap()
}

#[get("/qcircom?<id>")]
pub fn circom_query(cfg: &rocket::State<RocketCfg>, id: Option<String>) -> String{
    let circom_storage = Storage{
        path: cfg.path_to_db.join(CIRCOM_DB_NAME),
    };
    let circom_proof: String = circom_storage
        .fetch_by_unique_id(&id.unwrap())
        .unwrap()
        .unwrap();
    to_string(&circom_proof).unwrap()
}

#[get("/ping")]
pub fn ping() -> String{
    PING_SERVER_RESPONSE.to_string()
}