use std::{
    env::current_dir,
    ffi::OsStr,
    fs::read_dir,
    io::{Error, ErrorKind},
    path::{self, Component, Components, PathBuf},
};

const DEFAULT_SIBS_SCENARIO: &str = "build.sibs";
const SIBS_SCENARIO_EXT: &str = "sibs";

#[derive(Clone, Debug)]
pub struct Location {
    pub path: PathBuf,
    pub filename: PathBuf,
}

impl Location {
    pub fn from(filename: PathBuf) -> Result<Self, Error> {
        Ok(Self {
            filename: filename.clone(),
            path: filename
                .parent()
                .ok_or(Error::new(
                    ErrorKind::NotFound,
                    format!("Fail to find parent folder {filename:?}"),
                ))?
                .to_path_buf(),
        })
    }

    fn search(location: &PathBuf) -> Result<Option<(PathBuf, PathBuf)>, Error> {
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

    pub fn new() -> Result<Self, Error> {
        if let Some((path, filename)) = Self::search(&current_dir()?)? {
            Ok(Self { path, filename })
        } else {
            Err(Error::new(
                ErrorKind::NotFound,
                "Fail to find any sibs files; default sibs file - build.sibs also wasn't found",
            ))
        }
    }
}
