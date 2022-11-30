use fvm::call_manager::DefaultCallManager;
use fvm::executor::{ApplyKind, DefaultExecutor, Executor};
use fvm::kernel::default::DefaultKernel;
use fvm::machine::DefaultMachine;
use fvm_integration_tests::bundle;
use fvm_integration_tests::dummy::DummyExterns;
use fvm_integration_tests::tester::{Account, Tester};
use fvm_ipld_blockstore::MemoryBlockstore;
use fvm_ipld_encoding::{tuple::*, RawBytes};
use fvm_shared::address::Address;
use fvm_shared::econ::TokenAmount;
use fvm_shared::message::Message;
use fvm_shared::state::StateTreeVersion;
use fvm_shared::version::NetworkVersion;
use log::debug;
use num_traits::Zero;
use std::sync::Once;
/// Setup function that is only run once, even if called multiple times.
static INIT: Once = Once::new();

pub fn setup_logs() {
    INIT.call_once(|| {
        colog::init();
    });
}

pub fn new_tester() -> Tester<MemoryBlockstore, DummyExterns> {
    // Instantiate tester
    let bs = MemoryBlockstore::default();
    let bundle_root = bundle::import_bundle(&bs, actors_v10::BUNDLE_CAR).unwrap();
    Tester::new(NetworkVersion::V18, StateTreeVersion::V5, bundle_root, bs).unwrap()
}

pub fn setup_actor(
    mut tester: Tester<MemoryBlockstore, DummyExterns>,
    binary: Option<&[u8]>,
) -> (
    Address,
    DefaultExecutor<
        DefaultKernel<DefaultCallManager<DefaultMachine<MemoryBlockstore, DummyExterns>>>,
    >,
) {
    #[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug)]
    struct State {
        count: usize,
    }

    let sender: [Account; 1] = tester.create_accounts().unwrap();

    // Get wasm bin
    let wasm_bin = binary.unwrap();

    // Set actor state
    let actor_state = State { count: 0 };
    let state_cid = tester.set_state(&actor_state).unwrap();

    // Set actor
    let actor_address = Address::new_id(10000);

    tester
        .set_actor_from_bin(&wasm_bin, state_cid, actor_address, TokenAmount::zero())
        .unwrap();

    // Instantiate machine
    tester.instantiate_machine(DummyExterns).unwrap();
    let mut executor = tester.executor.unwrap();
    // Send message
    let message = Message {
        from: sender[0].1,
        to: actor_address,
        gas_limit: 1000000000,
        method_num: 1,
        sequence: 0,
        ..Message::default()
    };

    let res = executor.execute_message(message, ApplyKind::Explicit, 100);

    assert!(res.is_ok());

    (actor_address, executor)
}

pub fn call_function<E: Executor>(
    executor: &mut E,
    actor_address: Address,
    params: RawBytes,
    sender: [Account; 1],
    i: &mut u64,
) {
    // Send message
    let message = Message {
        from: sender[0].1,
        to: actor_address,
        gas_limit: 1000000000000,
        method_num: 2,
        sequence: *i,
        params,
        ..Message::default()
    };

    let res = executor.execute_message(message, ApplyKind::Explicit, 100);

    assert!(res.is_ok());

    let res = res.unwrap();
    debug!(
        "Return data: {:?} | Gas used {:?} | Gas burned {:?}",
        res.msg_receipt.return_data.deserialize::<String>().unwrap(),
        res.msg_receipt.gas_used,
        res.gas_burned
    );

    *i += 1;
}
