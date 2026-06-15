//! WASM binding for Uldren Loom via wasm-bindgen. Published as `@uldrenai/loom-wasm`.
//!
//! The browser / JS-runtime path, and the universal fallback when no prebuilt native binary exists.
//! Licensed under BUSL-1.1 (see the repo `LICENSE`). © Uldren Technologies LLC.

use loom_core::Object;
use wasm_bindgen::prelude::*;

/// The library version.
#[wasm_bindgen]
pub fn version() -> String {
    loom_core::VERSION.to_string()
}

/// Compute the Blob content address (`"algo:hex"`) of the given bytes.
#[wasm_bindgen]
pub fn blob_digest(data: &[u8]) -> String {
    Object::Blob(data.to_vec()).digest().to_string()
}
