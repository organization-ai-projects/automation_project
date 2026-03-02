fn main() {
    let mut bytes = [0_u8, 1_u8, 2_u8];
    // Internal-only unsafe: still reported as warning signal.
    unsafe {
        let ptr = bytes.as_mut_ptr();
        *ptr.add(1) = 42;
    }
    let _masked = bytes[1];
}
