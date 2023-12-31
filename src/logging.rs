use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use anyhow::Result;

fn writer(message: &str, prompt: &str, color: ColorSpec) -> Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(&color)?;
    write!(&mut stdout, "{prompt}: ")?;
    WriteColor::reset(&mut stdout)?;
    writeln!(&mut stdout, "{message}")?;
    Ok(())
}

fn create_color_spec(color: Color) -> ColorSpec {
    let colorspec = ColorSpec::new().set_fg(Some(color)).to_owned();
    return colorspec;
}

pub fn info(message: &str) {
    writer(message, "INFO", create_color_spec(Color::Rgb(43, 48, 71))).unwrap();
}

pub fn warn(message: &str) {
    writer(message, "WARNING", create_color_spec(Color::Yellow)).unwrap();
}

pub fn error(message: &str) {
    writer(message, "ERROR", create_color_spec(Color::Red)).unwrap();
}
