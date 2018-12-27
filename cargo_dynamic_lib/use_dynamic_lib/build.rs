fn main() {
  println!("cargo:rustc-env=PATH=%PATH%;../bin");
  println!("cargo:rustc-link-search=../dynamic_lib/lib");
  println!("cargo:rustc-link-lib=func");
}