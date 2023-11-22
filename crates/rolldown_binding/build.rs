fn main() {
  use napi_build::setup;
  println!("cargo:rustc-link-arg=--max-memory=4294967296");
  // lld only allocates 1MiB for the WebAssembly stack, and the array that you're allocating on the stack is exactly 1MiB.
  // 0x800000 bytes = 8MiB
  println!("cargo:rustc-link-arg=-zstack-size=0x800000");
  println!("cargo:rustc-link-arg=--export-table");
  println!("cargo:rustc-link-arg=--export=malloc");
  println!("cargo:rustc-link-arg=--export=free");
  setup();
}
