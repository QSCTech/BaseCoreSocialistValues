use base_core_socialist_values::Decoder;
use std::io::{self, stdin, stdout};
fn main() -> io::Result<()> {
    let mut reader = stdin();
    let mut writer = stdout();
    io::copy(&mut reader, &mut Decoder::new(&mut writer))?;
    Ok(())
}
