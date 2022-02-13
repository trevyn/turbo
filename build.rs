fn main() {
 if option_env!("BUILD_ID").is_none() {
  println!(
   "cargo:rustc-env=BUILD_TIME={}",
   time::OffsetDateTime::from(std::time::SystemTime::now())
    .format(&time::format_description::well_known::Rfc3339)
    .unwrap()
  );
 }
}
