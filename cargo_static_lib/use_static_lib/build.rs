fn main() {
  println!("cargo:rustc-link-search=../static_lib/lib");
  println!("cargo:rustc-link-lib=func");
}