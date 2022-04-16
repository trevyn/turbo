fn main() {
 // Opt in to "new style" fingerprinting by setting a rerun-if-* value:
 // https://github.com/rust-lang/cargo/blob/46c9b51957e335ad03051ae8c513f3ba2a55b3b2/src/cargo/core/compiler/fingerprint.rs#L267-L276

 let build_time_file = format!("{}/BUILD_TIME.txt", std::env::var("OUT_DIR").unwrap());

 if option_env!("BUILD_ADJECTIVE").is_some() {
  println!("cargo:rerun-if-env-changed=BUILD_ADJECTIVE");
  std::fs::write(build_time_file, "none").unwrap();
 } else {
  std::fs::write(
   build_time_file,
   time::OffsetDateTime::from(std::time::SystemTime::now())
    .format(&time::format_description::well_known::Rfc3339)
    .unwrap(),
  )
  .unwrap();
 }
}
