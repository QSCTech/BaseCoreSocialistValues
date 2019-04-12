extern crate clap;
use clap::{App, Arg};
use std::io::{BufReader, BufWriter, Read};

mod bcsv;
use bcsv::process;

const ENC_BLOCK_SIZE: usize = 1024 * 3 * 10;
const DEC_BLOCK_SIZE: usize = 1024 * 5;

const fn BCSV_LEN(len: usize) -> usize {
    return len * 18;
}

fn main() {
    let matches = App::new("BaseCoreSocialistValues")
        .version("0.1.0")
        .author("YangKeao keao.yang@yahoo.com")
        .arg(
            Arg::with_name("decode")
                .short("d")
                .long("decode")
                .help("decode data"),
        )
        .arg(Arg::with_name("FILE").required(true))
        .about(
            r#"
BaseCoreSocialistValues encode or decode FILE, or standard input, to standard output.

With no FILE, or when FILE is -, read standard input.
                          "#,
        )
        .get_matches();

    let file_name = matches.value_of("FILE").unwrap();

    let reader: Box<Read> = if file_name == "-" {
        Box::new(std::io::stdin())
    } else {
        match std::fs::File::open(file_name) {
            Ok(file) => Box::new(file),
            Err(err) => {
                eprintln!("{}", err);
                std::process::exit(1)
            }
        }
    };
    let reader = BufReader::new(reader);
    let writer = BufWriter::new(std::io::stdout());

    if matches.is_present("decode") {
        //        process(reader, writer, unimplemented!(), input_buffer, output_buffer);
    } else {
        //       process(reader, writer, unimplemented!(), input_buffer, output_buffer);
    }
}
