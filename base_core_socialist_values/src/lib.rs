use std::io::{self, Read, Write};

struct Buffer {
    inner: Vec<u8>,
}

impl Read for Buffer {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let size = self.inner.len();
        let cap = self.inner.capacity();
        if size <= buf.len() {
            buf[..size].copy_from_slice(self.inner.as_slice());
            self.inner = Vec::new();
            Ok(size)
        } else {
            buf.copy_from_slice(&self.inner[..buf.len()]);
            let header = Box::into_raw(self.inner.into_boxed_slice());
            let new_header = unsafe { header.offset(buf.len() as isize) };
            unsafe { drop(Vec::from_raw_parts(header, buf.len(), buf.len())) };
            self.inner =
                unsafe { Vec::from_raw_parts(new_header, size - buf.len(), cap - buf.len()) };
            Ok(buf.len())
        }
    }
}

impl Write for Buffer {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let space = self.inner.capacity() - self.inner.len();
        if space < buf.len() {
            self.inner.reserve(buf.len() - space);
        }
        unsafe {
            self.inner.set_len(self.inner.len() + buf.len());
        }
        self.inner.as_mut_slice()[..buf.len()].copy_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub struct Encoder {
    input_buf: Vec<u8>,
    output_data: Vec<u8>,
}

pub struct Decoder {}

impl Write for Encoder {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        unimplemented!()
    }

    fn flush(&mut self) -> io::Result<()> {
        unimplemented!()
    }
}

impl Read for Encoder {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        unimplemented!()
    }
}
