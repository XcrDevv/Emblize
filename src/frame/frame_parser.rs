// Todo: Add CRC

pub struct FrameParser<'a, const C: usize> {
    buf: [u8; C],
    head: usize,
    tail: usize,
    sync: &'a [u8],
}

impl<'a, const C: usize> FrameParser<'a, C> {
    pub fn new(sync: &'a [u8]) -> Self {
        Self {
            buf: [0; C],
            head: 0,
            tail: 0,
            sync
        }
    }

    pub fn writable(&mut self) -> (&mut [u8], &mut [u8]) {
        let free = self.free_space();
        if free == 0 {
            return (&mut [], &mut []);
        }
        if self.tail >= self.head {
            let (left, right) = self.buf.split_at_mut(self.tail);
            let first_len = right.len();
            let first_len = usize::min(first_len, free);
            let first = &mut right[..first_len];
            let second_len = free - first_len;
            let second = if second_len > 0 && self.head > 0 {
                &mut left[..second_len]
            } else {
                &mut []
            };
            (first, second)
        } else {
            let first = &mut self.buf[self.tail..self.head - 1];
            (first, &mut [])
        }
    }

    pub fn readable(&self) -> (&[u8], &[u8]) {
        if self.tail >= self.head {
            (&self.buf[self.head..self.tail], &[])
        } else {
            (&self.buf[self.head..], &self.buf[..self.tail])
        }
    }

    pub fn advance(&mut self, n: usize) {
        self.tail = (self.tail + n) % C;
    }

    pub fn discard(&mut self, n: usize) {
        self.head = (self.head + n) % C;
    }

    pub fn len(&self) -> usize {
        (self.tail + C - self.head) % C
    }

    pub fn free_space(&self) -> usize {
        C - 1 - self.len()
    }

    pub fn at(&self, i: usize) -> u8 {
        self.buf[(self.head + i) % C]
    }

    fn find_first_sync_byte(&self) -> Option<usize> {
        let len = self.len();

        for i in 0..len {
            if self.at(i) == self.sync[0] {
                return Some(i);
            }
        }

        None
    }

    fn find_sync(&self) -> Option<usize> {
        let sync_len = self.sync.len();
        let len = self.len();
        if len < sync_len {
            return None;
        }

        'outer: for i in 0..=(len - sync_len) {
            for j in 0..sync_len {
                if self.at(i + j) != self.sync[j] {
                    continue 'outer;
                }
            }
            return Some(i);
        }
        None
    }

        pub fn poll_frame(&mut self, out: &mut [u8]) -> Option<usize> {
        let sync_len = self.sync.len();
        if self.len() < sync_len {
            return None;
        }

        let position = match self.find_sync() {
            Some(p) => p,
            None => {
                let safe_discard = match self.find_first_sync_byte() {
                    Some(0) => 1,
                    Some(p) => p,
                    None => self.len(),
                };
                self.discard(safe_discard);
                return None;
            }
        };

        let header_len = sync_len + 2;
        if self.len() < position + header_len {
            self.discard(position);
            return None;
        }

        let len_hi = self.at(position + sync_len);
        let len_lo = self.at(position + sync_len + 1);
        let frame_length = u16::from_be_bytes([len_hi, len_lo]) as usize;

        if frame_length > out.len() {
            self.discard(position + 1);
            return None;
        }

        if self.len() < position + header_len + frame_length {
            self.discard(position);
            return None;
        }

        self.discard(position + header_len);

        let (first, second) = self.readable();
        if first.len() >= frame_length {
            out[..frame_length].copy_from_slice(&first[..frame_length]);
        } else {
            let first_len = first.len();
            out[..first_len].copy_from_slice(first);
            out[first_len..frame_length].copy_from_slice(&second[..frame_length - first_len]);
        }
        self.discard(frame_length);
        Some(frame_length)
    }
    
    pub fn poll_frames<F>(&mut self, buf: &mut [u8], mut f: F)
    where
        F: FnMut(&[u8])
    {
        while let Some(len) = self.poll_frame(buf) {
            f(&buf[..len]);
        }
    }
}