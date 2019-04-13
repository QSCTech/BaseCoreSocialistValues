use base_core_socialist_values::Encoder;
use std::io::{self, stdin, stdout};

fn main() -> io::Result<()> {
    let mut reader = stdin();
    let mut writer = stdout();
    io::copy(&mut reader, &mut Encoder::new(&mut writer))?;
    Ok(())
}
