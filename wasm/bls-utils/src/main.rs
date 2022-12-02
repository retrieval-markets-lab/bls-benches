
use fvm_wasm_instrument::{
	gas_metering,
	parity_wasm::{deserialize_buffer, serialize_to_file},
};

use std::{
	fs::read,
	path::Path,
};


fn inject_gas() {
    let path:&Path = Path::new("./fixtures/verify.wasm");
    println!("{}", path.display());
    let bytes: Vec<u8> = read(path).unwrap();
    let file_name = path.file_stem().unwrap().to_str().unwrap();    
    let savepath = Path::new("./fixtures").join(format!("{file_name}-gas-metered.wasm"));
    let module = deserialize_buffer(&bytes).unwrap();

    let gas_injected_module  = 
        gas_metering::inject(module, &gas_metering::ConstantCostRules::default(), "env").unwrap();
    let serialize_result = serialize_to_file(savepath, gas_injected_module);
    let _serialize = match serialize_result {
        Ok(file) => file,
        Err(error) => panic!("Problem serializing module to wasm file: {:?}", error),
    };

}

fn main () {
    inject_gas()
}