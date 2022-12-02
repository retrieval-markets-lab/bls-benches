
use fvm_wasm_instrument::{
	gas_metering,
    inject_stack_limiter,
	parity_wasm::{deserialize_buffer, serialize_to_file},
};

use std::{
	fs::read,
	path::Path,
};

const STACK_LIMIT: u32  = 128;

fn inject_fvm_modules() {
    let path:&Path = Path::new("./fixtures/verify.wasm");
    println!("{}", path.display());
    let bytes: Vec<u8> = read(path).unwrap();
    let file_name = path.file_stem().unwrap().to_str().unwrap();    
    let savepath = Path::new("./fixtures").join(format!("{file_name}-gas-metered.wasm"));
    let module = deserialize_buffer(&bytes).unwrap();

    let mut injected_module  = 
        gas_metering::inject(module, &gas_metering::ConstantCostRules::default(), "env").unwrap();
    injected_module = inject_stack_limiter(injected_module, STACK_LIMIT).unwrap();

    let serialize_result = serialize_to_file(savepath, injected_module);
    let _serialize = match serialize_result {
        Ok(file) => file,
        Err(error) => panic!("Problem serializing module to wasm file: {:?}", error),
    };

}

fn main () {
    inject_fvm_modules()
}