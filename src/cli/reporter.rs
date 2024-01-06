use ansi_term::Color;

pub fn associated(reporter: &str, msg: &str) {
    println!(
        "{}: {}",
        Color::White.bold().paint(reporter),
        Color::Cyan.paint(msg)
    );
}
