use std::{
    collections::HashMap,
    fs, io,
    path::{Path, PathBuf},
};
use uuid::Uuid;

#[derive(Debug)]
pub enum CodeSource {
    Inline(String),
    File(PathBuf),
}

impl CodeSource {
    pub fn content(&self) -> Result<String, io::Error> {
        Ok(match self {
            Self::Inline(c) => c.to_owned(),
            Self::File(filename) => fs::read_to_string(filename)?,
        })
    }
    pub fn sig(&self) -> Option<String> {
        match self {
            Self::Inline(_) => None,
            Self::File(filename) => Some(filename.to_string_lossy().to_string()),
        }
    }
}

#[derive(Debug, Default)]
pub struct CodeSources {
    pub sources: HashMap<Uuid, CodeSource>,
}

impl CodeSources {
    pub fn bound<P: AsRef<Path>>(filename: P, uuid: &Uuid) -> Result<Self, io::Error> {
        let mut sources = HashMap::new();
        let filename = std::fs::canonicalize(filename)?;
        sources.insert(*uuid, CodeSource::File(filename));
        Ok(Self { sources })
    }
    pub fn unbound<S: AsRef<str>>(content: S, uuid: &Uuid) -> Self {
        let mut sources = HashMap::new();
        sources.insert(*uuid, CodeSource::Inline(content.as_ref().to_owned()));
        Self { sources }
    }
    pub fn get_content(&self, src: &Uuid) -> Result<Option<String>, io::Error> {
        let Some(source) = self.sources.get(src) else {
            return Ok(None);
        };
        Ok(Some(source.content()?))
    }
    pub fn get_source(&self, src: &Uuid) -> Option<&CodeSource> {
        self.sources.get(src)
    }
    pub fn add_file_src<P: AsRef<Path>>(
        &mut self,
        filename: P,
        uuid: &Uuid,
    ) -> Result<(), io::Error> {
        let filename = std::fs::canonicalize(filename)?;
        if self.sources.iter().any(|(_, cs)| {
            if let CodeSource::File(path) = cs {
                path == &filename
            } else {
                false
            }
        }) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                    "File \"{}\" already has been attached",
                    filename.to_string_lossy()
                ),
            ));
        }
        self.sources.insert(*uuid, CodeSource::File(filename));
        Ok(())
    }
    pub fn add_inline_src<S: AsRef<str>>(&mut self, content: S, uuid: &Uuid) {
        self.sources
            .insert(*uuid, CodeSource::Inline(content.as_ref().to_owned()));
    }
}
