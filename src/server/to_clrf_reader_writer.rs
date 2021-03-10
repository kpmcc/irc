use std::io::{Read, Result, Write};

// Size of the internal buffer. The larger this is, the fewer calls to the
// source's `read` method need to be made.
const BUF_SIZE: usize = 1024;

/// A wrapper around a `source` implementing `Read` and `Write`. The `Read` implementation reads a
/// message up to \r\n, while the `Write` implementation delegates directly to the wrapped source.
pub struct ToClrfReaderWriter<'a, T: Read + Write> {
    // constraint: last_written >= last_read
    buf: [u8; BUF_SIZE],
    source: &'a mut T,
    read_next: usize,
    last_written: usize,
}

impl<T: Read + Write> ToClrfReaderWriter<'_, T> {
    pub fn new(source: &mut T) -> ToClrfReaderWriter<T> {
        ToClrfReaderWriter {
            buf: [0; BUF_SIZE],
            source,
            read_next: 0,
            last_written: 0,
        }
    }
}

impl<T: Read + Write> Read for ToClrfReaderWriter<'_, T> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let mut i: usize = 0;
        let mut cr_seen = false;
        loop {
            while self.read_next < BUF_SIZE && self.read_next < self.last_written {
                let b = self.buf[self.read_next];
                self.read_next += 1;
                buf[i] = b;
                if b == b'\n' && cr_seen {
                    return Ok(i + 1);
                }
                cr_seen = b == b'\r';
                i += 1;
            }

            // Nothing useful left in the buffer now.
            self.read_next = 0;
            self.last_written = self.source.read(&mut self.buf)?;
        }
    }
}

impl<T: Read + Write> Write for ToClrfReaderWriter<'_, T> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.source.write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.source.flush()
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Read, Result, Write};

    struct TestReadWrite {
        data: Vec<Vec<u8>>,
        index: usize,
    }

    impl TestReadWrite {
        pub fn new(data: Vec<Vec<u8>>) -> TestReadWrite {
            TestReadWrite { data, index: 0 }
        }
    }

    impl Read for TestReadWrite {
        fn read(&mut self, mut buf: &mut [u8]) -> Result<usize> {
            let result = buf.write(&self.data[self.index]);
            self.index += 1;
            result
        }
    }

    /// No-op implementation
    impl Write for TestReadWrite {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(0)
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    #[test]
    fn it_works() {
        let mut reader = TestReadWrite::new(vec![b"foo\r\nba".to_vec(), b"r\r\nbaz\r\nq".to_vec()]);
        let mut cb = super::ToClrfReaderWriter::new(&mut reader);
        let mut dst = [0; 5];
        let expected_arr = vec![b"foo\r\n", b"bar\r\n", b"baz\r\n"];
        for expected in expected_arr.iter() {
            cb.read(&mut dst).unwrap();
            assert_eq!(dst.len(), expected.len());
            for (i, (a, b)) in dst.iter().zip(expected.iter()).enumerate() {
                assert!(a == b, format!("Position {}: {} != {}", i, a, b));
            }
        }
    }
}
