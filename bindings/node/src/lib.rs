//! Node.js binding for Uldren Loom via napi-rs. Published as `@uldrenai/loom`.
//!
//! napi maps snake_case Rust to camelCase JS, so `blob_digest` is `blobDigest` in JavaScript.
//! Licensed under BUSL-1.1 (see the repo `LICENSE`). © Uldren Technologies LLC.

use loom_core::Object;
use napi::bindgen_prelude::Uint8Array;
use napi_derive::napi;

/// The library version.
#[napi]
pub fn version() -> String {
    loom_core::VERSION.to_string()
}

/// Compute the Blob content address (`"algo:hex"`) of the given bytes.
#[napi]
pub fn blob_digest(data: Uint8Array) -> String {
    Object::Blob(data.to_vec()).digest().to_string()
}
