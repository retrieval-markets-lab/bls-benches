use fvm_wasm_instrument::gas_metering;
use parity_wasm::elements::{serialize_to_file, Module};
use std::{fs::read, path::Path};

// const STACK_LIMIT: u32 = 128;

pub fn inject_fvm_modules(file_path: &Path) {
    let bytes: Vec<u8> = read(file_path).unwrap();
    let file_name = file_path.file_stem().unwrap().to_str().unwrap();
    let savepath = Path::new("./wasm-files").join(format!("{file_name}_fvm_injected.wasm"));
    let injected_module =
        gas_metering::inject(&bytes, &gas_metering::ConstantCostRules::default(), "env").unwrap();

    // TODO: Enable stack limiter
    // let injected_module = inject_stack_limiter(&bytes, STACK_LIMIT).unwrap();

    let save_module = Module::from_bytes(injected_module).unwrap();

    let serialize_result = serialize_to_file(savepath, save_module);
    match serialize_result {
        Ok(file) => file,
        Err(error) => panic!("Error serializing module to wasm file: {:?}", error),
    };
}
