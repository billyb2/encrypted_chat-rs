use base64::{encode_config, decode_config};

use smaz::{compress, decompress};

use wasm_bindgen::prelude::*;

use rsa::{PublicKey, RsaPrivateKey, RsaPublicKey, PaddingScheme, pkcs8::{FromPublicKey, ToPublicKey}, pkcs1::{FromRsaPrivateKey, ToRsaPrivateKey} };
use rand::rngs::OsRng;

const BIT_SIZE: usize = 2048;
const PADDING: PaddingScheme = PaddingScheme::PKCS1v15Encrypt;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    Ok(())
}

#[wasm_bindgen]
//TODO: Do precompute stuff
pub fn gen_priv_key() -> String {
    let mut rng = OsRng;
    let priv_key = RsaPrivateKey::new(&mut rng, BIT_SIZE).expect("failed to generate a key");

    let priv_key_string = &*priv_key.to_pkcs1_pem().unwrap();
    (*priv_key_string).clone()
    
}

#[wasm_bindgen]
pub fn gen_pub_key(priv_key: String) -> String {

    let public_key = RsaPrivateKey::from_pkcs1_pem(&priv_key).unwrap().to_public_key();
    public_key.to_public_key_pem().unwrap()

}

#[wasm_bindgen]
pub fn enc(unencrypted_string: String, pub_key: String) -> String {
    let unencrypted_uncompressed_bytes = unencrypted_string.into_bytes();
    let unencrypted_bytes = compress(&unencrypted_uncompressed_bytes);

    let mut rng = OsRng;
    let pub_key = RsaPublicKey::from_public_key_pem(&pub_key).unwrap();

    let encrypted_bytes = pub_key.encrypt(&mut rng, PADDING, &unencrypted_bytes).expect("failed to encrypt");

    encode_config(encrypted_bytes, base64::URL_SAFE_NO_PAD)

}

#[wasm_bindgen]
pub fn dec(encrypted_string: String, priv_key: String) -> String {
    let encrypted_compressed_bytes = decode_config(encrypted_string, base64::URL_SAFE_NO_PAD).unwrap();

    let priv_key = RsaPrivateKey::from_pkcs1_pem(&priv_key).unwrap();
    let unencrypted_compressed_bytes = priv_key.decrypt(PADDING, &encrypted_compressed_bytes).unwrap();

    let string_bytes = decompress(&unencrypted_compressed_bytes).unwrap();

    String::from_utf8_lossy(&string_bytes).to_string()

}