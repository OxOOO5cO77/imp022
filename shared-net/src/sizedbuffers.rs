use std::cmp::min;
use std::mem::size_of;

type SizeType = u16;

pub struct VSizedBuffer {
    pub(crate) raw: Vec<u8>,
    rpos: usize,
    wpos: usize,
}

impl VSizedBuffer {
    pub fn new(size: usize) -> Self {
        VSizedBuffer {
            raw: vec![0; size + Self::sizesize()],
            rpos: Self::sizesize(),
            wpos: Self::sizesize(),
        }
    }

    pub const fn sizesize() -> usize {
        size_of::<SizeType>()
    }

    pub fn extract_size(buf: &[u8]) -> usize {
        let mut size_buf = [0_u8; size_of::<SizeType>()];
        size_buf.copy_from_slice(&buf[..size_of::<SizeType>()]);
        SizeType::from_be_bytes(size_buf) as usize
    }

    pub fn rewind(&mut self) -> &mut Self {
        self.rpos = Self::sizesize();
        self
    }

    pub fn reset(&mut self) {
        self.raw[..Self::sizesize()].copy_from_slice(&[0_u8; Self::sizesize()]);
        self.wpos = Self::sizesize();
        self.rpos = Self::sizesize();
    }

    pub fn capacity(&self) -> usize {
        self.raw.capacity() - self.wpos
    }

    pub fn remaining(&self) -> usize {
        self.size() - (self.rpos - Self::sizesize())
    }

    pub fn size(&self) -> usize {
        VSizedBuffer::extract_size(&self.raw)
    }

    pub fn set_size(&mut self, new_size: usize) -> &mut Self {
        self.raw[..size_of::<SizeType>()].copy_from_slice(&SizeType::to_be_bytes(new_size as SizeType));
        self
    }

    fn visited(&mut self, bytes: usize) -> &mut Self {
        self.rpos += bytes;
        self
    }

    fn stored(&mut self, bytes: usize) -> &mut Self {
        let new_size = self.size() + bytes;
        self.set_size(new_size);
        self.wpos += bytes;
        self
    }

    pub fn push_u8(&mut self, push: &u8) -> &mut Self {
        self.raw[self.wpos] = *push;
        self.stored(size_of::<u8>())
    }
    pub fn pull_u8(&mut self) -> u8 {
        let result = self.raw[self.rpos];
        self.visited(size_of::<u8>());
        result
    }
    pub fn xfer_u8(&mut self, push: &mut VSizedBuffer) -> &mut Self {
        self.push_u8(&push.pull_u8())
    }

    pub fn push_u16(&mut self, push: &u16) -> &mut Self {
        self.raw[self.wpos..self.wpos + size_of::<u16>()].copy_from_slice(&u16::to_be_bytes(*push));
        self.stored(size_of::<u16>())
    }
    pub fn pull_u16(&mut self) -> u16 {
        let mut buf = [0_u8; size_of::<u16>()];
        buf.copy_from_slice(&self.raw[self.rpos..self.rpos + size_of::<u16>()]);
        let result = u16::from_be_bytes(buf);
        self.visited(size_of::<u16>());
        result
    }
    pub fn xfer_u16(&mut self, push: &mut VSizedBuffer) -> &mut Self {
        self.push_u16(&push.pull_u16())
    }

    pub fn push_u128(&mut self, push: &u128) -> &mut Self {
        self.raw[self.wpos..self.wpos + size_of::<u128>()].copy_from_slice(&u128::to_be_bytes(*push));
        self.stored(size_of::<u128>())
    }
    pub fn pull_u128(&mut self) -> u128 {
        let mut buf = [0_u8; size_of::<u128>()];
        buf.copy_from_slice(&self.raw[self.rpos..self.rpos + size_of::<u128>()]);
        let result = u128::from_be_bytes(buf);
        self.visited(size_of::<u128>());
        result
    }
    pub fn xfer_u128(&mut self, push: &mut VSizedBuffer) -> &mut Self {
        self.push_u128(&push.pull_u128())
    }

