use fvm_wasm_instrument::gas_metering;
use parity_wasm::elements::{serialize_to_file, Module};
use std::{fs::read, path::Path};

use bls_signatures::Serialize as BlsSerialize;
use bls_utils::{make_sig_safe, run_sig_verification, VerifyParams};
use fvm_ipld_encoding::RawBytes;
use fvm_shared::crypto::signature::Signature;
use group::GroupEncoding;
use wasmer::{imports, FromToNativeWasmType, Instance, Module as WasmModule, Store, Value};

const STACK_LIMIT: u32 = 128;

fn inject_fvm_modules() {
    let path: &Path = Path::new("../fixtures/verify.wasm");
    println!("{}", path.display());
    let bytes: Vec<u8> = read(path).unwrap();
    let file_name = path.file_stem().unwrap().to_str().unwrap();
    let savepath = Path::new("../fixtures").join(format!("{file_name}-fvm-injected.wasm"));
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

fn import_wasm_module() {
    let wasm_file = Path::new("./fixtures/fvm.wasm");

    let mut store = Store::default();
    let module = WasmModule::from_file(&store, &wasm_file).unwrap();
    // The module doesn't import anything, so we create an empty import object.
    let import_object = imports! {};
    let instance = Instance::new(&mut store, &module, &import_object).unwrap();

    let add_one = instance
        .exports
        .get_function("run_sig_verification")
        .unwrap();
    let result = add_one.call(&mut store, &[Value::I32(i32::from(0_u8))]);
}

fn test_wasm_bundle() {
    let (aggregated_signature, hashes, public_keys, _) = make_sig_safe(1, 64);

    let signature_bytes = Signature::new_bls(aggregated_signature.as_bytes());
    let public_keys: Vec<_> = public_keys.iter().map(|x| x.as_bytes()).collect();
    let hash_vec: Vec<_> = hashes
        .iter()
        .map(|x| x.to_bytes().as_mut().to_vec())
        .collect();

    let params = VerifyParams {
        aggregate_signature: signature_bytes,
        pub_keys: public_keys,
        hashes: hash_vec,
    };

    let params = RawBytes::serialize(params).unwrap();

    let res = run_sig_verification(params.bytes());
}

fn main() {
    test_wasm_bundle()
}
