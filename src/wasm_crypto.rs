use crypto_box::{rand_core::OsRng, SecretKey};
use std::sync::Mutex;
use turbocharger::prelude::*;

struct client_sk([u8; 32]);

impl client_sk {
 fn load() -> Self {
  let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
  let mut keydata =
   hex::decode(&local_storage.get("keydata").unwrap_or_default().unwrap_or_default())
    .unwrap_or_default();
  if keydata.len() != 32 {
   let new_keydata = *SecretKey::generate(&mut OsRng).as_bytes();
   local_storage.set("keydata", &hex::encode(new_keydata)).unwrap();
   keydata = new_keydata.into()
  }
  Self(keydata.try_into().unwrap())
 }
 fn set_sk(&mut self, sk: String) {
  let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
  let keydata = hex::decode(&sk).unwrap_or_default();
  if keydata.len() == 32 {
   local_storage.set("keydata", &sk).unwrap();
   self.0 = keydata.try_into().unwrap();
  }
 }
 fn pk(&self) -> [u8; 32] {
  *SecretKey::from(self.0).public_key().as_bytes()
 }
 // fn encrypt(&self, data: Vec<u8>) -> Vec<u8> {
 //  sealed_box::seal(&data, &SecretKey::from(self.0).public_key())
 // }
 fn decrypt(&self, data: &[u8]) -> Option<Vec<u8>> {
  crypto_box::seal_open(&SecretKey::from(self.0), data).ok()
 }
}

static CLIENT_SK: Lazy<Mutex<client_sk>> = Lazy::new(|| Mutex::new(client_sk::load()));

#[wasm_bindgen]
pub async fn wasm_notify_client_pk() -> Result<(), JsValue> {
 let pk = CLIENT_SK.lock().unwrap().pk();
 crate::app::notify_client_pk(pk.to_vec()).await.map_err(|e| e.to_string().into())
}

#[wasm_bindgen]
pub fn wasm_client_sk() -> String {
 hex::encode(CLIENT_SK.lock().unwrap().0)
}

#[wasm_bindgen]
pub fn wasm_set_client_sk(sk: String) {
 CLIENT_SK.lock().unwrap().set_sk(sk);
}

#[tracked]
pub async fn wasm_decrypt_u8<T: AsRef<[u8]>>(data: T) -> Result<Vec<u8>, tracked::StringError> {
 Ok(CLIENT_SK.lock().unwrap().decrypt(data.as_ref())?)
}

#[tracked]
pub fn wasm_decrypt<T: AsRef<[u8]>>(data: T) -> Result<String, tracked::StringError> {
 Ok(std::str::from_utf8(&CLIENT_SK.lock().unwrap().decrypt(data.as_ref())?)?.to_string())
}

#[wasm_bindgen]
pub fn wasm_test_crypto_box() -> String {
 use crypto_box::{aead::Aead, aead::AeadCore, PublicKey, SalsaBox, SecretKey};

 let mut rng = crypto_box::rand_core::OsRng;
 let alice_secret_key = crypto_box::SecretKey::generate(&mut rng);

 // Get the public key for the secret key we just generated
 let alice_public_key_bytes = *alice_secret_key.public_key().as_bytes();

 // Obtain your recipient's public key.
 let bob_public_key = PublicKey::from([
  0xe8, 0x98, 0xc, 0x86, 0xe0, 0x32, 0xf1, 0xeb, 0x29, 0x75, 0x5, 0x2e, 0x8d, 0x65, 0xbd, 0xdd,
  0x15, 0xc3, 0xb5, 0x96, 0x41, 0x17, 0x4e, 0xc9, 0x67, 0x8a, 0x53, 0x78, 0x9d, 0x92, 0xc7, 0x54,
 ]);

 // Create a `Box` by performing Diffie-Hellman key agreement between
 // the two keys.
 let alice_box = SalsaBox::new(&bob_public_key, &alice_secret_key);

 // Get a random nonce to encrypt the message under
 let nonce = SalsaBox::generate_nonce(&mut rng);

 // Message to encrypt
 let plaintext = b"Top secret message we're encrypting";

 // Encrypt the message using the box
 let ciphertext = alice_box.encrypt(&nonce, &plaintext[..]).unwrap();

 //
 // Decryption
 //

 // Either side can encrypt or decrypt messages under the Diffie-Hellman key
 // they agree upon. The example below shows Bob's side.
 let bob_secret_key = SecretKey::from([
  0xb5, 0x81, 0xfb, 0x5a, 0xe1, 0x82, 0xa1, 0x6f, 0x60, 0x3f, 0x39, 0x27, 0xd, 0x4e, 0x3b, 0x95,
  0xbc, 0x0, 0x83, 0x10, 0xb7, 0x27, 0xa1, 0x1d, 0xd4, 0xe7, 0x84, 0xa0, 0x4, 0x4d, 0x46, 0x1b,
 ]);

 // Deserialize Alice's public key from bytes
 let alice_public_key = PublicKey::from(alice_public_key_bytes);

 // Bob can compute the same Box as Alice by performing the reciprocal
 // key exchange operation.
 let bob_box = SalsaBox::new(&alice_public_key, &bob_secret_key);

 // Decrypt the message, using the same randomly generated nonce
 let decrypted_plaintext = bob_box.decrypt(&nonce, &ciphertext[..]).unwrap();

 assert_eq!(&plaintext[..], &decrypted_plaintext[..]);
 // let mut rng = crypto_box::rand_core::OsRng;
 // let crypto_box_secret_key = crypto_box::SecretKey::generate(&mut rng);
 std::str::from_utf8(&decrypted_plaintext).unwrap().to_string()
}

#[wasm_bindgen(start)]
pub fn main() {
 console_error_panic_hook::set_once();
 tracing_wasm::set_as_global_default();

 let dev_string = format!("DEV-{}", include_str!(concat!(env!("OUT_DIR"), "/BUILD_TIME.txt")));
 let build_id = option_env!("BUILD_ID").unwrap_or(&dev_string);

 tracked::set_build_id(build_id);
}
