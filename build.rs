fn main() {
  println!("Running pre build script");

  // Set the SDL2 library directory
  println!("cargo:rustc-link-search=native=/opt/homebrew/lib");

  // Link against the SDL2 library
  println!("cargo:rustc-link-lib=SDL2");
}