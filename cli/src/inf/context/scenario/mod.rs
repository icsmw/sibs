mod error;
pub use error::E;
use std::{
    env::current_dir,
    ffi::OsStr,
    fs::read_dir,
    path::{self, Component, Path, PathBuf},
};

const DEFAULT_SIBS_SCENARIO: &str = "build.sibs";
const SIBS_SCENARIO_EXT: &str = "sibs";

#[derive(Clone, Debug)]
pub struct Scenario {
    pub path: PathBuf,
    pub filename: PathBuf,
}

impl Scenario {
    pub fn new() -> Result<Self, E> {
        if let Some((path, filename)) = Self::search(&current_dir()?)? {
            Ok(Self { path, filename })
        } else {
            Err(E::ScenarioNotFound)
        }
    }
    #[cfg(test)]
    pub fn dummy() -> Self {
        Self {
            path: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
            filename: PathBuf::from("dummy.sibs"),
        }
    }
    pub fn from(filename: &Path) -> Result<Self, E> {
        let filename_str = filename.to_string_lossy().to_string();
        let abs = if !filename.is_absolute() {
            current_dir()?.join(filename)
        } else {
            filename.to_owned()
        };
        if !abs.exists() {
            Err(E::PathDoesNotExist(filename_str.clone()))?;
        }
        Ok(Self {
            filename: abs.to_path_buf(),
            path: abs
                .parent()
                .ok_or(E::NoParentFolderFor(filename_str))?
                .to_path_buf(),
        })
    }

    pub fn to_relative_path(&self, path: &Path) -> String {
        path.to_string_lossy()
            .to_string()
            .replace(&self.path.to_string_lossy().to_string(), "")
    }

    pub fn to_abs_path(&self, path: &Path) -> Result<PathBuf, E> {
        Ok(self.path.join(path).canonicalize()?)
    }

    fn search(location: &PathBuf) -> Result<Option<(PathBuf, PathBuf)>, E> {
        if location.join(DEFAULT_SIBS_SCENARIO).exists() {
            return Ok(Some((
                location.clone(),
                location.join(DEFAULT_SIBS_SCENARIO),
            )));
        }
        let mut filename: Option<PathBuf> = None;
        for entry in read_dir(location)? {
            let entry = entry?.path();
            if entry.is_file() {
                if let Some(ext) = entry.extension() {
                    if ext.to_string_lossy() == SIBS_SCENARIO_EXT {
                        filename = Some(entry);
                        break;
                    }
                }
            }
        }
        if let Some(filename) = filename.take() {
            return Ok(Some((location.clone(), filename)));
        } else if matches!(location.components().last(), Some(Component::RootDir))
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
