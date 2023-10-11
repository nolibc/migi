use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub fn info(message: &str) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(43, 48, 71)))).unwrap();

    write!(&mut stdout, "INFO: ").unwrap();
    WriteColor::reset(&mut stdout).unwrap();
    writeln!(&mut stdout, "{message}").unwrap();
}

pub fn warn(message: &str) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow))).unwrap();

    write!(&mut stdout, "WARNING: ").unwrap();
    WriteColor::reset(&mut stdout).unwrap();
    writeln!(&mut stdout, "{message}").unwrap();
}

pub fn error(message: &str) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red))).unwrap();

    write!(&mut stdout, "ERROR: ").unwrap();
    WriteColor::reset(&mut stdout).unwrap();
    writeln!(&mut stdout, "{message}").unwrap();
}
