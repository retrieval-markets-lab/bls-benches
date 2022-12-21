#[cfg(feature="runtime")]
mod runtime;

#[cfg(feature="runtime")]
use runtime::runtime::run_wasm_module;

fn main() {
   
    #[cfg(feature="runtime")]
    run_wasm_module();
}


