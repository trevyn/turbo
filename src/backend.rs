#[turbocharger::backend]
mod backend {

 use turbosql::Turbosql;

 #[turbocharger::server_only]
 use {
  anyhow::Context,
  blst::min_sig::*,
  rand::{RngCore, SeedableRng},
  rand_chacha::ChaCha20Rng,
  turbosql::select,
 };

 #[derive(Turbosql)]
 pub struct Person {
  pub rowid: Option<i64>,
  pub name: Option<String>,
 }

 pub async fn insert_person(p: Person) -> Result<i64, turbosql::Error> {
  p.insert() // returns rowid
 }

 pub async fn get_person(rowid: i64) -> Result<Person, turbosql::Error> {
  select!(Person "WHERE rowid = ?", rowid)
 }

 pub async fn get_new_secret_key() -> Result<String, anyhow::Error> {
  let mut seed = [0u8; 32];
  rand::rngs::OsRng.fill_bytes(&mut seed);
  let mut rng = ChaCha20Rng::from_seed(seed);

  let mut ikm = [0u8; 32];
  rng.fill_bytes(&mut ikm);

  let sk = SecretKey::key_gen(&ikm, &[]).unwrap();
  let pk = sk.sk_to_pk();

  dbg!(hex::encode(sk.to_bytes()));
  dbg!(hex::encode(pk.compress()));

  let dst = b"BLS_SIG_BLS12381G2_XMD:SHA-256_SSWU_RO_NUL_";
  let msg = b"blst is such a blast";
  let sig = sk.sign(msg, dst, &[]);

  dbg!(hex::encode(sig.compress()));

  let err = sig.verify(true, msg, dst, &[], &pk, true);
  dbg!(err);
  assert_eq!(err, blst::BLST_ERROR::BLST_SUCCESS);

  Ok(hex::encode(sk.to_bytes()))
 }

 pub async fn getblockchaininfo() -> Result<String, anyhow::Error> {
  let cookie = std::fs::read_to_string("/root/.bitcoin/.cookie")?;
  let mut cookie_iter = cookie.split(":");
  let username = cookie_iter.next().context("no username")?;
  let password = cookie_iter.next().context("no password")?;

  let client = reqwest::Client::new();
  let res = client
   .post("http://127.0.0.1:8332")
   .basic_auth(username, Some(password))
   .body(r#"{"jsonrpc": "1.0", "id":"curltest", "method": "getblockchaininfo", "params": [] }"#)
   .send()
   .await?
   .text()
   .await?;
  Ok(res)
 }
}
