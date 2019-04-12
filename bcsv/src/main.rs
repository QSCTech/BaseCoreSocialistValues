extern crate clap;
use clap::{Arg, App};
use std::io::{BufWriter, Read, BufReader};

mod bcsv;
use bcsv::{encode, decode};

fn main() {
    let matches = App::new("BaseCoreSocialistValues")
                    .version("0.1.0")
                    .author("YangKeao keao.yang@yahoo.com")
                    .arg(Arg::with_name("decode")
                         .short("d")
                         .long("decode")
                         .help("decode data"))
                    .arg(Arg::with_name("FILE")
                         .required(true))
                    .about(r#"
BaseCoreSocialistValues encode or decode FILE, or standard input, to standard output.

With no FILE, or when FILE is -, read standard input.
                          "#)
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
        decode(reader, writer)
    } else {
        encode(reader, writer)
    }
}
