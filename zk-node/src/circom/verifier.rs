use crate::types::CircomProof;
use std::path::PathBuf;
use ark_serialize::CanonicalDeserialize;
use ark_groth16::{Groth16, Proof, PreparedVerifyingKey};
use ark_crypto_primitives::snark::SNARK;
use ark_ec::bls12::Bls12;
use ark_bls12_377::{Bls12_377, Config, Fr};

type GrothBls = Groth16<Bls12_377>;

pub struct CircomVerifier{
    pub circuit: String,
    pub env_path_to_circuit: PathBuf,
    pub inputs_serialized: Vec<Vec<u8>>
}

impl CircomVerifier{
    pub fn verify(&self, proof: CircomProof) -> bool{
        let mut deserialized_inputs = Vec::new();
        for input in proof.inputs{
            deserialized_inputs.push(Fr::deserialize_uncompressed(input.as_slice()).unwrap())
        };
        let deserialized_proof: Proof<Bls12<Config>> = Proof::deserialize_uncompressed(&mut proof.proof.as_slice()).unwrap();
        let deserialized_vk: PreparedVerifyingKey<Bls12<Config>> = PreparedVerifyingKey::deserialize_uncompressed(&mut proof.vk.as_slice()).unwrap();
        GrothBls::verify_with_processed_vk(&deserialized_vk, &deserialized_inputs, &deserialized_proof).unwrap()
    }
}