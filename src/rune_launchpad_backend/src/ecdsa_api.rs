use ic_cdk::{api::{ management_canister::ecdsa::{EcdsaCurve, EcdsaKeyId, EcdsaPublicKeyArgument, SignWithEcdsaArgument}}, call};

// The fee for the `sign_with_ecdsa` endpoint using the test key.
const SIGN_WITH_ECDSA_COST_CYCLES: u64 = 10_000_000_000;

/// Returns the ECDSA public key of this canister at the given derivation path.
pub async fn ecdsa_public_key(key_name: String, derivation_path: Vec<Vec<u8>>) -> Vec<u8> {
    // Retrieve the public key of this canister at the given derivation path
    // from the ECDSA API.

    let res = ic_cdk::api::management_canister::ecdsa::ecdsa_public_key(EcdsaPublicKeyArgument {
        canister_id: None,
        derivation_path,
        key_id:  EcdsaKeyId {
            curve: EcdsaCurve::Secp256k1,
            name: key_name,
        },
    }).await;
   
    res.unwrap().0.public_key
}

pub async fn sign_with_ecdsa(
    key_name: String,
    derivation_path: Vec<Vec<u8>>,
    message_hash: Vec<u8>,
) -> Vec<u8> {
    let res = ic_cdk::api::management_canister::ecdsa::sign_with_ecdsa(SignWithEcdsaArgument{
        message_hash,
        derivation_path,
        key_id: EcdsaKeyId {
            curve: EcdsaCurve::Secp256k1,
            name: key_name,
        },
    }).await;

    res.unwrap().0.signature
}