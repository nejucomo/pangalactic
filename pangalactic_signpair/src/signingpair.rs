use rust_sodium::crypto::sign;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SigningPair {
    pub signer: Signer,
    pub verifier: Verifier,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Signer(sign::SecretKey);

#[derive(Clone, Debug, PartialEq, Eq)]
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
    pub fn sign(&self, msg: &[u8]) -> Vec<u8> {
        sign::sign(msg, &self.0)
    }
}

impl Verifier {
    pub fn verify(&self, signedmsg: &[u8]) -> Result<Vec<u8>, ()> {
        sign::verify(signedmsg, &self.0)
    }
}
