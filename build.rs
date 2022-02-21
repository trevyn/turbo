fn main() {
 // Opt in to "new style" fingerprinting by setting a rerun-if-* value:
 // https://github.com/rust-lang/cargo/blob/46c9b51957e335ad03051ae8c513f3ba2a55b3b2/src/cargo/core/compiler/fingerprint.rs#L267-L276
 println!("cargo:rerun-if-env-changed=BUILD_ADJECTIVE");
 if option_env!("BUILD_ADJECTIVE").is_none() {
  println!(
   "cargo:rustc-env=BUILD_TIME={}",
   time::OffsetDateTime::from(std::time::SystemTime::now())
    .format(&time::format_description::well_known::Rfc3339)
    .unwrap()
  );
 }
}
