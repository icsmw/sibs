use crate::*;

#[derive(Debug)]
pub enum DrivingError {
    Parsing(LinkedErr<ParserError>),
    Semantic(LinkedErr<SemanticError>),
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
