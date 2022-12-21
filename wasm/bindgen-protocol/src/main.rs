use fp_bindgen::{prelude::*, types::CargoDependency, BindingConfig, BindingsType};
use once_cell::sync::Lazy;

use std::collections::{BTreeMap, BTreeSet};
const VERSION: &str = "1.0.0";
const AUTHORS: &str = r#"["Alexander Dante Camuto dante@protocol.ai", "Amean Asad <amean.asad@protocol.ai>"]"#;
const NAME: &str = "bindings";


static PLUGIN_DEPENDENCIES: Lazy<BTreeMap<&str, CargoDependency>> = Lazy::new(|| {
/* fp-bindgen-support must be added here so we can pass the features we desire for the
    rust-plugin generation.
*/
    BTreeMap::from([(
        "fp-bindgen-support",
        CargoDependency {
            git: Some("https://github.com/fiberplane/fp-bindgen"),
            features: BTreeSet::from(["async", "guest"]),
            ..CargoDependency::default()
        },
    )])
});


#[derive(Serializable)]
pub struct VerifyParams {
    pub aggregate_signature: Vec<u8>,
    pub pub_keys: Vec<Vec<u8>>,
    pub hashes: Vec<Vec<u8>>,
}

fp_export! {
    fn run_sig_verification(data: VerifyParams) -> bool;
}

fp_import! {}

fn gen_binding(bindings_type: BindingsType) {
    let output_path = format!("../bls-utils/{}", bindings_type);

    fp_bindgen!(BindingConfig {
        bindings_type,
        path: &output_path
    });
}

fn main() {
    let plugin_binding = BindingsType::RustPlugin(RustPluginConfig {
        name: NAME,
        authors: AUTHORS,
        version: VERSION,
        dependencies: PLUGIN_DEPENDENCIES.clone(),
    });

    let runtime_binding = BindingsType::RustWasmerWasiRuntime;

    gen_binding(plugin_binding);
    gen_binding(runtime_binding);
}
