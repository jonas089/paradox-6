use reqwest::blocking::Client;
extern crate serde_json;
extern crate zk_node;
mod constants;
#[allow(unused)]
use constants::*;
#[allow(unused)]
use zk_node::constants::*;
pub fn main(){
    panic!("Run cargo test instead.");
}

struct MockClient {
    url: String,
    client: Client,
}

trait ProverCli {
    fn submit(
        &self,
        proof_serialized: String
    ) -> Result<reqwest::blocking::Response, reqwest::Error>;
}

impl ProverCli for MockClient {
    fn submit(
        &self,
        proof_serialized: String
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        let data = [
            ("proof_serialized", &proof_serialized),
        ];
        let response = self.client.post(&self.url).form(&data).send().unwrap();

        //if response.status().is_success(){
        Ok(response)
    }
}

#[test]
fn noir(){
    use zk_node::types::Proof as ZkProof;
    use zk_node::types::NoirProof;
    let cli = MockClient{
        url: String::from(DEFAULT_PEER),
        client: Client::new()
    };

    // valid proof parameters
    let verifier_1: String = String::from(NOIR_VERIFIER);
    let proof_str: String = String::from(VALID_NOIR_PROOF);
    // invalid proof parameters
    let verifier_2: String = String::from(INVALID_NOIR_VERIFIER);

    // Submit valid proof
    let noir: NoirProof = NoirProof{
        hash: String::from("A0x00"),
        verifier: verifier_1,
        proof: proof_str.clone(),
        circuit: String::from("test")
    };
    let proof: ZkProof = ZkProof::Noir(noir);
    let proof_serialized = serde_json::to_string(&proof).unwrap();
    // submit proof to node
    let response = ProverCli::submit(&cli, proof_serialized).unwrap();
    assert!(response.status().is_success());
    println!("Server Response: {:?}", response.text().unwrap());

    // Submit invalid proof
    let noir_2: NoirProof = NoirProof{
        hash: String::from("R0x01"),
        verifier: verifier_2,
        proof: proof_str,
        circuit: String::from("test")
    };
    let proof_2: ZkProof = ZkProof::Noir(noir_2);
    let proof_2_serialized = serde_json::to_string(&proof_2).unwrap();
    let response_2 = ProverCli::submit(&cli, proof_2_serialized).unwrap();
    assert!(response_2.status().is_success());
    println!("Server Response: {:?}", response_2.text().unwrap());
}

#[test]
fn circom(){
    use zk_node::types::Proof as ZkProof;
    use ark_serialize::CanonicalSerialize;
    use zk_node::types::CircomProof;
    use ark_circom::{CircomConfig, CircomBuilder, CircomCircuit};
    use ark_bls12_377::{Bls12_377, Config};
    use std::path::PathBuf;
    use ark_groth16::{Groth16, ProvingKey};
    use ark_crypto_primitives::snark::SNARK;
    use ark_ec::bls12::Bls12;
    use std::env;

    type GrothBls = Groth16<Bls12_377>;

    let cli = MockClient{
        url: String::from(DEFAULT_PEER),
        client: Client::new()
    };

    // Load the WASM and R1CS for witness and proof generation
    let env_path_to_circuit: PathBuf = PathBuf::from(env::var(CIRCUITS_ENV_PATH).expect(MISSING_CIRCUITS_PATH));
    // Load the WASM and R1CS for witness and proof generation
    let cfg: CircomConfig<Bls12<Config>> = CircomConfig::<Bls12_377>::new(
        env_path_to_circuit.join(CIRCOM).join("multiplier").join("multiplier.wasm"),
        env_path_to_circuit.join(CIRCOM).join("multiplier").join("multiplier.r1cs")
    ).unwrap();

    let mut builder: CircomBuilder<Bls12<Config>> = CircomBuilder::new(cfg);
    let raw_inputs: Vec<(String, i32)> = vec![("a".to_string(), 2), ("b".to_string(), 20), ("c".to_string(), 40)];
    for raw_input in &raw_inputs{
        builder.push_input(&raw_input.0, raw_input.1);
    }
    
    let circom: CircomCircuit<Bls12<Config>> = builder.setup();
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
    let params: ProvingKey<Bls12<Config>> = GrothBls::generate_random_parameters_with_reduction(circom, &mut rng).unwrap();
    let circom: CircomCircuit<Bls12<Config>> = builder.build().unwrap();
    let public_inputs = circom.get_public_inputs().unwrap();
    let proof: ark_groth16::Proof<Bls12<Config>> = GrothBls::prove(&params, circom, &mut rng).unwrap();
    let pvk: ark_groth16::PreparedVerifyingKey<Bls12<Config>> = GrothBls::process_vk(&params.vk).unwrap();

    let mut pvk_buffer: Vec<u8> = Vec::new();
    let _ = pvk.serialize_uncompressed(&mut pvk_buffer);
    let mut proof_buffer: Vec<u8> = Vec::new();
    let _ = proof.serialize_uncompressed(&mut proof_buffer);
    
    // serialize the proof
    let mut serialized_proof: Vec<u8> = Vec::new();
    let _ = proof.serialize_uncompressed(&mut serialized_proof);
    // serialize the vk
    let mut serialized_vk: Vec<u8> = Vec::new();
    let _ = pvk.serialize_uncompressed(&mut serialized_vk);
    // serialize the inputs
    let mut serialized_inputs = Vec::new();
    for input in public_inputs{
        let mut buffer = Vec::new();
        input.serialize_uncompressed(&mut buffer).unwrap();
        serialized_inputs.push(buffer);
    }
    let proof_data_serialized = ZkProof::Circom(CircomProof{
        hash: String::from("A0x02"),
        circuit: "multiplier".to_string(),
        vk: pvk_buffer,
        proof: proof_buffer,
        inputs: serialized_inputs
    });
    let proof_message_serialized = serde_json::to_string(&proof_data_serialized).unwrap();
    let response: reqwest::blocking::Response = ProverCli::submit(&cli, proof_message_serialized).unwrap();
    assert!(response.status().is_success());
    println!("Server Response: {:?}", response.text().unwrap());
}