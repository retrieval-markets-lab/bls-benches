mod lib;

use fvm_wasm_instrument::{
	gas_metering,
    inject_stack_limiter,
	parity_wasm::{deserialize_buffer, serialize_to_file},
};

use std::{
	fs::read,
	path::Path,
};

use lib::make_sig_safe;
use fvm_ipld_encoding::tuple::{Deserialize_tuple, Serialize_tuple};
use fvm_ipld_encoding::{RawBytes};
use bls_signatures::{Serialize as BlsSerialize};
use fvm_shared::crypto::{signature::Signature};
use group::{GroupEncoding};

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
        Err(error) => panic!("Error serializing module to wasm file: {:?}", error),
    };
}


#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug)]
struct VerifyParams {
    pub aggregate_signature: Vec<u8>,
    pub pub_keys: Vec<Vec<u8>>,
    pub hashes: Vec<Vec<u8>>,
}

fn main () {
    #[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug)]
    struct VerifyParams {
        pub aggregate_signature: Signature,
        pub pub_keys: Vec<Vec<u8>>,
        pub hashes: Vec<Vec<u8>>,
    }
    let (
        aggregated_signature, 
        hashes, 
        public_keys, _
    ) = make_sig_safe(1, 64);
    
    let signature_bytes = Signature::new_bls(aggregated_signature.as_bytes());
    let public_keys: Vec<_> = public_keys.iter().map(|x| x.as_bytes()).collect();
    let hash_vec: Vec<_> = hashes.iter().map(|x| x.to_bytes().as_mut().to_vec()).collect();
   
    let params = VerifyParams {
        aggregate_signature: signature_bytes,
        pub_keys: public_keys,
        hashes: hash_vec
    };

    // println!("{:?}", params);

    let params =  RawBytes::serialize(params).unwrap();
    println!("{:?}", params.to_vec());

    let params = match RawBytes::new(params.to_vec()).deserialize() {
        Ok(p) => p,
        Err(err) => {
            panic!("Error: {:?}", err);
        }
    };

}