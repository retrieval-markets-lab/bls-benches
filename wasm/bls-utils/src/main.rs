#[cfg(feature="runtime")]
mod runtime;

#[cfg(feature="runtime")]
use runtime::{
    runtime::run_wasm_module,
    fvm_injection::inject_fvm_modules,
};

fn main() {
   
    #[cfg(feature="runtime")]
    inject_fvm_modules();
    #[cfg(feature="runtime")]
    run_wasm_module();
}


