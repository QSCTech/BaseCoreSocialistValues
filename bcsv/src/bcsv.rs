use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::thread;

pub(crate) fn process<R: Read, W: Write, Processor: Read + Write>(
    reader: &mut BufReader<R>,
    writer: &mut BufWriter<W>,
    processor: &mut Processor,
    input_buffer_size: usize,
    output_buffer_size: usize,
) {
    let mut input_buffer = vec![0; input_buffer_size];
    let input_buffer = &mut input_buffer[..];
    let mut output_buffer = vec![0; output_buffer_size];
    let output_buffer = &mut output_buffer[..];
    while let n = reader.read(input_buffer).unwrap() {
        if n == 0 {
            break;
        }
        processor.write_all(&input_buffer[0..n]).unwrap();
        let size = processor.read(output_buffer).unwrap();

        writer.write_all(&output_buffer[0..size]).unwrap();
    }
    writer.flush();
}

#[cfg(test)]
mod test {
    extern crate base_core_socialist_values;

    use super::*;
    use base_core_socialist_values::Buffer;
    use std::io::{BufReader, BufWriter};

    #[test]
    fn process_by_vec() {
        let mut input = vec![1u8, 2, 3, 4, 5];
        let mut reader = BufReader::new(&input[..]);
        let mut writer = BufWriter::new(vec![0; 0]);

        let mut processor = Buffer::new();

        process(&mut reader, &mut writer, &mut processor, 5, 5);

        assert_eq!(writer.get_ref(), &vec![1u8, 2, 3, 4, 5]);
    }
}
