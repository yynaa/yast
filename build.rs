use std::process::Command;

fn main() {
  let version = String::from_utf8(
    Command::new("date")
      .args(["+%y.%m%d-%H%M"])
      .output()
      .unwrap()
      .stdout,
  )
  .unwrap();

  println!("cargo:rustc-env=PROTOTYPE_VERSION={}", version);
}
