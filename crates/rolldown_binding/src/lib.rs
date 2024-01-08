#[cfg(all(
  not(all(target_os = "linux", target_env = "musl", target_arch = "aarch64")),
  not(debug_assertions),
  not(target_os = "wasi")
))]
#[global_allocator]
static ALLOC: mimalloc_rust::GlobalMiMalloc = mimalloc_rust::GlobalMiMalloc;

pub mod bundler;
pub mod options;
pub mod output;
pub mod utils;
scoped_tls::scoped_thread_local!(static NAPI_ENV: napi::Env);

#[cfg(target_os = "wasi")]
#[napi_derive::napi]
pub fn setup_wasm_panic_hook() {
  use std::sync::Once;

  fn hook_impl(info: &std::panic::PanicInfo) {
    println!("{}", info);
  }

  pub fn hook(info: &std::panic::PanicInfo) {
    hook_impl(info);
  }

  static SET_HOOK: Once = Once::new();
  SET_HOOK.call_once(|| {
    std::panic::set_hook(Box::new(hook));
  });

  let _ =
    miette::set_hook(Box::new(|_| Box::new(miette::MietteHandlerOpts::new().width(400).build())));
}
