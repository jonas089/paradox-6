use core::panic;
use std::process::Command;
use std::fs::{File, create_dir, self};
use std::io::Write;
use std::path::PathBuf;
use tempfile::tempdir;
use crate::types::NoirProof;
use crate::noir::constants as noir_constants;

pub struct NoirVerifier {
    // path to circuits (/nix/.../circuits/noir/)
    pub env_path_to_circuit: PathBuf
}

impl NoirVerifier{
    // verify a noir proof against a global circuit
    pub fn verify_cmd(&self, temp_data: NoirProof) -> bool{
        assert!(&self.env_path_to_circuit.is_dir());
        let temp_dir = tempdir().unwrap();
        let temp_dir = temp_dir.path().to_path_buf();
        self.copy_constants(temp_data.circuit.clone(), temp_dir.clone());
        // create subdirectory "proofs" in temp
        let temp_proofs = temp_dir.join(noir_constants::PROOFS);
        create_dir(&temp_proofs).expect("Failed to create temp/proofs!");
        let mut proof_file = match File::create(temp_proofs.join(noir_constants::PROOF_FILE_NAME)) {
            Err(msg) => panic!("{:?}", msg),
            Ok(file) => file,
        };
        proof_file.write_all(temp_data.proof.as_bytes()).expect("Failed to write proof!");
        let mut verifier_file = match File::create(temp_dir.join(noir_constants::VERIFIER_TOML)) {
            Err(msg) => panic!("{:?}", msg),
            Ok(file) => file,
        };
        verifier_file.write_all(temp_data.verifier.as_bytes()).expect("Failed to write verifier!");
        /*assert!(temp_dir.join(noir_constants::NARGO_TOML).is_file());
        assert!(temp_dir.join("src").join("main.nr").is_file());
        assert!(temp_dir.join("proofs").join("test.proof").is_file());
        assert!(temp_dir.join("Verifier.toml").is_file());*/
        // call nargo binary (nix or local in path)
        let command = Command::new(noir_constants::NARGO)
            .arg(noir_constants::VERIFY_COMMAND)
            .arg("--workspace")
            .current_dir(temp_dir.to_str().unwrap())
            .output()
            .unwrap();
        let mut is_valid = true;
        if command.status.success() {
            let _ = String::from_utf8_lossy(&command.stdout);
        } else {
            //let error = String::from_utf8_lossy(&command.stderr);
            is_valid = false;
        }
        is_valid
    }

    // copy constants that are required for every proof and circuit
    pub fn copy_constants(&self, circuit: String, temp_dir: PathBuf){
        let temp_src = temp_dir.join("src");
        create_dir(&temp_src).unwrap();
        // copy all files from circuit src to temp
        for script in fs::read_dir(&self.env_path_to_circuit.join(&circuit).join(noir_constants::SRC)).unwrap(){
            let script_unwrapped = script.unwrap();
            let script_path = &script_unwrapped.path();
            let destination_path = &temp_src.join(&script_unwrapped.file_name());
            if let Err(msg) = fs::copy(script_path, destination_path){
                panic!("Failed to copy script! \n Code: {:?}", msg)
            };
        };
        // copy default Nargo.toml to temp
        let temp_nargo_toml = temp_dir.join(noir_constants::NARGO_TOML);
        if let Err(msg) = fs::copy(self.env_path_to_circuit.join(circuit).join(noir_constants::NARGO_TOML), temp_nargo_toml){
            panic!("Failed to copy Nargo.toml! \n Code: {:?}", msg)
        };
    }
}

#[test]
fn verify(){
    use std::env;
    use crate::constants::*;
    use crate::noir::constants as noir_constants;
    // nix develop, cargo test
    let mut temp_data = NoirProof{
        hash: String::from(noir_constants::DEFAULT_HASH),
        verifier: String::from(noir_constants::DEFAULT_VERIFIER),
        // todo: replace w. up to date proof
        proof: String::from(noir_constants::DEFAULT_PROOF),
        circuit: String::from(noir_constants::DEFAULT_CIRCUIT)
    };
    let env_path_to_circuit = env::var(CIRCUITS_ENV_PATH).expect(MISSING_CIRCUITS_PATH);
    let noir_verifier = NoirVerifier{
        env_path_to_circuit: PathBuf::from(env_path_to_circuit).join(NOIR)
    };
    
    let result: bool = noir_verifier.verify_cmd(temp_data.clone());
    println!("Proof result: {:?}", &result);
    assert_eq!(result, true);

    temp_data.proof = String::from(r#"y="0x0000000000000000000000000000000000000000000000000000000000000003""#);
    let result: bool = noir_verifier.verify_cmd(temp_data);
    assert_eq!(result, false);
}