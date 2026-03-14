//! projects/products/unstable/neurosymbolic_moe/backend/src/orchestrator/checksum_writer.rs
pub(crate) fn append_segment_delimiter(
    buffer: &mut String,
    delimiter: &str,
    first_segment: &mut bool,
) {
    if !*first_segment {
        buffer.push_str(delimiter);
    }
    *first_segment = false;
}

pub(crate) fn fnv1a64_init() -> u64 {
    0xcbf29ce484222325_u64
}

pub(crate) fn fnv1a64_update(hash: &mut u64, bytes: &[u8]) {
    for byte in bytes {
        *hash ^= u64::from(*byte);
        *hash = hash.wrapping_mul(0x100000001b3);
    }
}
