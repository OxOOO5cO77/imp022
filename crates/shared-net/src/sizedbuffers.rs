use std::cmp::min;
use std::mem::size_of;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum SizedBufferError {
    Read(usize, usize),
    Write(usize, usize),
    Utf8Error(FromUtf8Error),
}

pub trait Bufferable: Sized {
    fn push_into(&self, buf: &mut SizedBuffer) -> Result<usize, SizedBufferError>;
    fn pull_from(buf: &mut SizedBuffer) -> Result<Self, SizedBufferError>;
    fn size_in_buffer(&self) -> usize;
}

#[derive(Clone)]
pub struct SizedBuffer {
    pub(crate) raw: Vec<u8>,
    rpos: usize,
    wpos: usize,
}

type SizeMarkerType = u16;

impl SizedBuffer {
    pub fn new(size: usize) -> Self {
        SizedBuffer {
            raw: vec![0; size + Self::sizesize()],
            rpos: Self::sizesize(),
            wpos: Self::sizesize(),
        }
    }

    pub fn from<T: Bufferable>(bufferable: &T) -> Result<Self, SizedBufferError> {
        let mut result = Self::new(bufferable.size_in_buffer());
        result.push(bufferable)?;
        Ok(result)
    }

    pub const fn sizesize() -> usize {
        size_of::<SizeMarkerType>()
    }

    pub fn extract_size(buf: &[u8]) -> usize {
        let mut size_buf = [0_u8; size_of::<SizeMarkerType>()];
        size_buf.copy_from_slice(&buf[..size_of::<SizeMarkerType>()]);
        SizeMarkerType::from_le_bytes(size_buf) as usize
    }

    pub fn rewind(&mut self) {
        self.rpos = Self::sizesize();
    }

    pub fn reset(&mut self) {
        self.raw[..Self::sizesize()].copy_from_slice(&[0_u8; Self::sizesize()]);
        self.wpos = Self::sizesize();
        self.rpos = Self::sizesize();
    }

    pub fn write_remain(&self) -> usize {
        self.raw.capacity() - self.wpos
    }

    pub fn read_remain(&self) -> usize {
        self.size() - (self.rpos - Self::sizesize())
    }

    pub fn size(&self) -> usize {
        SizedBuffer::extract_size(&self.raw)
    }

    pub fn set_size(&mut self, new_size: usize) {
        self.raw[..size_of::<SizeMarkerType>()].copy_from_slice(&SizeMarkerType::to_le_bytes(new_size as SizeMarkerType));
    }

    fn visited(&mut self, bytes: usize) {
        self.rpos += bytes;
    }

    fn stored(&mut self, bytes: usize) {
        let new_size = self.size() + bytes;
        self.set_size(new_size);
        self.wpos += bytes;
    }

    pub fn push_bytes(&mut self, push: &[u8]) -> Result<usize, SizedBufferError> {
        let len = push.len();
        if len > self.write_remain() {
            return Err(SizedBufferError::Write(len, self.write_remain()));
        }
        self.raw[self.wpos..len + self.wpos].copy_from_slice(&push[..len]);
        self.stored(len);
        Ok(len)
    }
    pub fn pull_bytes_n(&mut self, bytes: usize) -> Result<Vec<u8>, SizedBufferError> {
        if bytes > self.read_remain() {
            return Err(SizedBufferError::Read(bytes, self.read_remain()));
        }
        let mut slice = vec![0; bytes];
        slice.copy_from_slice(&self.raw[self.rpos..self.rpos + bytes]);
        self.visited(bytes);
        Ok(slice)
    }
    pub fn pull_remaining(&mut self) -> Result<Vec<u8>, SizedBufferError> {
        self.pull_bytes_n(self.read_remain())
    }
    pub fn xfer_bytes(&mut self, push: &mut SizedBuffer) -> Result<usize, SizedBufferError> {
        self.push_bytes(&push.pull_remaining()?)
    }

    pub fn push<T: Bufferable>(&mut self, ob: &T) -> Result<usize, SizedBufferError> {
        ob.push_into(self)
    }
    pub fn pull<T: Bufferable>(&mut self) -> Result<T, SizedBufferError> {
        T::pull_from(self)
    }
    pub fn xfer<T: Bufferable>(&mut self, other: &mut SizedBuffer) -> Result<usize, SizedBufferError> {
        self.push(&other.pull::<T>()?)
    }
}

