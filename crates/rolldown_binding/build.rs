fn main() {
  use napi_build::setup;
  println!("cargo:rustc-link-arg=--max-memory=2147483648");
  // lld only allocates 1MiB for the WebAssembly stack, and the array that you're allocating on the stack is exactly 1MiB.
  // 0x800000 bytes = 8MiB
  println!("cargo:rustc-link-arg=-zstack-size=0x800000");
  setup();
}
