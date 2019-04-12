#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::mem::replace;

const WORD_SET: [&'static str; 12] = [
    "富强", "民主", "文明", "和谐", "自由", "平等", "公正", "法治", "爱国",
    "敬业", "诚信", "友善",
];

lazy_static! {
    static ref WORD_MAP: HashMap<&'static str, usize> = {
        let mut map = HashMap::new();
        for (index, word) in WORD_SET.iter().enumerate() {
            map.insert(*word, index);
        }
        map
    };
}

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
            let header =
                Box::into_raw(replace(&mut self.inner, Vec::new()).into_boxed_slice()) as *mut u8;
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
    output_data: Buffer,
}

pub struct Decoder {
    input_buf: Buffer,
    output_data: Buffer,
}

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
