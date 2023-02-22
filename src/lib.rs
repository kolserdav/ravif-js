use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
pub fn say_hello() {
    println!("Hello, world!");
}
