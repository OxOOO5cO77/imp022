use std::io::Error;

use tokio::io::{AsyncWrite, AsyncWriteExt};

use crate::VSizedBuffer;

pub(crate) async fn write_buf<T>(stream: &mut T, buf: &VSizedBuffer) -> Result<usize, Error> where T: Unpin + AsyncWrite {
    let len = buf.size() + VSizedBuffer::sizesize();
    let amt = stream.write(&buf.raw[..len]).await?;
    Ok(amt)
}

pub const fn split_u128(src: u128) -> (u64, u64) {
    const MASK: u128 = u64::MAX as u128;
    let hi = (src >> 64) as u64;
    let lo = (src & MASK) as u64;

    (hi, lo)
}

pub const fn join_u128(hi: u64, lo: u64) -> u128 {
    let hi = hi as u128;
    let lo = lo as u128;
    (hi << 64) | lo
}


#[cfg(test)]
mod test {
    mod write_buf {
        use std::io::Cursor;

        use crate::util::write_buf;
        use crate::VSizedBuffer;

        #[tokio::test]
        async fn write_normal() {
            let mut tester = Cursor::new(Vec::new());
            let mut buf = VSizedBuffer::new(64);
            let test = [1, 2, 3, 4, 5, 6, 7, 8];
            buf.push_bytes(&test);

            let result = write_buf(&mut tester, &buf).await;

            match result {
                Ok(bytes) => {
                    assert_eq!(bytes, test.len() + VSizedBuffer::sizesize());

                    let result = tester.into_inner();

                    assert_eq!(result[VSizedBuffer::sizesize()..], [1, 2, 3, 4, 5, 6, 7, 8]);  // size + data
                }
                Err(err) => {
                    std::panic::panic_any(err)
                }
            }
        }
    }

    mod split_join {
        use crate::util::{join_u128, split_u128};

        #[test]
        fn split_join() {
            let test_in = 0x1234_5678_90AB_CDEF_FEDC_BA09_8765_4321;
            let (hi, lo) = split_u128(test_in);
            let test_out = join_u128(hi, lo);

            assert_eq!(test_in, test_out);
        }
    }
}