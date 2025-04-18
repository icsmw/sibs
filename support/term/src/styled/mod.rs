use console::strip_ansi_codes;

mod bold;
mod color;
mod ordered;

pub trait Styled {
    fn apply(&mut self, str: &str) -> String;
    fn finalize(&mut self, str: &str) -> String {
        str.to_string()
    }
}

pub fn striped_len(str: &str) -> usize {
    strip_ansi_codes(str).len()
}

pub fn apply<T: AsRef<str>>(width: usize, str: T) -> String {
    let mut elements: Vec<Box<dyn Styled>> = vec![
        Box::new(bold::Bold::new(width)),
        Box::new(color::Color::new(width)),
        Box::new(ordered::Ordered::new(width)),
    ];
    let lines = str
        .as_ref()
        .split('\n')
        .map(|s| {
            let mut out = s.to_owned();
            elements
                .iter_mut()
                .for_each(|styled| out = styled.apply(&out));
            out
        })
        .collect::<Vec<String>>();
    lines
        .iter()
        .map(|s| {
            let mut out = s.to_owned();
            elements
                .iter_mut()
                .for_each(|styled| out = styled.finalize(&out));
            out
        })
        .collect::<Vec<String>>()
        .join("\n")
}
