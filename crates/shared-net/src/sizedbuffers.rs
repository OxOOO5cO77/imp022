use std::cmp::min;
use std::mem::size_of;

pub trait Bufferable {
    fn push_into(&self, buf: &mut VSizedBuffer);
    fn pull_from(buf: &mut VSizedBuffer) -> Self;
    fn size_in_buffer(&self) -> usize;
}

pub struct VSizedBuffer {
    pub(crate) raw: Vec<u8>,
    rpos: usize,
    wpos: usize,
}

type SizeMarkerType = u16;

impl VSizedBuffer {
    pub fn new(size: usize) -> Self {
        VSizedBuffer {
            raw: vec![0; size + Self::sizesize()],
            rpos: Self::sizesize(),
            wpos: Self::sizesize(),
        }
    }

    pub const fn sizesize() -> usize {
        size_of::<SizeMarkerType>()
    }

    pub fn extract_size(buf: &[u8]) -> usize {
        let mut size_buf = [0_u8; size_of::<SizeMarkerType>()];
        size_buf.copy_from_slice(&buf[..size_of::<SizeMarkerType>()]);
        SizeMarkerType::from_be_bytes(size_buf) as usize
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
        self.raw[..size_of::<SizeMarkerType>()].copy_from_slice(&SizeMarkerType::to_be_bytes(new_size as SizeMarkerType));
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
    pub fn drain_bytes(&mut self) -> Vec<u8> {
        self.pull_bytes_n(self.remaining())
    }
    pub fn xfer_bytes(&mut self, push: &mut VSizedBuffer) -> &mut Self {
        self.push_bytes(&push.drain_bytes())
    }

    pub fn push<T: Bufferable>(&mut self, ob: &T) -> &mut Self {
        ob.push_into(self);
        self
    }
    pub fn pull<T: Bufferable>(&mut self) -> T {
        T::pull_from(self)
    }
    pub fn xfer<T: Bufferable>(&mut self, other: &mut VSizedBuffer) -> &mut Self {
        self.push(&other.pull::<T>())
    }
}

macro_rules! bufferable_ints {
    (for $($t:ty),+) => {
        $(
        impl Bufferable for $t {
            fn push_into(&self, buf: &mut VSizedBuffer) {
                buf.raw[buf.wpos..buf.wpos + size_of::<Self>()].copy_from_slice(&Self::to_be_bytes(*self));
                buf.stored(size_of::<Self>());
            }

            fn pull_from(buf: &mut VSizedBuffer) -> Self {
                let mut temp_buf = [0_u8; size_of::<Self>()];
                temp_buf.copy_from_slice(&buf.raw[buf.rpos..buf.rpos + size_of::<Self>()]);
                let result = Self::from_be_bytes(temp_buf);
                buf.visited(size_of::<Self>());
                result
            }

            fn size_in_buffer(&self) -> usize {
                size_of::<Self>()
            }
        }
        )*
    }
}

bufferable_ints!(for u8, u16, u32, u64, u128);

impl Bufferable for String {
    fn push_into(&self, buf: &mut VSizedBuffer) {
        (self.len() as u8).push_into(buf);
        let smaller = min(buf.capacity(), self.len());
        let bytes = self.as_bytes();
        buf.raw[buf.wpos..smaller + buf.wpos].copy_from_slice(&bytes[..smaller]);
        buf.stored(bytes.len());
    }

    fn pull_from(buf: &mut VSizedBuffer) -> String {
        let len = u8::pull_from(buf) as usize;
        let mut slice = vec![0; len];
        slice.copy_from_slice(&buf.raw[buf.rpos..buf.rpos + len]);
        buf.visited(len);
        String::from_utf8(slice).unwrap_or_default()
    }

    fn size_in_buffer(&self) -> usize {
        self.len() + 1
    }
}

#[cfg(test)]
mod test {
    mod test_vsizedbuffer {
        use crate::sizedbuffers::Bufferable;
        use crate::VSizedBuffer;

        #[test]
        fn test_u8() {
            let mut buf = VSizedBuffer::new(64);
            let orig = 123_u8;
            buf.push(&orig);
            let result = buf.pull::<u8>();

            assert_eq!(buf.size(), orig.size_in_buffer());
            assert_eq!(orig, result);
        }

        #[test]
        fn test_u16() {
            let mut buf = VSizedBuffer::new(64);
            let orig = 1_234_u16;
            buf.push(&orig);
            let result = buf.pull::<u16>();

            assert_eq!(buf.size(), orig.size_in_buffer());
            assert_eq!(orig, result);
        }

        #[test]
        fn test_u64() {
            let mut buf = VSizedBuffer::new(64);
            let orig = 1_234_567_u64;
            buf.push(&orig);
            let result = buf.pull::<u64>();

            assert_eq!(buf.size(), orig.size_in_buffer());
            assert_eq!(orig, result);
        }

        #[test]
        fn test_u128() {
            let mut buf = VSizedBuffer::new(64);
            let orig = 1_234_567_890_u128;
            buf.push(&orig);
            let result = buf.pull::<u128>();

            assert_eq!(buf.size(), orig.size_in_buffer());
            assert_eq!(orig, result);
        }

        #[test]
        fn test_bytes() {
            let mut target = VSizedBuffer::new(64);
            target.push_bytes(&[1, 2, 3, 4, 5, 6, 7, 8, 9]);
            let result = target.drain_bytes();

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

            let result = target.drain_bytes();

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

            let result = target.drain_bytes();

            assert_eq!(result.len(), 7);
            assert_eq!(result, &[1, 2, 3, 4, 5, 6, 7]);
        }

        #[test]
        fn test_string() {
            let mut source = VSizedBuffer::new(64);
            let mut target = VSizedBuffer::new(64);

            source.push(&"This is a test".to_string());
            let mut total_len = "This is a test".len() + 1;
            assert_eq!(source.size(), total_len);

            source.push(&String::from("So is this"));
            total_len += "So is this".len() + 1;
            assert_eq!(source.size(), total_len);

            let test1 = source.pull::<String>();
            assert_eq!("This is a test", test1);

            target.xfer::<String>(&mut source);

            let test2 = target.pull::<String>();
            assert_eq!("So is this", test2);
        }
    }
}