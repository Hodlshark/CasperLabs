#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;

extern crate contract_ffi;
use contract_ffi::contract_api::*;

#[no_mangle]
pub extern "C" fn revert_test_ext() {
    // Call revert with an application specific non-zero exit code.
    // It is 2 because another contract used by test_revert.py calls revert with 1.
    revert(Error::User(2));
}

#[no_mangle]
pub extern "C" fn call() {
    let pointer = store_function_at_hash("revert_test_ext", BTreeMap::new());
    put_key("revert_test", &pointer.into())
}