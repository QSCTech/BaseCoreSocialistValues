extern crate base_core_socialist_values;
extern crate clap;
use base_core_socialist_values::{Decoder, Encoder};
use clap::{App, Arg};
use std::io::{copy, BufReader, BufWriter, Read, Write};

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
        .arg(Arg::with_name("FILE"))
        .about(
            r#"
BaseCoreSocialistValues encode or decode FILE, or standard input, to standard output.

With no FILE, or when FILE is -, read standard input.
                          "#,
        )
        .get_matches();

    let file_name = match matches.value_of("FILE") {
        Some(filename) => filename,
        None => "-",
    };

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
    let mut reader = BufReader::new(reader);
    let mut writer = BufWriter::new(std::io::stdout());

    if matches.is_present("decode") {
        copy(&mut reader, &mut Decoder::new(&mut writer)).unwrap();
    } else {
        copy(&mut reader, &mut Encoder::new(&mut writer)).unwrap();
    }
    writer.flush().unwrap();
}
