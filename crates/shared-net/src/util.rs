use std::io::Error;

use tokio::io::{AsyncWrite, AsyncWriteExt};

use crate::SizedBuffer;

pub(crate) async fn write_buf<T>(stream: &mut T, buf: &SizedBuffer) -> Result<usize, Error>
where
    T: Unpin + AsyncWrite,
{
    let len = buf.size() + SizedBuffer::sizesize();
    let amt = stream.write(&buf.raw[..len]).await?;
    Ok(amt)
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use super::write_buf;
    use crate::SizedBuffer;

    #[tokio::test]
    async fn write_normal() {
        let mut tester = Cursor::new(Vec::new());
        let mut buf = SizedBuffer::new(64);
        let test = [1, 2, 3, 4, 5, 6, 7, 8];
        if let Err(err) = buf.push_bytes(&test) {
            std::panic::panic_any(err)
        }

        match write_buf(&mut tester, &buf).await {
            Ok(bytes) => {
                assert_eq!(bytes, test.len() + SizedBuffer::sizesize());

                let result = tester.into_inner();

                assert_eq!(result[SizedBuffer::sizesize()..], [1, 2, 3, 4, 5, 6, 7, 8]);
                // size + data
            }
            Err(err) => std::panic::panic_any(err),
        }
    }
}
