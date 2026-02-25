pub struct FrameBuf<const N: usize> {
    pub buf: [u8; N],
    pub len: usize
}

impl<const N: usize> FrameBuf<N> {
    pub fn new() -> Self {
        Self {
            buf: [0; N],
            len: 0
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.buf[..self.len]
    }
}