macro_rules! bufferable_ints {
    (for $($t:ty),+) => {
        $(
        impl Bufferable for $t {
            fn push_into(&self, buf: &mut SizedBuffer) -> Result<usize, SizedBufferError> {
                if size_of::<Self>() > buf.write_remain() {
                    return Err(SizedBufferError::Write(size_of::<Self>(), buf.write_remain()));
                }
                buf.raw[buf.wpos..buf.wpos + size_of::<Self>()].copy_from_slice(&Self::to_le_bytes(*self));
                buf.stored(size_of::<Self>());
                Ok(size_of::<Self>())
            }

            fn pull_from(buf: &mut SizedBuffer) -> Result<Self, SizedBufferError> {
                if size_of::<Self>() > buf.read_remain() {
                    return Err(SizedBufferError::Read(size_of::<Self>(), buf.read_remain()));
                }
                let mut temp_buf = [0_u8; size_of::<Self>()];
                temp_buf.copy_from_slice(&buf.raw[buf.rpos..buf.rpos + size_of::<Self>()]);
                let result = Self::from_le_bytes(temp_buf);
                buf.visited(size_of::<Self>());
                Ok(result)
            }

            fn size_in_buffer(&self) -> usize {
                size_of::<Self>()
            }
        }
        )*
    }
}

bufferable_ints!(for u16, u32, u64, u128);

impl Bufferable for u8 {
    fn push_into(&self, buf: &mut SizedBuffer) -> Result<usize, SizedBufferError> {
        buf.raw[buf.wpos] = *self;
        buf.stored(size_of::<Self>());
        Ok(size_of::<Self>())
    }

    fn pull_from(buf: &mut SizedBuffer) -> Result<Self, SizedBufferError> {
        let result = buf.raw[buf.rpos];
        buf.visited(size_of::<Self>());
        Ok(result)
    }

    fn size_in_buffer(&self) -> usize {
        size_of::<Self>()
    }
}

impl Bufferable for bool {
    fn push_into(&self, buf: &mut SizedBuffer) -> Result<usize, SizedBufferError> {
        let as_byte = if *self {
            1u8
        } else {
            0
        };
        as_byte.push_into(buf)
    }

    fn pull_from(buf: &mut SizedBuffer) -> Result<Self, SizedBufferError> {
        let as_byte = u8::pull_from(buf)?;
        Ok(as_byte != 0)
    }

    fn size_in_buffer(&self) -> usize {
        size_of::<u8>()
    }
}

impl Bufferable for String {
    fn push_into(&self, buf: &mut SizedBuffer) -> Result<usize, SizedBufferError> {
        let bytes = self.as_bytes();
        let byte_len = bytes.len() as u8;
        byte_len.push_into(buf)?;
        let smaller = min(buf.write_remain(), byte_len as usize);
        buf.raw[buf.wpos..smaller + buf.wpos].copy_from_slice(&bytes[..smaller]);
        buf.stored(smaller);
        Ok(smaller)
    }

    fn pull_from(buf: &mut SizedBuffer) -> Result<Self, SizedBufferError> {
        let len = u8::pull_from(buf)? as usize;
        let mut slice = vec![0; len];
        slice.copy_from_slice(&buf.raw[buf.rpos..buf.rpos + len]);
        buf.visited(len);
        String::from_utf8(slice).map_err(SizedBufferError::Utf8Error)
    }

    fn size_in_buffer(&self) -> usize {
        size_of::<u8>() + self.len()
    }
}

impl<T: Bufferable> Bufferable for Vec<T> {
    fn push_into(&self, buf: &mut SizedBuffer) -> Result<usize, SizedBufferError> {
        let mut pushed = 0;
        pushed += (self.len() as u8).push_into(buf)?;
        for item in self {
            pushed += item.push_into(buf)?;
        }
        Ok(pushed)
    }

    fn pull_from(buf: &mut SizedBuffer) -> Result<Self, SizedBufferError> {
        let len = u8::pull_from(buf)? as usize;
        let mut vec = Vec::with_capacity(len);
        for _ in 0..len {
            let item = T::pull_from(buf)?;
            vec.push(item);
        }
        Ok(vec)
    }

