mod imp;
mod scheme;
mod themes;

use mottle::theme::{ThemeBuilder, Type};
use std::io;

fn main() -> io::Result<()> {
    gen_theme("Noel", themes::Noel)?;
    gen_theme("Yuru", themes::Yuru)?;

    Ok(())
}

fn gen_theme(name: &str, scheme: impl scheme::Scheme) -> io::Result<()> {
    let mut builder = ThemeBuilder::new(name.to_string(), Type::Dark);
    imp::add_rules(&mut builder, &scheme);
    builder.build().save()?;

    Ok(())
}
