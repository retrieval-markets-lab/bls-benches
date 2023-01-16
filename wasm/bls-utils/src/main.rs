#[cfg(feature = "runtime")]
mod runtime;

#[cfg(feature = "runtime")]
use runtime::{fvm_injection::inject_fvm_modules, module::run_wasm_module};

use std::path::Path;

fn main() {
    let file_path = Path::new("./wasm-files/bls_utils.wasm");
    #[cfg(feature = "runtime")]
    inject_fvm_modules(file_path);

    let module_path = Path::new("./wasm-files/bls_utils_fvm_injected.wasm");
    #[cfg(feature = "runtime")]
    run_wasm_module(module_path);
}
