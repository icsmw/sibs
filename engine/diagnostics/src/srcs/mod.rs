mod context;
mod error;
#[cfg(test)]
mod tests;

pub use context::*;
pub use error::*;

use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};
use uuid::Uuid;

#[derive(Debug)]
pub enum CodeSource {
    Inline(String),
    File(PathBuf),
}

impl CodeSource {
    pub fn content(&self) -> Result<String, CodeSourceError> {
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
    sources: HashMap<Uuid, CodeSource>,
    files: HashMap<PathBuf, Uuid>,
}

impl CodeSources {
    pub fn bound<P: AsRef<Path>>(filename: P, uuid: &Uuid) -> Result<Self, CodeSourceError> {
        let mut sources = Self::default();
        sources.add_file_src(filename.as_ref().canonicalize()?, uuid)?;
        Ok(sources)
    }
    pub fn unbound<S: AsRef<str>>(content: S, uuid: &Uuid) -> Self {
        let mut sources = Self::default();
        sources.add_inline_src(content, uuid);
        sources
    }

    pub fn enter_file<P: AsRef<Path>>(
        &mut self,
        filename: P,
        parent: Option<&CodeSourceContext>,
    ) -> Result<CodeSourceContext, CodeSourceError> {
        let path = filename.as_ref().canonicalize()?;
        let source = if let Some(source) = self.files.get(&path) {
            *source
        } else {
            let source = Uuid::new_v4();
            self.add_file_src(path.clone(), &source)?;
            source
        };
        let mut ancestry = parent.map(|p| p.ancestry.clone()).unwrap_or_default();
        if ancestry.contains(&source) {
            ancestry.push(source);
            return Err(CodeSourceError::ImportCycle { path, ancestry });
        }
        ancestry.push(source);
        Ok(CodeSourceContext { source, ancestry })
    }
    pub fn get_content(&self, src: &Uuid) -> Result<Option<String>, CodeSourceError> {
        let Some(source) = self.sources.get(src) else {
            return Ok(None);
        };
        Ok(Some(source.content()?))
    }
    pub fn get_source(&self, src: &Uuid) -> Option<&CodeSource> {
        self.sources.get(src)
    }

    /// Attach a new identity to an already canonicalized path.
    /// Existing files are resolved by `enter_file` before reaching this method.
    fn add_file_src(&mut self, filename: PathBuf, uuid: &Uuid) -> Result<(), CodeSourceError> {
        if self.sources.contains_key(uuid) || self.files.contains_key(&filename) {
            return Err(CodeSourceError::AlreadyImported(filename));
        }
        self.files.insert(filename.clone(), *uuid);
        self.sources.insert(*uuid, CodeSource::File(filename));
        Ok(())
    }
    fn add_inline_src<S: AsRef<str>>(&mut self, content: S, uuid: &Uuid) {
        if let Some(CodeSource::File(path)) = self.sources.get(uuid) {
            self.files.remove(path);
        }
        self.sources
            .insert(*uuid, CodeSource::Inline(content.as_ref().to_owned()));
    }
}
