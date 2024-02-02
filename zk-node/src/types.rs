use serde_derive::{Serialize, Deserialize};
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct CircomProof{
    pub hash: String,
    pub circuit: String,
    pub vk: Vec<u8>,
    pub proof: Vec<u8>,
    pub inputs: Vec<Vec<u8>>
}


// Noir specific Types
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct NoirProof {
    pub hash: String,
    pub verifier: String,
    pub proof: String,
    pub circuit: String,
}

// Multi-lib Proof type
#[derive(Clone, Serialize, Deserialize)]
pub enum Proof{
    Noir (NoirProof),
    Circom (CircomProof)
}

impl Proof{
    pub fn unwrap_noir(self) -> Option<NoirProof>{
        match self {
            Proof::Noir(noir) => Some(noir),
            _ => None
        }
    }

    pub fn unwrap_circom(self) -> Option<CircomProof>{
        match self{
            Proof::Circom(circom) => Some(circom),
            _ => None
        }
    }
}

#[test]
fn type_tests(){
    let noir: NoirProof = NoirProof{
        hash: String::from("0x"),
        verifier: String::from("0x"),
        proof: String::from("0x"),
        circuit: String::from("0x")
    };
    let proof: Proof = Proof::Noir(noir);
    println!("Proof serialized: {:?}", serde_json::to_string(&proof).unwrap());
    let proof_deserialized: Proof = serde_json::from_str(&serde_json::to_string(&proof).unwrap()).unwrap();
    let _ = proof_deserialized.unwrap_noir().unwrap();
}