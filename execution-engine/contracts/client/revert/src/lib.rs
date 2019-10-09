#![no_std]

extern crate contract_ffi;

use contract_ffi::contract_api::{self, Error};

#[no_mangle]
pub extern "C" fn call() {
    contract_api::runtime::revert(Error::User(100))
}
