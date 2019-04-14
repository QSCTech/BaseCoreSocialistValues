#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[macro_use]
extern crate lazy_static;

use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::io::{self, ErrorKind, Read, Write};

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
    static ref BYTE_SET: HashSet<&'static u8> = {
        let mut set = HashSet::new();
        for word in WORD_SET.iter() {
            for item in word.as_bytes().iter() {
                set.insert(item);
            }
        }
        set
    };
}

fn detect_order(str: &str) -> u8 {
    let index1 = WORD_MAP.get(&str[0..6]).unwrap() / 4;
    let index2 = WORD_MAP.get(&str[6..12]).unwrap() / 4;
    let index3 = WORD_MAP.get(&str[12..18]).unwrap() / 4;

    if index1 == 1 && index2 == 0 && index3 == 2 {
        1
    } else if index1 == 1 && index2 == 2 && index3 == 0 {
        2
    } else if index1 == 2 && index2 == 1 && index3 == 0 {
        3
    } else if index1 == 0 && index2 == 1 && index3 == 2 {
        0
    } else {
        unreachable!()
    }
}

struct Char<'a> {
    words: [&'a str; 3],
    order: u8,
}

impl<'a> Char<'a> {
    fn new(byte: u8) -> Self {
        Self {
            order: byte & 0b11,
            words: [
                WORD_SET[((byte >> 2) & 0b11) as usize],
                WORD_SET[(4 + ((byte >> 4) & 0b11)) as usize],
                WORD_SET[(8 + ((byte >> 6) & 0b11)) as usize],
            ],
        }
    }

    fn encode_into(self, mut writer: impl Write) -> io::Result<usize> {
        match self.order {
            0 => {
                for word in self.words.iter() {
                    writer.write_all(word.as_bytes())?;
                }
            }
            1 => {
                writer.write_all(self.words[1].as_bytes())?;
                writer.write_all(self.words[0].as_bytes())?;
                writer.write_all(self.words[2].as_bytes())?;
            }
            2 => {
                writer.write_all(self.words[1].as_bytes())?;
                writer.write_all(self.words[2].as_bytes())?;
                writer.write_all(self.words[0].as_bytes())?;
            }
            3 => {
                writer.write_all(self.words[2].as_bytes())?;
                writer.write_all(self.words[1].as_bytes())?;
                writer.write_all(self.words[0].as_bytes())?;
            }
            _ => unreachable!(),
        }
        Ok(24)
    }

    fn decode_into(&self, mut writer: impl Write) -> io::Result<usize> {
        let byte = self.order
            + ((WORD_MAP.get(self.words[0]).unwrap() % 4) << 2) as u8
            + ((WORD_MAP.get(self.words[1]).unwrap() % 4) << 4) as u8
            + ((WORD_MAP.get(self.words[2]).unwrap() % 4) << 6) as u8;
        writer.write_all(&[byte])?;
        Ok(1)
    }
}

impl<'a> TryFrom<&'a [u8; 18]> for Char<'a> {
    type Error = io::Error;
    fn try_from(bytes: &'a [u8; 18]) -> Result<Self, Self::Error> {
        let str = std::str::from_utf8(bytes).unwrap();
        let order = detect_order(&str);
        Ok(match order {
            0 => Self {
                order: detect_order(&str),
                words: [&str[0..6], &str[6..12], &str[12..18]],
            },
            1 => Self {
                order: detect_order(&str),
                words: [&str[6..12], &str[0..6], &str[12..18]],
            },
            2 => Self {
                order: detect_order(&str),
                words: [&str[12..18], &str[0..6], &str[6..12]],
            },
            3 => Self {
                order: detect_order(&str),
                words: [&str[12..18], &str[6..12], &str[0..6]],
            },
            _ => Err(io::Error::new(
                ErrorKind::InvalidInput,
                "invalid input data",
            ))?,
        })
    }
}

struct Buffer {
    inner: Vec<u8>,
}

