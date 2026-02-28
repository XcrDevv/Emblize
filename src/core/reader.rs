use crate::{core::utils::endian::BytesNum, error::{Error, Result}};

pub struct Reader<'a> {
    buf: &'a [u8],
    pos: usize
}

impl<'a> Reader<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self {
            buf,
            pos: 0,
        }
    }

    pub fn take_bytes(&mut self, size: usize) -> Result<&'a [u8]> {
        let remaining = self.buf.len().saturating_sub(self.pos);

        if size > remaining {
            return Err(Error::UnexpectedEof);
        }

        let end = self.pos + size;
        let slice = &self.buf[self.pos..end];
        self.pos = end;
        Ok(slice)
    }

    pub fn read_byte(&mut self) -> Result<u8> {
        if self.pos + 1 > self.buf.len() {
            return Err(Error::UnexpectedEof);
        }

        let byte = self.buf[self.pos];
        self.pos += 1;

        Ok(byte)
    }

    pub fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> {
        let remaining = self.buf.len().saturating_sub(self.pos);

        if buf.len() > remaining {
            return Err(Error::UnexpectedEof);
        }

        let end = self.pos + buf.len();
        buf.copy_from_slice(&self.buf[self.pos..end]);
        self.pos = end;
        Ok(())
    }

    pub fn read_number<T: BytesNum>(&mut self) -> Result<T> {
        let size = core::mem::size_of::<T::Bytes>();
        if self.pos + size > self.buf.len() {
            return Err(Error::UnexpectedEof);
        }

        let bytes = T::Bytes::try_from(&self.buf[self.pos..self.pos + size]).ok().unwrap();

        self.pos += size;

        Ok(T::from_be_bytes(bytes))
    }

    pub fn read_str_uft8(&mut self, len: usize) -> Result<&'a str> {
        if self.pos + len > self.buf.len() {
            return Err(Error::UnexpectedEof);
        }

        let str_utf8 = str::from_utf8(&self.buf[self.pos..self.pos + len])
            .map_err(|_| Error::NoUTF8)?;
        self.pos += len;

        Ok(str_utf8)
    }
}