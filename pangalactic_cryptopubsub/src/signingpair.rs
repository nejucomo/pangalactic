use rust_sodium::crypto::sign;

#[derive(Clone, Debug)]
pub struct SigningPair {
    pub(crate) public: sign::PublicKey,
    secret: sign::SecretKey,
}

impl SigningPair {
    pub(crate) fn generate() -> SigningPair {
        let (public, secret) = sign::gen_keypair();
        SigningPair { public, secret }
    }
}