impl Read for Buffer {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let size = self.inner.len();
        if size <= buf.len() {
            buf[..size].copy_from_slice(self.inner.as_slice());
            self.inner = Vec::new();
            Ok(size)
        } else {
            buf.copy_from_slice(&self.inner[..buf.len()]);
            self.inner.drain(..buf.len());
            Ok(buf.len())
        }
    }
}

impl Write for Buffer {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Buffer {
    fn new() -> Self {
        Self { inner: Vec::new() }
    }

    fn len(&self) -> usize {
        self.inner.len()
    }
}

/// The Encoder
///
/// # Example
///
/// ```ignore
/// use base_core_socialist_values::Encoder;
/// use std::io::{self, stdin, stdout};
///
/// fn main() -> io::Result<()> {
///     let mut reader = stdin();
///     let mut writer = stdout();
///     io::copy(&mut reader, &mut Encoder::new(&mut writer))?;
///     Ok(())
/// }
/// ```
pub struct Encoder<W: Write> {
    writer: W,
}

/// The Decoder
///
/// # Example
///
/// ```ignore
/// use base_core_socialist_values::Decoder;
/// use std::io::{self, stdin, stdout};
///
/// fn main() -> io::Result<()> {
///     let mut reader = stdin();
///     let mut writer = stdout();
///     io::copy(&mut reader, &mut Decoder::new(&mut writer))?;
///     Ok(())
/// }
/// ```
pub struct Decoder<W: Write> {
    input_buf: Buffer,
    writer: W,
}

impl<W: Write> Encoder<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }
    pub fn get_writer(self) -> W {
        self.writer
    }
}

impl<W: Write> Decoder<W> {
    pub fn new(writer: W) -> Self {
        Self {
            input_buf: Buffer::new(),
            writer,
        }
    }
    pub fn get_writer(self) -> W {
        self.writer
    }
}

impl<W: Write> Write for Encoder<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        for byte in buf {
            Char::new(*byte).encode_into(&mut self.writer)?;
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

impl<W: Write> Write for Decoder<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut all_bytes = 0;
        let mut start_point = 0;

        for i in 0..buf.len() {
            if BYTE_SET.contains(&buf[i]) {
                start_point = i;
                break;
            } else {
                if i==buf.len() - 1 {
                    start_point = buf.len();
                }
            }
        }

        all_bytes += start_point;
        {
            let mut i = start_point;
            'outer: loop {
                if i == buf.len() {
                    break;
                }
                if !BYTE_SET.contains(&buf[i]) {
                    if start_point < i {
                        let bytes = self.input_buf.write(&buf[start_point..i])?;
                        all_bytes += bytes;
                        if bytes < i - start_point {
                            break;
                        }

                        for pos in i+1..buf.len() {
                            if BYTE_SET.contains(&buf[pos]) {
                                i = pos;
                                all_bytes += pos - start_point;
                                start_point = pos;
                                continue 'outer;
                            } else {
                                if pos == buf.len() - 1 {
                                    all_bytes = buf.len();
                                    break 'outer;
                                }
                            }
                        }
                    }
                }
                i += 1;
            }
        }

        while self.input_buf.len() >= 18 {
            let mut bytes = [0; 18];
            self.input_buf.read_exact(&mut bytes)?;
            Char::try_from(&bytes)?.decode_into(&mut self.writer)?;
        }
        Ok(all_bytes)
    }

    fn flush(&mut self) -> io::Result<()> {
        if self.input_buf.len() != 0 {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "input buffer doesn't be flushed",
            ));
        }
        self.writer.flush()
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn encode(str: &str) -> String {
    let mut encoder = Encoder::new(Vec::new());
    encoder.write_all(str.as_bytes()).unwrap();

    String::from_utf8(encoder.get_writer()).unwrap()
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn decode(str: &str) -> String {
    let mut decoder = Decoder::new(Vec::new());
    decoder.write_all(str.as_bytes()).unwrap();

    String::from_utf8(decoder.get_writer()).unwrap()
}
