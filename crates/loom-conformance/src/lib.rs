//! Shared conformance vectors plus a generic runner.
//!
//! The [`BLOB_VECTORS`] are canonical test vectors: fixed inputs mapped to the fixed `blake3:`
//! address of the corresponding [`Object::Blob`]. Every implementation, in every language, must
//! reproduce these exact digests - this is what pins the data model across the polyglot bindings.

use loom_core::{Digest, Object, ObjectStore, Result};

/// One canonical blob vector: input bytes → expected `algo:hex` address of `Object::Blob(input)`.
#[derive(Debug, Clone, Copy)]
pub struct BlobVector {
    /// Human-readable name (for test output).
    pub name: &'static str,
    /// The raw blob content.
    pub input: &'static [u8],
    /// The expected content address of `Object::Blob(input)`.
    pub expect_digest: &'static str,
}

/// The canonical blob vectors. Keep in sync across all implementations.
pub const BLOB_VECTORS: &[BlobVector] = &[
    BlobVector {
        name: "empty",
        input: b"",
        expect_digest: "blake3:687376c930d7020a32f04c396fc2e5eab49cd09a738fa03d573033416a6a47ce",
    },
    BlobVector {
        name: "abc",
        input: b"abc",
        expect_digest: "blake3:314b0f564f15b78141e461fb4e95f27b7ef721e4dd38da327f6da8e304104058",
    },
    BlobVector {
        name: "hello-loom",
        input: b"hello loom",
        expect_digest: "blake3:3c7e3dbb1517bba8ebd66b707b60951c5fb4abdc2ad24da0ebeca57a30b09fbd",
    },
    BlobVector {
        name: "big-200",
        input: &[0u8; 200],
        expect_digest: "blake3:b379698acb8a6d10a763a1c4a7e9f164808b0361e1854400ebba5e75259b813f",
    },
];

/// Run the canonical blob vectors against any [`ObjectStore`].
///
/// For each vector this asserts that (a) `Object::Blob(input).digest()` equals the published
/// address, and (b) the store round-trips: `put` returns that address and `get` returns the exact
/// canonical bytes.
pub fn run_blob_vectors<S: ObjectStore>(store: &mut S) -> Result<()> {
    for v in BLOB_VECTORS {
        let obj = Object::Blob(v.input.to_vec());
        let expected = Digest::parse(v.expect_digest)?;
        assert_eq!(
            obj.digest(),
            expected,
            "data-model digest mismatch for vector '{}'",
            v.name
        );

        let stored = store.put(&obj.canonical())?;
        assert_eq!(
            stored, expected,
            "store returned wrong address for vector '{}'",
            v.name
        );
        assert_eq!(
            store.get(&stored)?.as_deref(),
            Some(obj.canonical().as_slice()),
            "store round-trip mismatch for vector '{}'",
            v.name
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use loom_core::MemoryStore;

    #[test]
    fn memory_store_passes_blob_vectors() {
        let mut store = MemoryStore::new();
        run_blob_vectors(&mut store).expect("MemoryStore must pass the canonical vectors");
        assert_eq!(store.len(), BLOB_VECTORS.len());
    }
}