    pub fn push_bytes(&mut self, push: &[u8]) -> &mut Self {
        let smaller = min(self.capacity(), push.len());
        self.raw[self.wpos..smaller + self.wpos].copy_from_slice(&push[..smaller]);
        self.stored(smaller)
    }
    pub fn pull_bytes_n(&mut self, bytes: usize) -> Vec<u8> {
        let mut slice = vec![0; bytes];
        slice.copy_from_slice(&self.raw[self.rpos..self.rpos + bytes]);
        self.visited(bytes);
        slice
    }
    pub fn pull_bytes(&mut self) -> Vec<u8> {
        self.pull_bytes_n(self.remaining())
    }
    pub fn xfer_bytes(&mut self, push: &mut VSizedBuffer) -> &mut Self {
        self.push_bytes(&push.pull_bytes())
    }

    pub fn push_string(&mut self, push: &str) -> &mut Self {
        self.push_u8(&(push.len() as u8));
        self.push_bytes(push.as_bytes())
    }
    pub fn pull_string(&mut self) -> String {
        let len = self.pull_u8() as usize;
        String::from_utf8(self.pull_bytes_n(len)).unwrap_or_default()
    }
    pub fn xfer_string(&mut self, push: &mut VSizedBuffer) -> &mut Self {
        self.push_string(&push.pull_string())
    }
}

#[cfg(test)]
mod test {
    mod test_vsizedbuffer {
        use crate::VSizedBuffer;

        #[test]
        fn test_u8() {
            let mut buf = VSizedBuffer::new(64);
            buf.push_u8(&123);
            let result = buf.pull_u8();

            assert_eq!(buf.size(), 1);
            assert_eq!(result, 123);
        }

        #[test]
        fn test_u16() {
            let mut buf = VSizedBuffer::new(64);
            buf.push_u128(&1_234_567_890);
            let result = buf.pull_u128();

            assert_eq!(buf.size(), 16);
            assert_eq!(result, 1_234_567_890);
        }

        #[test]
        fn test_u128() {
            let mut buf = VSizedBuffer::new(64);
            buf.push_u128(&1_234_567_890);
            let result = buf.pull_u128();

            assert_eq!(buf.size(), 16);
            assert_eq!(result, 1_234_567_890);
        }

        #[test]
        fn test_bytes() {
            let mut target = VSizedBuffer::new(64);
            target.push_bytes(&[1, 2, 3, 4, 5, 6, 7, 8, 9]);
            let result = target.pull_bytes();

            assert_eq!(target.size(), 9);
            assert_eq!(result, &[1, 2, 3, 4, 5, 6, 7, 8, 9]);
        }

        #[test]
        fn test_buffer() {
            let mut target = VSizedBuffer::new(64);
            let mut source = VSizedBuffer::new(64);
            target.push_bytes(&[1, 2, 3, 4]);
            assert_eq!(target.size(), 4);

            source.push_bytes(&[5, 6, 7, 8, 9]);
            assert_eq!(source.size(), 5);

            target.xfer_bytes(&mut source);
            assert_eq!(target.size(), 9);

            let result = target.pull_bytes();

            assert_eq!(result.len(), 9);
            assert_eq!(result, &[1, 2, 3, 4, 5, 6, 7, 8, 9]);
        }

        #[test]
        fn test_bytes_n() {
            let mut target = VSizedBuffer::new(64);
            let mut source = VSizedBuffer::new(64);
            target.push_bytes(&[1, 2, 3, 4]);
            assert_eq!(target.size(), 4);

            source.push_bytes(&[5, 6, 7, 8, 9]);
            assert_eq!(source.size(), 5);

            target.push_bytes(&source.pull_bytes_n(3));
            assert_eq!(source.remaining(), 2);
            assert_eq!(target.size(), 7);

            let result = target.pull_bytes();

            assert_eq!(result.len(), 7);
            assert_eq!(result, &[1, 2, 3, 4, 5, 6, 7]);
        }

        #[test]
        fn test_string() {
            let mut source = VSizedBuffer::new(64);
            let mut target = VSizedBuffer::new(64);

            source.push_string("This is a test");
            let mut total_len = "This is a test".len() + 1;
            assert_eq!(source.size(), total_len);

            source.push_string(String::from("So is this").as_str());
            total_len += "So is this".len() + 1;
            assert_eq!(source.size(), total_len);

            let test1 = source.pull_string();
            assert_eq!(test1, "This is a test");

            target.xfer_string(&mut source);

            let test2 = target.pull_string();
            assert_eq!(test2, "So is this");
        }
    }
}