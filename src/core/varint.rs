pub const fn varint_usize_max_len() -> usize {
    (usize::BITS as usize + 6) / 7
}

pub fn varint_usize(mut n: usize, out: &mut [u8; varint_usize_max_len()]) -> &[u8] {
    let mut i = 0;

    while n >= 0x80 {
        out[i] = (n as u8 & 0x7F) | 0x80;
        n >>= 7;
        i += 1;
    }

    out[i] = n as u8;
    i += 1;

    &out[..i]
}