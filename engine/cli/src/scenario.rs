use crate::*;
use std::{
    env::current_dir,
    ffi::OsStr,
    fs::read_dir,
    path::{self, Component, Path, PathBuf},
};

const DEFAULT_SIBS_SCENARIO: &str = "main.sibs";
const SIBS_SCENARIO_EXT: &str = "sibs";

#[derive(Clone, Debug)]
pub struct Scenario {
    pub filepath: PathBuf,
}

impl Scenario {
    pub fn new() -> Result<Self, E> {
        let current = current_dir()?;
        Self::search(&current)?
            .map(|filepath| Self { filepath })
            .ok_or(E::ScenarioNotFound(current.to_string_lossy().to_string()))
    }

    pub fn from<P: AsRef<Path>>(filepath: P) -> Result<Self, E> {
        let filepath = filepath.as_ref().to_path_buf();
        let filepath = if !filepath.is_absolute() {
            current_dir()?.join(filepath)
        } else {
            filepath
        };
        if !filepath.exists() {
            Err(E::PathDoesNotExist(filepath.to_string_lossy().to_string()))?;
        }
        Ok(Self { filepath })
    }

    fn search(location: &PathBuf) -> Result<Option<PathBuf>, E> {
        if location.join(DEFAULT_SIBS_SCENARIO).exists() {
            return Ok(Some(location.join(DEFAULT_SIBS_SCENARIO)));
        }
        let mut filename: Option<PathBuf> = None;
        for entry in read_dir(location)? {
            let entry = entry?.path();
            if entry.is_file() {
                if let Some(ext) = entry.extension() {
                    if ext.to_string_lossy().to_lowercase() == SIBS_SCENARIO_EXT {
                        filename = Some(entry);
                        break;
                    }
                }
            }
        }
        if let Some(filename) = filename.take() {
            return Ok(Some(filename));
        } else if matches!(location.components().next_back(), Some(Component::RootDir))
            || location.components().count() <= 1
        {
            return Ok(None);
        }
        let mut left = location.components().collect::<Vec<Component<'_>>>();
        let _ = left.pop();
        Self::search(&PathBuf::from(
            left.iter()
                .map(|c| c.as_os_str())
                .collect::<Vec<&OsStr>>()
                .join(OsStr::new(path::MAIN_SEPARATOR_STR)),
        ))
    }
}
