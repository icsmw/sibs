use std::collections::HashMap;

use crate::*;

#[derive(Debug, Default)]
pub struct Errors {
    errors: HashMap<String, DrivingError>,
}

impl Errors {
    pub fn insert(&mut self, err: DrivingError) {
        let stamp = err.stamp();
        if !self.errors.contains_key(&stamp) {
            self.errors.insert(stamp, err);
        }
    }
    pub fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = DrivingError>,
    {
        for err in iter {
            self.insert(err);
        }
    }
}

#[derive(Debug)]
pub enum DrivingError {
    Parsing(LinkedErr<ParserError>),
    Semantic(LinkedErr<SemanticError>),
}

impl ErrorCode for DrivingError {
    fn code(&self) -> &'static str {
        match self {
            Self::Parsing(err) => err.e.code(),
            Self::Semantic(err) => err.e.code(),
        }
    }
    fn src(&self) -> ErrorSource {
        match self {
            Self::Parsing(err) => err.e.src(),
            Self::Semantic(err) => err.e.src(),
        }
    }
}

impl fmt::Display for DrivingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DrivingError::Parsing(err) => write!(f, "{}", err.e),
            DrivingError::Semantic(err) => write!(f, "{}", err.e),
        }
    }
}

impl DrivingError {
    pub fn link(&self) -> &LinkedPosition {
        match self {
            Self::Parsing(err) => &err.link,
            Self::Semantic(err) => &err.link,
        }
    }
    pub fn stamp(&self) -> String {
        match self {
            Self::Parsing(err) => format!(
                "{}:{}:{}:{}",
                err.e.code(),
                err.link.src,
                err.link.from.abs,
                err.link.to.abs
            ),
            Self::Semantic(err) => format!(
                "{}:{}:{}:{}",
                err.e.code(),
                err.link.src,
                err.link.from.abs,
                err.link.to.abs
            ),
        }
    }
}

pub struct ErrorsIterator<'a> {
    errors: Vec<&'a DrivingError>,
    anchor: &'a Anchor,
    parser: &'a Parser,
    index: usize,
}

impl<'a> ErrorsIterator<'a> {
    pub fn new(errors: Vec<&'a DrivingError>, anchor: &'a Anchor, parser: &'a Parser) -> Self {
        Self {
            errors,
            anchor,
            parser,
            index: 0,
        }
    }
}

pub struct ErrorLocator<'a> {
    pub err: &'a DrivingError,
    pub locator: LocationIterator<'a>,
}

impl<'a> ErrorLocator<'a> {
    pub fn new(err: &'a DrivingError, locator: LocationIterator<'a>) -> Self {
        Self { err, locator }
    }
}

impl<'a> Iterator for ErrorsIterator<'a> {
    type Item = ErrorLocator<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(err) = self.errors.get(self.index) else {
            return None;
        };
        self.index += 1;
        let link = err.link();
        Some(ErrorLocator::new(
            err,
            LocationIterator::new(self.anchor, link.src.clone(), link.from.abs, &self.parser),
        ))
    }
}

#[test]
fn test() {
    let mut driver = Driver::unbound(
        r#"/// This is description component_a
component component_aaa() {
    /// This description is task_a
    task task_aaa() {
        let my_string = "fdsfsdfsd";
        let aaa: num = 5;
        let b: bool = true;
        let b: num = 411123233232;
        let vvv = my_string;
        let ttt: str = "asasjkdsa";
        
        let command = `some{ inject }command`;
        let c = 'qwhjerkslft{ ttt }ofpsdfgfreddh{ bbb }fdsfsd{ if a == 4 { "fdfsdf"; } else { "dfsd"; } }';
        aaa.fns::sum(aaa);
        if aaa == 5 && ddd == 5 && ddd != 6 {
            return true;
        } else {
            return false;
        }
    }
};"#,
        true,
    );
    driver.read().unwrap_or_else(|err| panic!("{err}"));
    let errors = driver.errors().unwrap();
    for err in errors {
        println!("{:?}", err.err);
    }
}
