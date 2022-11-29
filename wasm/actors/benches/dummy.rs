use actor_utils::*;
use criterion::{criterion_group, criterion_main, Criterion};
use fvm::executor::{ApplyKind, Executor};
use fvm_integration_tests::dummy::DummyExterns;
use fvm_integration_tests::tester::Account;
use fvm_ipld_encoding::tuple::*;
use fvm_shared::address::Address;
use fvm_shared::econ::TokenAmount;
use fvm_shared::message::Message;
use log::trace;
use num_traits::Zero;

const WAT: &str = r#"
;; Mock invoke function
(module
  (func (export "invoke") (param $x i32) (result i32)
    (i32.const 1)
  )
)
"#;

macro_rules! bench_dummy {
    ($name:ident) => {
        fn $name(c: &mut Criterion) {
            setup_logs();

            #[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug)]
            struct State {
                empty: bool,
            }

            let mut tester = new_tester();

            let sender: [Account; 1] = tester.create_accounts().unwrap();

            // Get wasm bin
            let wasm_bin = wat::parse_str(WAT).unwrap();

            // Set actor state
            let actor_state = State { empty: true };
            let state_cid = tester.set_state(&actor_state).unwrap();

            // Set actor
            let actor_address = Address::new_id(10000);

            tester
                .set_actor_from_bin(&wasm_bin, state_cid, actor_address, TokenAmount::zero())
                .unwrap();

            // Instantiate machine
            tester.instantiate_machine(DummyExterns).unwrap();
            let mut executor = tester.executor.unwrap();

            c.bench_function(&format!("dummy"), |b| {
                b.iter(|| {
                    // Send message
                    // Send message
                    let message = Message {
                        from: sender[0].1,
                        to: actor_address,
                        gas_limit: 1000000000,
                        method_num: 1,
                        ..Message::default()
                    };

                    let res = executor.execute_message(message, ApplyKind::Explicit, 100);

                    assert!(res.is_ok());
                    let res = res.unwrap();
                    trace!(
                        "Return data: {:?} | Gas used {:?} | Gas burned {:?}",
                        res.msg_receipt.return_data.deserialize::<String>().unwrap(),
                        res.msg_receipt.gas_used,
                        res.gas_burned
                    );
                })
            });
        }
    };
}

bench_dummy!(bench_dummy);

criterion_group!(benches, bench_dummy,);

criterion_main!(benches);