    fn size_in_buffer(&self) -> usize {
        size_of::<u8>() + self.iter().map(|item| item.size_in_buffer()).sum::<usize>()
    }
}

impl<T: Bufferable + Default + Copy, const N: usize> Bufferable for [T; N] {
    fn push_into(&self, buf: &mut SizedBuffer) -> Result<usize, SizedBufferError> {
        let mut pushed = 0;
        for item in self {
            pushed += item.push_into(buf)?;
        }
        Ok(pushed)
    }

    fn pull_from(buf: &mut SizedBuffer) -> Result<Self, SizedBufferError> {
        let mut this = [T::default(); N];
        for item in &mut this {
            *item = T::pull_from(buf)?;
        }
        Ok(this)
    }

    fn size_in_buffer(&self) -> usize {
        self.iter().map(|item| item.size_in_buffer()).sum::<usize>()
    }
}

impl<T: Bufferable, U: Bufferable> Bufferable for (T, U) {
    fn push_into(&self, buf: &mut SizedBuffer) -> Result<usize, SizedBufferError> {
        let mut pushed = 0;
        pushed += self.0.push_into(buf)?;
        pushed += self.1.push_into(buf)?;
        Ok(pushed)
    }

    fn pull_from(buf: &mut SizedBuffer) -> Result<Self, SizedBufferError> {
        let first = T::pull_from(buf)?;
        let second = U::pull_from(buf)?;
        Ok((first, second))
    }

    fn size_in_buffer(&self) -> usize {
        self.0.size_in_buffer() + self.1.size_in_buffer()
    }
}

#[cfg(test)]
mod test {
    mod test_sized_buffer {
        use crate::sizedbuffers::{Bufferable, SizedBufferError};
        use crate::SizedBuffer;

        #[test]
        fn test_u8() -> Result<(), SizedBufferError> {
            let orig = 123_u8;
            let mut buf = SizedBuffer::from(&orig)?;
            let result = buf.pull::<u8>()?;

            assert_eq!(buf.size(), orig.size_in_buffer());
            assert_eq!(orig, result);
            Ok(())
        }

        #[test]
        fn test_u16() -> Result<(), SizedBufferError> {
            let orig = 1_234_u16;
            let mut buf = SizedBuffer::from(&orig)?;
            let result = buf.pull::<u16>()?;

            assert_eq!(buf.size(), orig.size_in_buffer());
            assert_eq!(orig, result);
            Ok(())
        }

        #[test]
        fn test_u64() -> Result<(), SizedBufferError> {
            let orig = 1_234_567_u64;
            let mut buf = SizedBuffer::from(&orig)?;
            let result = buf.pull::<u64>()?;

            assert_eq!(buf.size(), orig.size_in_buffer());
            assert_eq!(orig, result);
            Ok(())
        }

        #[test]
        fn test_u128() -> Result<(), SizedBufferError> {
            let orig = 1_234_567_890_u128;
            let mut buf = SizedBuffer::from(&orig)?;
            let result = buf.pull::<u128>()?;

            assert_eq!(buf.size(), orig.size_in_buffer());
            assert_eq!(orig, result);
            Ok(())
        }

        #[test]
        fn test_bytes() -> Result<(), SizedBufferError> {
            let orig = [1, 2, 3, 4, 5, 6, 7, 8, 9];
            let mut target = SizedBuffer::new(64);
            target.push_bytes(&orig)?;
            let result = target.pull_remaining()?;

            assert_eq!(target.size(), 9);
            assert_eq!(result, orig);
            Ok(())
        }

        #[test]
        fn test_buffer() -> Result<(), SizedBufferError> {
            let mut target = SizedBuffer::new(64);
            let mut source = SizedBuffer::new(64);
            target.push_bytes(&[1, 2, 3, 4])?;
            assert_eq!(target.size(), 4);

            source.push_bytes(&[5, 6, 7, 8, 9])?;
            assert_eq!(source.size(), 5);

            target.xfer_bytes(&mut source)?;
            assert_eq!(target.size(), 9);

            let result = target.pull_remaining()?;

            assert_eq!(result.len(), 9);
            assert_eq!(result, &[1, 2, 3, 4, 5, 6, 7, 8, 9]);
            Ok(())
        }

