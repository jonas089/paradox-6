use serde_derive::{Serialize, Deserialize};
// todo: replace with bls!! -> bn254 not secure

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Groth16Proof{
    pub a: Vec<u8>,
    pub b: Vec<u8>,
    pub c: Vec<u8>,
}

/*
impl Groth16Proof{
    pub fn build(&self) -> ark_groth16::Proof<Bls12<Config>>{
        ark_groth16::Proof{
        a: G1Affine::deserialize_uncompressed(&*self.a).unwrap(),
        b: G2Affine::deserialize_uncompressed(&*self.b).unwrap(),
        c: G1Affine::deserialize_uncompressed(&*self.c).unwrap()
        }
    }
}
*/

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Groth16VerifyingKey{
    pub alpha_g1: Vec<u8>,
    pub beta_g2: Vec<u8>,
    pub delta_g2: Vec<u8>,
    pub gamma_g2: Vec<u8>,
    pub gamma_abc_g1: Vec<Vec<u8>>
}

/*
impl Groth16VerifyingKey{
    pub fn build(&self) -> ark_groth16::VerifyingKey<Bls12<Config>>{
        let alpha_g1: sw::Affine<ark_bls12_377::g1::Config> = G1Affine::deserialize_uncompressed(&*self.alpha_g1).unwrap();
        let beta_g2: sw::Affine<ark_bls12_377::g2::Config> = G2Affine::deserialize_uncompressed(&*self.beta_g2).unwrap();
        let gamma_g2: sw::Affine<ark_bls12_377::g2::Config> = G2Affine::deserialize_uncompressed(&*self.gamma_g2).unwrap();
        let delta_g2: sw::Affine<ark_bls12_377::g2::Config> = G2Affine::deserialize_uncompressed(&*self.delta_g2).unwrap();

        let mut gamma_abc_g1: Vec<sw::Affine<ark_bls12_377::g1::Config>> = Vec::new();
        for gamma_abc in self.gamma_abc_g1.clone(){
            gamma_abc_g1.push(G1Affine::deserialize_uncompressed(&*gamma_abc).unwrap());
        };
        ark_groth16::VerifyingKey { 
            alpha_g1,
            beta_g2,
            gamma_g2, 
            delta_g2,
            gamma_abc_g1
        }
    }
}
*/