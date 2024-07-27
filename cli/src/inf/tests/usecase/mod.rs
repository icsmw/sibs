pub(crate) mod strategy;
use log::debug;
use rand::Rng;
use std::{
    env::temp_dir,
    fs::{create_dir, remove_dir_all, remove_file, OpenOptions},
    io::{self, Write},
    path::PathBuf,
    time::Instant,
};
pub(crate) use strategy::Strategy;
use uuid::Uuid;

pub struct UseCaseEmpty {
    pub root: PathBuf,
}

impl UseCaseEmpty {
    pub fn gen() -> Result<Self, io::Error> {
        let root = temp_dir().join(Uuid::new_v4().to_string());
        create_dir(&root)?;
        Ok(Self { root })
    }
    pub fn clean(&self) -> Result<(), io::Error> {
        if !self.root.exists() {
            return Ok(());
        }
        remove_dir_all(&self.root)
    }
}
pub struct UseCase {
    pub files: Vec<PathBuf>,
    pub root: PathBuf,
}

impl UseCase {
    pub fn gen(
        folders_strategy: Strategy,
        files_strategy: Strategy,
        deep: u8,
    ) -> Result<Self, io::Error> {
        let now = Instant::now();
        debug!("Start generiting use case: {folders_strategy}; {files_strategy} per folder; deep = {deep};");
        let mut files = Vec::new();
        let mut fill = |parent: &PathBuf| -> Result<Vec<PathBuf>, io::Error> {
            let mut created = Vec::new();
            let mut folders_cursor = folders_strategy.get_cursor(parent);
            for _ in 0..folders_strategy.count() {
                let folder = folders_cursor.next();
                create_dir(&folder)?;
                let mut files_cursor = files_strategy.get_cursor(&folder);
                for _ in 0..files_strategy.count() {
                    let filename = files_cursor.next();
                    let mut file = OpenOptions::new()
                        .write(true)
                        .create(true)
                        .truncate(true)
                        .open(&filename)?;
                    file.write_all(Uuid::new_v4().as_bytes())?;
                    file.flush()?;
                    files.push(filename);
                }
                created.push(folder);
            }
            Ok(created)
        };
        let tmp = temp_dir();
        let root = tmp.join(Uuid::new_v4().to_string());
        if root.exists() {
            remove_dir_all(&root)?;
        }
        create_dir(&root)?;
        let mut folders: Vec<PathBuf> = fill(&root)?;
        for _ in 0..deep {
            let to_be_processed: Vec<PathBuf> = folders.to_vec();
            folders = Vec::new();
            for folder in to_be_processed.iter() {
                folders.append(&mut fill(folder)?);
            }
        }
        debug!(
            "in \"{}\" created {} files in {}µs / {}ms / {}s",
            root.display(),
            files.len(),
            now.elapsed().as_micros(),
            now.elapsed().as_millis(),
            now.elapsed().as_secs()
        );
        Ok(Self { files, root })
    }

    pub fn change(&self, count: usize) -> Result<(), io::Error> {
        if self.files.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "No files has been created. Cannot change a state",
            ));
        }
        for _ in 0..count {
            let Some(filename) = self
                .files
                .get(rand::thread_rng().gen_range(0..self.files.len() - 1))
            else {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "Cannot find a file path by index",
                ));
            };
            let mut file = OpenOptions::new().append(true).open(filename)?;
            file.write_all(Uuid::new_v4().as_bytes())?;
            file.flush()?;
        }
        Ok(())
    }

    pub fn remove(&self, count: usize) -> Result<(), io::Error> {
        if self.files.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "No files has been created. Cannot change a state",
            ));
        }
        if count > self.files.len() - 1 {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Cannot remove more files than created",
            ));
        }
        let s = rand::thread_rng().gen_range(0..self.files.len() - 1 - count);
        for i in s..(s + count) {
            let Some(filename) = self.files.get(i) else {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "Cannot find a file path by index",
                ));
            };
            if filename.exists() {
                remove_file(filename)?;
            }
        }
        Ok(())
    }

    pub fn clean(&self) -> Result<(), io::Error> {
        if !self.root.exists() {
            return Ok(());
        }
        let Some(parent) = self.root.parent() else {
            return Ok(());
        };
        if !parent.starts_with(temp_dir()) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("parent of root isn't belong to {}", temp_dir().display()),
            ));
        }
        remove_dir_all(&self.root)?;
        debug!("Removed {}", self.root.display());
        Ok(())
    }
}