        #[test]
        fn test_bytes_n() -> Result<(), SizedBufferError> {
            let mut target = SizedBuffer::new(64);
            let mut source = SizedBuffer::new(64);
            target.push_bytes(&[1, 2, 3, 4])?;
            assert_eq!(target.size(), 4);

            source.push_bytes(&[5, 6, 7, 8, 9])?;
            assert_eq!(source.size(), 5);

            target.push_bytes(&source.pull_bytes_n(3)?)?;
            assert_eq!(source.read_remain(), 2);
            assert_eq!(target.size(), 7);

            let result = target.pull_remaining()?;

            assert_eq!(result.len(), 7);
            assert_eq!(result, &[1, 2, 3, 4, 5, 6, 7]);
            Ok(())
        }

        #[test]
        fn test_string() -> Result<(), SizedBufferError> {
            let mut source = SizedBuffer::new(64);
            let mut target = SizedBuffer::new(64);

            source.push(&"This is a test".to_string())?;
            let mut total_len = "This is a test".len() + 1;
            assert_eq!(source.size(), total_len);

            source.push(&String::from("So is this"))?;
            total_len += "So is this".len() + 1;
            assert_eq!(source.size(), total_len);

            let test1 = source.pull::<String>()?;
            assert_eq!("This is a test", test1);

            target.xfer::<String>(&mut source)?;

            let test2 = target.pull::<String>()?;
            assert_eq!("So is this", test2);
            Ok(())
        }

        #[test]
        fn test_vec() -> Result<(), SizedBufferError> {
            let orig = vec![0u32, 1, 2, 3, 4, 5, 6, 7, 8, 9];

            let mut buf = SizedBuffer::from(&orig)?;
            let result = Vec::<u32>::pull_from(&mut buf)?;

            assert_eq!(orig.len(), result.len());
            assert_eq!(orig, result);
            Ok(())
        }

        #[test]
        fn test_array() -> Result<(), SizedBufferError> {
            type TestArray = [u32; 10];
            let orig: TestArray = [0u32, 1, 2, 3, 4, 5, 6, 7, 8, 9];

            let mut buf = SizedBuffer::from(&orig)?;
            let result = TestArray::pull_from(&mut buf)?;

            assert_eq!(orig.len(), result.len());
            assert_eq!(orig, result);
            Ok(())
        }

        #[test]
        fn test_empty_array() -> Result<(), SizedBufferError> {
            type TestArray = [u32; 0];
            let orig: TestArray = [];

            let mut buf = SizedBuffer::from(&orig)?;
            let result = TestArray::pull_from(&mut buf)?;

            assert_eq!(orig.len(), result.len());
            assert_eq!(orig, result);
            Ok(())
        }

        #[test]
        fn test_array_array() -> Result<(), SizedBufferError> {
            type TestArray = [u32; 10];
            type TestArrayArray = [TestArray; 3];
            let orig_item: TestArray = [0u32, 1, 2, 3, 4, 5, 6, 7, 8, 9];
            let orig: TestArrayArray = [orig_item, orig_item, orig_item];

            let mut buf = SizedBuffer::from(&orig)?;
            let result = TestArrayArray::pull_from(&mut buf)?;

            assert_eq!(orig.len(), result.len());
            assert_eq!(orig, result);

            Ok(())
        }

        #[test]
        fn test_tuple() -> Result<(), SizedBufferError> {
            type TestTuple = (u128, u8);
            let orig: TestTuple = (128, 8);

            let mut buf = SizedBuffer::from(&orig)?;
            let result = TestTuple::pull_from(&mut buf)?;

            assert_eq!(orig, result);
            Ok(())
        }

        #[test]
        fn test_tuple_arrays() -> Result<(), SizedBufferError> {
            type TestTuple = ([u128; 4], Vec<bool>);
            let orig: TestTuple = ([128, 8, 0, 8172637813], vec![true, false, true, true, false, true]);

            let mut buf = SizedBuffer::from(&orig)?;
            let result = TestTuple::pull_from(&mut buf)?;

            assert_eq!(orig.0.len(), result.0.len());
            assert_eq!(orig.1.len(), result.1.len());
            assert_eq!(orig, result);
            Ok(())
        }
    }
}
