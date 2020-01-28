use engine_test_support::{
    internal::{
        DeployItemBuilder, ExecuteRequestBuilder, InMemoryWasmTestBuilder, DEFAULT_GENESIS_CONFIG,
        DEFAULT_PAYMENT, STANDARD_PAYMENT_CONTRACT,
    },
    DEFAULT_ACCOUNT_ADDR,
};
use types::account::PublicKey;

const PASS_INIT_REMOVE: &str = "init_remove";
const PASS_TEST_REMOVE: &str = "test_remove";
const PASS_INIT_UPDATE: &str = "init_update";
const PASS_TEST_UPDATE: &str = "test_update";

const CONTRACT_EE_550_REGRESSION: &str = "ee_550_regression.wasm";
const KEY_2_ADDR: [u8; 32] = [101; 32];
const DEPLOY_HASH: [u8; 32] = [42; 32];

#[ignore]
#[test]
fn should_run_ee_550_remove_with_saturated_threshold_regression() {
    let exec_request_1 = ExecuteRequestBuilder::standard(
        DEFAULT_ACCOUNT_ADDR,
        CONTRACT_EE_550_REGRESSION,
        (String::from(PASS_INIT_REMOVE),),
    )
    .build();

    let exec_request_2 = {
        let deploy_item = DeployItemBuilder::new()
            .with_address(DEFAULT_ACCOUNT_ADDR)
            .with_session_code(
                CONTRACT_EE_550_REGRESSION,
                (String::from(PASS_TEST_REMOVE),),
            )
            .with_payment_code(STANDARD_PAYMENT_CONTRACT, (*DEFAULT_PAYMENT,))
            .with_authorization_keys(&[
                PublicKey::new(DEFAULT_ACCOUNT_ADDR),
                PublicKey::new(KEY_2_ADDR),
            ])
            .with_deploy_hash(DEPLOY_HASH)
            .build();

        ExecuteRequestBuilder::from_deploy_item(deploy_item).build()
    };

    let mut builder = InMemoryWasmTestBuilder::default();

    builder
        .run_genesis(&DEFAULT_GENESIS_CONFIG)
        .exec(exec_request_1)
        .expect_success()
        .commit()
        .exec(exec_request_2)
        .expect_success()
        .commit();
}

#[ignore]
#[test]
fn should_run_ee_550_update_with_saturated_threshold_regression() {
    let exec_request_1 = ExecuteRequestBuilder::standard(
        DEFAULT_ACCOUNT_ADDR,
        CONTRACT_EE_550_REGRESSION,
        (String::from(PASS_INIT_UPDATE),),
    )
    .build();

    let exec_request_2 = {
        let deploy_item = DeployItemBuilder::new()
            .with_address(DEFAULT_ACCOUNT_ADDR)
            .with_session_code(
                CONTRACT_EE_550_REGRESSION,
                (String::from(PASS_TEST_UPDATE),),
            )
            .with_payment_code(STANDARD_PAYMENT_CONTRACT, (*DEFAULT_PAYMENT,))
            .with_authorization_keys(&[
                PublicKey::new(DEFAULT_ACCOUNT_ADDR),
                PublicKey::new(KEY_2_ADDR),
            ])
            .with_deploy_hash(DEPLOY_HASH)
            .build();

        ExecuteRequestBuilder::from_deploy_item(deploy_item).build()
    };

    let mut builder = InMemoryWasmTestBuilder::default();

    builder
        .run_genesis(&DEFAULT_GENESIS_CONFIG)
        .exec(exec_request_1)
        .expect_success()
        .commit()
        .exec(exec_request_2)
        .expect_success()
        .commit();
}
