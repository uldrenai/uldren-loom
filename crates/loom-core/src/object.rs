//! Content objects and their canonical serialization.
//!
//! Every object has exactly one canonical byte form; its [`Digest`] is the hash of that form. The
//! canonical framing is `[type:1][len:uvarint][body]`, which binds the object type into the hash for
//! domain separation. [`ObjectType`] enumerates the object kinds; [`Object`] currently constructs
//! the [`Object::Blob`] variant.

use crate::digest::Digest;

/// The object-type tags. Discriminants are the on-disk `object_type` byte.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ObjectType {
    /// Opaque file content (or one chunk of it).
    Blob = 0x01,
    /// Ordered list of chunk digests composing a large file.
    ChunkList = 0x02,
    /// A directory: name → entry map with metadata.
    Tree = 0x03,
    /// A snapshot: root tree + parents + author + message.
    Commit = 0x04,
    /// A named, optionally-signed pointer to another object.
    Tag = 0x05,
}

/// A content object.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Object {
    /// Opaque bytes.
    Blob(Vec<u8>),
}

impl Object {
    /// The object-type tag for this object.
    pub fn object_type(&self) -> ObjectType {
        match self {
            Object::Blob(_) => ObjectType::Blob,
        }
    }

    /// Canonical serialization: `[type:1][len:uvarint][body]`.
    pub fn canonical(&self) -> Vec<u8> {
        let (ty, body): (ObjectType, &[u8]) = match self {
            Object::Blob(b) => (ObjectType::Blob, b),
        };
        let mut out = Vec::with_capacity(body.len() + 10);
        out.push(ty as u8);
        write_uvarint(&mut out, body.len() as u64);
        out.extend_from_slice(body);
        out
    }

    /// The content address of this object (BLAKE3 of its canonical form).
    pub fn digest(&self) -> Digest {
        Digest::blake3(&self.canonical())
    }
}

/// Append an unsigned LEB128 varint to `out`.
fn write_uvarint(out: &mut Vec<u8>, mut value: u64) {
    loop {
        let mut byte = (value & 0x7f) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        out.push(byte);
        if value == 0 {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blob_canonical_framing() {
        // Empty blob: [0x01 type][0x00 len].
        assert_eq!(Object::Blob(vec![]).canonical(), vec![0x01, 0x00]);
        // 3-byte blob "abc": [0x01][0x03]['a','b','c'].
        assert_eq!(
            Object::Blob(b"abc".to_vec()).canonical(),
            vec![0x01, 0x03, b'a', b'b', b'c']
        );
    }

    #[test]
    fn uvarint_multibyte_length() {
        // 200 bytes => length encodes as 0xC8 0x01 (LEB128).
        let blob = Object::Blob(vec![0u8; 200]);
        let c = blob.canonical();
        assert_eq!(&c[..3], &[0x01, 0xC8, 0x01]);
        assert_eq!(c.len(), 1 + 2 + 200);
    }

    #[test]
    fn digest_is_deterministic_and_type_tagged() {
        let a = Object::Blob(b"abc".to_vec()).digest();
        let b = Object::Blob(b"abc".to_vec()).digest();
        assert_eq!(a, b);
        // The blob digest is over canonical bytes (with the type tag), so it differs from the raw
        // blake3 of "abc".
        assert_ne!(a, Digest::blake3(b"abc"));
    }
}
