// Spawn a receiving thread for any backend
use std::path::PathBuf;
use crate::storage::storer::{Storage, UniversalStorage};
use crate::noir::verifier::NoirVerifier;
use crate::circom::verifier::CircomVerifier;
use crate::types::{Proof, NoirProof, CircomProof};
use crate::sync::gossip::{Peer, Gossiper};
extern crate tokio;
use std::env;
use async_trait::async_trait;
use crate::constants::*;
// development only -> pre eventstream & logging
use crate::node::response::{Logger, DebugMessage};

#[async_trait]
pub trait NoirThread{
    async fn spawn(&self, storer: Storage);
}

#[async_trait]
pub trait CircomThread{
    async fn spawn(&self, storer: Storage);
}

pub struct Thread{
    pub path_to_db: PathBuf,
    pub peers: Vec<(String, u16)>,
    pub proof: Proof,
    pub proof_serialized: String,
}

impl Thread{
    pub fn gossip(&self){
        let mut _peers: Vec<Peer> = Vec::new();
        for _peer in &self.peers{
            let peer: Peer = Peer{
                host: _peer.0.to_string(),
                port: _peer.1
            };
            _peers.push(peer);
        };
        // gossip proof
        let gossiper: Gossiper = Gossiper{
            peers: _peers
        };
        // handle set of responses for peers
        let responses: Vec<(String, Result<reqwest::blocking::Response, reqwest::Error>)> = gossiper.gossip(self.proof_serialized.clone());
        for (_, response) in responses.into_iter().enumerate(){
            match response.1{
                Ok(_response) => {
                    if _response.status().is_success(){
                        let body = _response.text().unwrap();
                        Logger{
                            message: DebugMessage::PeerResponse
                        }.print(Some(&response.0), None, Some(&body));
                    }
                    else{
                        Logger{
                            message: DebugMessage::FailedToGossip
                        }.eprint(Some(&response.0), None, Some(&_response.text().unwrap()));
                    }
                },
                Err(error) => {
                    Logger{
                        message: DebugMessage::FailedToReadResponseBody
                    }.eprint(Some(&response.0), None, Some(&error.to_string()))
                }
            }
        }
    }
    
    pub async fn gossip_blocking(self){
        tokio::task::spawn_blocking(move || {
            self.gossip();
        }).await.unwrap();
    }

    pub async fn store_blocking(&self, storer: Storage, proof: Proof ){
        match proof {
            Proof::Noir(proof) => {
                tokio::task::spawn_blocking(move || {
                    UniversalStorage::insert(&storer, proof.clone().hash, serde_json::to_string(&proof).unwrap()).expect(FAILED_TO_STORE_PROOF);
                    Logger{
                        message: DebugMessage::ProofStored
                    }.print(None, Some(&proof.hash), None);
                    /*
                        todo: emit event
                    */
                }).await.unwrap();
            },
            Proof::Circom(proof) => {
                tokio::task::spawn_blocking(move || {
                    UniversalStorage::insert(&storer, proof.clone().hash, serde_json::to_string(&proof).unwrap()).expect(FAILED_TO_STORE_PROOF);
                    Logger{
                        message: DebugMessage::ProofStored
                    }.print(None, Some(&proof.hash), None);
                    /*
                        todo: emit event
                    */
                }).await.unwrap();
            }
        }
    }
}

#[async_trait]
impl NoirThread for Thread{
     async fn spawn(&self, storer: Storage){
        let path_to_db: PathBuf = self.path_to_db.clone();
        let peers: Vec<(String, u16)> = self.peers.clone();
        let proof: Proof = self.proof.clone();
        let proof_serialized: String = self.proof_serialized.clone();
        let proof_unwrapped: NoirProof = self.proof.clone().unwrap_noir().unwrap();
        let env_path_to_circuit: PathBuf = PathBuf::from(env::var(CIRCUITS_ENV_PATH).expect(MISSING_CIRCUITS_PATH));
        
        tokio::spawn(async move {
            let verifier: NoirVerifier = NoirVerifier {
                env_path_to_circuit: env_path_to_circuit.join(NOIR),
            };
            // this is an I/O task => await
            let proof_unwrapped_clone = proof_unwrapped.clone();
            let proof_valid: bool = tokio::task::spawn_blocking(move || {
                verifier.verify_cmd(proof_unwrapped_clone)
            }).await.unwrap();
            // handle valid proof
            if proof_valid {
                Logger{
                    message: DebugMessage::ProofAccepted
                }.print(None, Some(&proof_unwrapped.hash), None);
                let thread = Thread{
                    path_to_db,
                    peers,
                    proof: proof.clone(),
                    proof_serialized
                };
                thread.store_blocking(storer.clone(), proof.clone()).await;
                thread.gossip_blocking().await;
            // handle invalid proof
            } else {
                Logger{
                    message: DebugMessage::ProofRejected
                }.eprint(None, Some(&proof_unwrapped.hash), None);
                /*
                    todo: emit event
                */
            }
        });
        
    }
}

#[async_trait]
impl CircomThread for Thread{
    async fn spawn(&self, storer: Storage){
        let path_to_db: PathBuf = self.path_to_db.clone();
        let peers: Vec<(String, u16)> = self.peers.clone();
        let proof: Proof = self.proof.clone();
        let proof_unwrapped: CircomProof = self.proof.clone().unwrap_circom().unwrap();
        let proof_unwrapped_clone: CircomProof = proof_unwrapped.clone();
        let proof_serialized: String = self.proof_serialized.clone();
        let env_path_to_circuit: PathBuf = PathBuf::from(env::var(CIRCUITS_ENV_PATH).expect(MISSING_CIRCUITS_PATH));
        tokio::spawn(async move {
            let proof_valid: bool = tokio::task::spawn_blocking(move || {
                let verifier: CircomVerifier = CircomVerifier{
                    circuit: proof_unwrapped_clone.circuit.clone(),
                    env_path_to_circuit: env_path_to_circuit.join(CIRCOM),
                    inputs_serialized: proof_unwrapped_clone.inputs.clone()
                };
                verifier.verify(proof_unwrapped_clone)
            }).await.unwrap();
            // handle circom proof
            if proof_valid {
                Logger{
                    message: DebugMessage::ProofAccepted
                }.print(None, Some(&proof_unwrapped.hash), None);
                let thread = Thread{
                    path_to_db,
                    peers,
                    proof: proof.clone(),
                    proof_serialized
                };
                thread.store_blocking(storer.clone(), proof.clone()).await;
                thread.gossip_blocking().await;
            }
            else{
                Logger{
                    message: DebugMessage::ProofRejected
                }.eprint(None, Some(&proof_unwrapped.hash), None)
                /*
                    todo: emit event
                */
            }
        });
    }
}