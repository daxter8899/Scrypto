#[rustfmt::skip]
pub mod test_runner;

use radix_engine::{
    ledger::InMemorySubstateStore,
    transaction::{NonceProvider, TransactionBuilder, TransactionExecutor},
    wasm::InvokeError,
};
use scrypto::call_data;
use test_runner::wat2wasm;

#[test]
fn test_loop() {
    // Arrange
    let mut substate_store = InMemorySubstateStore::with_bootstrap();
    let mut executor = TransactionExecutor::new(&mut substate_store, true);

    // Act
    let code = wat2wasm(&include_str!("webassembly/loop.wat").replace("${n_minus_one}", "99"));
    let package_address = executor
        .publish_package(code)
        .expect("Failed to publish package");
    let transaction = TransactionBuilder::new()
        .call_function(package_address, "Test", call_data!(f()))
        .build(executor.get_nonce([]))
        .sign([]);
    let receipt = executor
        .validate_and_execute(&transaction)
        .expect("Failed to execute transaction");

    // Assert
    receipt.result.expect("It should work")
}

#[test]
fn test_loop_out_of_tbd() {
    // Arrange
    let mut substate_store = InMemorySubstateStore::with_bootstrap();
    let mut executor = TransactionExecutor::new(&mut substate_store, true);

    // Act
    let code = wat2wasm(&include_str!("webassembly/loop.wat").replace("${n_minus_one}", "999999"));
    let package_address = executor
        .publish_package(code)
        .expect("Failed to publish package");
    let transaction = TransactionBuilder::new()
        .call_function(package_address, "Test", call_data!(f()))
        .build(executor.get_nonce([]))
        .sign([]);
    let receipt = executor
        .validate_and_execute(&transaction)
        .expect("Failed to execute transaction");

    // Assert
    assert_invoke_error!(receipt.result, InvokeError::OutOfTbd { .. })
}

#[test]
fn test_recursion() {}

#[test]
fn test_recursion_stack_overflow() {}