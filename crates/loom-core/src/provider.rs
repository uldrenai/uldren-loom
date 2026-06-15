//! The provider contract - the low-level store the engine builds on.
//!
//! This trait is synchronous; the asynchronous engine and binding APIs wrap it.

pub mod memory;

use crate::digest::Digest;
use crate::error::Result;

/// A content-addressed object store.
pub trait ObjectStore {
    /// Store canonical object bytes and return their content address.
    ///
    /// Storing an object that already exists is a no-op that returns the same [`Digest`]. The
    /// address is computed as `blake3(canonical)`, so an implementation cannot store a mis-addressed
    /// object.
    fn put(&mut self, canonical: &[u8]) -> Result<Digest>;

    /// Fetch canonical object bytes by address, or `None` if absent.
    fn get(&self, digest: &Digest) -> Result<Option<Vec<u8>>>;

    /// Whether the object exists.
    fn has(&self, digest: &Digest) -> Result<bool>;

    /// Number of distinct objects stored.
    fn len(&self) -> usize;

    /// Whether the store holds no objects.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
