use std::io::Cursor;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn encode(s: &str) -> String {
    let encoded = zstd::stream::encode_all(Cursor::new(s.as_bytes()), 20).unwrap();
    base32768::encode(&encoded)
}

#[wasm_bindgen]
pub fn decode(s: &str) -> String {
    let bin = base32768::decode(s).unwrap();
    String::from_utf8(zstd::stream::decode_all(Cursor::new(bin)).unwrap()).unwrap()
}
