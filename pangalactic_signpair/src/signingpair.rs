use sodiumoxide::crypto::sign;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct SigningPair {
    pub signer: Signer,
    pub verifier: Verifier,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Signer(sign::SecretKey);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Verifier(sign::PublicKey);

impl SigningPair {
    pub fn generate() -> SigningPair {
        pangalactic_sodiuminit::init_if_necessary();
        let (public, secret) = sign::gen_keypair();
        let signer = Signer(secret);
        let verifier = Verifier(public);
        SigningPair { signer, verifier }
    }
}

impl Signer {
    pub fn sign<T>(&self, msg: T) -> Vec<u8>
    where
        T: AsRef<[u8]>,
    {
        sign::sign(msg.as_ref(), &self.0)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct InvalidSignature;

impl Verifier {
    pub fn verify<T>(&self, signedmsg: T) -> Result<Vec<u8>, InvalidSignature>
    where
        T: AsRef<[u8]>,
    {
        sign::verify(signedmsg.as_ref(), &self.0).map_err(|()| InvalidSignature)
    }
}
