use std::io::Error;

use tokio::io::{AsyncWrite, AsyncWriteExt};

use crate::VSizedBuffer;

pub(crate) async fn write_buf<T>(stream: &mut T, buf: &VSizedBuffer) -> Result<usize, Error>
where
    T: Unpin + AsyncWrite,
{
    let len = buf.size() + VSizedBuffer::sizesize();
    let amt = stream.write(&buf.raw[..len]).await?;
    Ok(amt)
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

                    assert_eq!(result[VSizedBuffer::sizesize()..], [1, 2, 3, 4, 5, 6, 7, 8]);
                    // size + data
                }
                Err(err) => std::panic::panic_any(err),
            }
        }
    }
}
