use ansi_term::Color;
pub struct Description {
    pub key: Vec<String>,
    pub desc: String,
}

pub fn associated(reporter: &str, msg: &str) {
    println!(
        "{}: {}",
        Color::White.bold().paint(reporter),
        Color::Cyan.paint(msg)
    );
}

pub fn desc(desc: Vec<Description>) {
    let max = desc
        .iter()
        .map(|d| d.key.join(", ").len())
        .max()
        .unwrap_or_default();
    desc.iter().for_each(|desc| {
        let keys = desc.key.join(", ");
        println!(
            "{}{} - {}",
            Color::White.bold().paint(&keys),
            " ".repeat(max - keys.len()),
            Color::Cyan.paint(&desc.desc)
        );
    });
}
