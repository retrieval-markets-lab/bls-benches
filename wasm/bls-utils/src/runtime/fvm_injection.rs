use fvm_wasm_instrument::gas_metering;
use parity_wasm::elements::{serialize_to_file, Module};
use std::{fs::read, path::Path};


// const STACK_LIMIT: u32 = 128;

pub fn inject_fvm_modules() {
    let path: &Path = Path::new("./wasm-files/bls_utils.wasm");
    println!("{}", path.display());
    let bytes: Vec<u8> = read(path).unwrap();
    let file_name = path.file_stem().unwrap().to_str().unwrap();
    let savepath = Path::new("./wasm-files").join(format!("{file_name}_fvm_injected.wasm"));
    let injected_module =
        gas_metering::inject(&bytes, &gas_metering::ConstantCostRules::default(), "env").unwrap();

    // TODO: Enable stack limiter
    // let injected_module = inject_stack_limiter(&bytes, STACK_LIMIT).unwrap();

    let save_module = Module::from_bytes(&injected_module).unwrap();

    let serialize_result = serialize_to_file(savepath, save_module);
    let _serialize = match serialize_result {
        Ok(file) => file,
        Err(error) => panic!("Error serializing module to wasm file: {:?}", error),
    };
}

