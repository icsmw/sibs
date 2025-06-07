use crate::*;
use std::path::PathBuf;

pub fn register(efns: &mut EFns) -> Result<(), E> {
    #[import(fs)]
    /// Creates a new, empty directory at the provided path
    ///
    /// # Platform-specific behavior
    ///
    /// This function currently corresponds to the `mkdir` function on Unix
    /// and the `CreateDirectoryW` function on Windows.
    ///
    /// **NOTE**: If a parent of the given path doesn't exist, this function will
    /// return an error.
    ///
    /// # Errors
    ///
    /// This function will return an error in the following situations, but is not
    /// limited to just these cases:
    ///
    /// * User lacks permissions to create directory at `path`.
    /// * A parent of the given path doesn't exist. (To create a directory and all
    ///   its missing parents at the same time, use the [`create_dir_all`]
    ///   function.)
    /// * `path` already exists.
    fn create_dir(path: PathBuf) -> Result<(), E> {
        Ok(std::fs::create_dir(path)?)
    }
    #[import(fs)]
    /// Recursively create a directory and all of its parent components if they
    /// are missing.
    ///
    /// If this function returns an error, some of the parent components might have
    /// been created already.
    ///
    /// If the empty path is passed to this function, it always succeeds without
    /// creating any directories.
    ///
    /// # Platform-specific behavior
    ///
    /// This function currently corresponds to multiple calls to the `mkdir`
    /// function on Unix and the `CreateDirectoryW` function on Windows.
    ///
    /// # Errors
    ///
    /// The function will return an error if any directory specified in path does not exist and
    /// could not be created.
    ///
    /// Notable exception is made for situations where any of the directories
    /// specified in the `path` could not be created as it was being created concurrently.
    /// Such cases are considered to be successful. That is, calling `create_dir_all`
    /// concurrently from multiple threads or processes is guaranteed not to fail
    /// due to a race condition with itself.
    fn create_dir_all(path: PathBuf) -> Result<(), E> {
        Ok(std::fs::create_dir_all(path)?)
    }
    #[import(fs)]
    /// Removes an empty directory.
    ///
    /// If you want to remove a directory that is not empty, as well as all
    /// of its contents recursively, consider using [`remove_dir_all`]
    /// instead.
    ///
    /// # Platform-specific behavior
    ///
    /// This function currently corresponds to the `rmdir` function on Unix
    /// and the `RemoveDirectory` function on Windows.
    ///
    /// # Errors
    ///
    /// This function will return an error in the following situations, but is not
    /// limited to just these cases:
    ///
    /// * `path` doesn't exist.
    /// * `path` isn't a directory.
    /// * The user lacks permissions to remove the directory at the provided `path`.
    /// * The directory isn't empty.
    ///
    /// This function will only ever return an error of kind `NotFound` if the given
    /// path does not exist. Note that the inverse is not true,
    /// ie. if a path does not exist, its removal may fail for a number of reasons,
    /// such as insufficient permissions.
    fn remove_dir(path: PathBuf) -> Result<(), E> {
        Ok(std::fs::remove_dir(path)?)
    }
    #[import(fs)]
    /// Removes a directory at this path, after removing all its contents. Use
    /// carefully!
    ///
    /// This function does **not** follow symbolic links and it will simply remove the
    /// symbolic link itself.
    ///
    /// # Platform-specific behavior
    ///
    /// This function currently corresponds to `openat`, `fdopendir`, `unlinkat` and `lstat` functions
    /// on Unix (except for REDOX) and the `CreateFileW`, `GetFileInformationByHandleEx`,
    /// `SetFileInformationByHandle`, and `NtCreateFile` functions on Windows.
    ///
    /// On REDOX, as well as when running in Miri for any target, this function is not protected against
    /// time-of-check to time-of-use (TOCTOU) race conditions, and should not be used in
    /// security-sensitive code on those platforms. All other platforms are protected.
    ///
    /// # Errors
    ///
    /// [`remove_dir_all`] will fail if [`remove_dir`] or [`remove_file`] fail on *any* constituent
    /// paths, *including* the root `path`. Consequently,
    ///
    /// - The directory you are deleting *must* exist, meaning that this function is *not idempotent*.
    /// - [`remove_dir_all`] will fail if the `path` is *not* a directory.
    ///
    /// Consider ignoring the error if validating the removal is not required for your use case.
    fn remove_dir_all(path: PathBuf) -> Result<(), E> {
        Ok(std::fs::remove_dir_all(path)?)
    }
    #[import(fs)]
    /// Removes a file from the filesystem.
    ///
    /// Note that there is no
    /// guarantee that the file is immediately deleted (e.g., depending on
    /// platform, other open file descriptors may prevent immediate removal).
    ///
    /// # Platform-specific behavior
    ///
    /// This function currently corresponds to the `unlink` function on Unix.
    /// On Windows, `DeleteFile` is used or `CreateFileW` and `SetInformationByHandle` for readonly files.
    /// Note that, this [may change in the future][changes].
    ///
    /// # Errors
    ///
    /// This function will return an error in the following situations, but is not
    /// limited to just these cases:
    ///
    /// * `path` points to a directory.
    /// * The file doesn't exist.
    /// * The user lacks permissions to remove the file.
    ///
    /// This function will only ever return an error of kind `NotFound` if the given
    /// path does not exist. Note that the inverse is not true,
    /// ie. if a path does not exist, its removal may fail for a number of reasons,
    /// such as insufficient permissions.
    fn remove_file(path: PathBuf) -> Result<(), E> {
        Ok(std::fs::remove_file(path)?)
    }
    #[import(fs)]
    /// Renames a file or directory to a new name, replacing the original file if
    /// `to` already exists.
    ///
    /// This will not work if the new name is on a different mount point.
    ///
    /// # Platform-specific behavior
    ///
    /// This function currently corresponds to the `rename` function on Unix
    /// and the `MoveFileExW` or `SetFileInformationByHandle` function on Windows.
    ///
    /// Because of this, the behavior when both `from` and `to` exist differs. On
    /// Unix, if `from` is a directory, `to` must also be an (empty) directory. If
    /// `from` is not a directory, `to` must also be not a directory. The behavior
    /// on Windows is the same on Windows 10 1607 and higher if `FileRenameInfoEx`
    /// is supported by the filesystem; otherwise, `from` can be anything, but
    /// `to` must *not* be a directory.
    ///
    /// # Errors
    ///
    /// This function will return an error in the following situations, but is not
    /// limited to just these cases:
    ///
    /// * `from` does not exist.
    /// * The user lacks permissions to view contents.
    /// * `from` and `to` are on separate filesystems.
    fn rename(a: PathBuf, b: PathBuf) -> Result<(), E> {
        Ok(std::fs::rename(a, b)?)
    }
    #[import(fs)]
    /// Copies the contents of one file to another. This function will also
    /// copy the permission bits of the original file to the destination file.
    ///
    /// This function will **overwrite** the contents of `to`.
    ///
    /// Note that if `from` and `to` both point to the same file, then the file
    /// will likely get truncated by this operation.
    ///
    /// On success, the total number of bytes copied is returned and it is equal to
    /// the length of the `to` file as reported by `metadata`.
    ///
    /// # Platform-specific behavior
    ///
    /// This function currently corresponds to the `open` function in Unix
    /// with `O_RDONLY` for `from` and `O_WRONLY`, `O_CREAT`, and `O_TRUNC` for `to`.
    /// `O_CLOEXEC` is set for returned file descriptors.
    ///
    /// On Linux (including Android), this function attempts to use `copy_file_range(2)`,
    /// and falls back to reading and writing if that is not possible.
    ///
    /// On Windows, this function currently corresponds to `CopyFileEx`. Alternate
    /// NTFS streams are copied but only the size of the main stream is returned by
    /// this function.
    ///
    /// On MacOS, this function corresponds to `fclonefileat` and `fcopyfile`.
    ///
    /// # Errors
    ///
    /// This function will return an error in the following situations, but is not
    /// limited to just these cases:
    ///
    /// * `from` is neither a regular file nor a symlink to a regular file.
    /// * `from` does not exist.
    /// * The current process does not have the permission rights to read
    ///   `from` or write `to`.
    /// * The parent directory of `to` doesn't exist.
    fn copy(a: PathBuf, b: PathBuf) -> Result<u64, E> {
        Ok(std::fs::copy(a, b)?)
    }
    #[import(fs)]
    /// Creates a new hard link on the filesystem.
    ///
    /// The `link` path will be a link pointing to the `original` path. Note that
    /// systems often require these two paths to both be located on the same
    /// filesystem.
    ///
    /// If `original` names a symbolic link, it is platform-specific whether the
    /// symbolic link is followed. On platforms where it's possible to not follow
    /// it, it is not followed, and the created hard link points to the symbolic
    /// link itself.
    ///
    /// # Platform-specific behavior
    ///
    /// This function currently corresponds the `CreateHardLink` function on Windows.
    /// On most Unix systems, it corresponds to the `linkat` function with no flags.
    /// On Android, VxWorks, and Redox, it instead corresponds to the `link` function.
    /// On MacOS, it uses the `linkat` function if it is available, but on very old
    /// systems where `linkat` is not available, `link` is selected at runtime instead.
    ///
    /// # Errors
    ///
    /// This function will return an error in the following situations, but is not
    /// limited to just these cases:
    ///
    /// * The `original` path is not a file or doesn't exist.
    /// * The 'link' path already exists.
    fn hard_link(a: PathBuf, b: PathBuf) -> Result<(), E> {
        Ok(std::fs::hard_link(a, b)?)
    }
    #[import(fs)]
    /// Returns the canonical, absolute form of a path with all intermediate
    /// components normalized and symbolic links resolved.
    ///
    /// # Platform-specific behavior
    ///
    /// This function currently corresponds to the `realpath` function on Unix
    /// and the `CreateFile` and `GetFinalPathNameByHandle` functions on Windows.
    ///
    /// On Windows, this converts the path to use [extended length path][path]
    /// syntax, which allows your program to use longer path names, but means you
    /// can only join backslash-delimited paths to it, and it may be incompatible
    /// with other applications (if passed to the application on the command-line,
    /// or written to a file another application may read).
    ///
    /// [path]: https://docs.microsoft.com/en-us/windows/win32/fileio/naming-a-file
    ///
    /// # Errors
    ///
    /// This function will return an error in the following situations, but is not
    /// limited to just these cases:
    ///
    /// * `path` does not exist.
    /// * A non-final component in path is not a directory.
    fn canonicalize(path: PathBuf) -> Result<PathBuf, E> {
        Ok(std::fs::canonicalize(path)?)
    }
    #[import(fs)]
    /// Reads the entire contents of a file into a string.
    ///
    /// This is a convenience function for using [`File::open`] and [`read_to_string`]
    /// with fewer imports and without an intermediate variable.
    ///
    /// # Errors
    ///
    /// This function will return an error if `path` does not already exist.
    ///
    /// If the contents of the file are not valid UTF-8, then an error will also be
    /// returned.
    ///
    /// While reading from the file, this function handles `io::ErrorKind::Interrupted`
    /// with automatic retries.
    fn read_to_string(path: PathBuf) -> Result<String, E> {
        Ok(std::fs::read_to_string(path)?)
    }
    #[import(fs)]
    /// Writes a slice as the entire contents of a file.
    ///
    /// This function will create a file if it does not exist,
    /// and will entirely replace its contents if it does.
    ///
    /// Depending on the platform, this function may fail if the
    /// full directory path does not exist.
    fn write(path: PathBuf, data: String) -> Result<(), E> {
        Ok(std::fs::write(path, data)?)
    }
    #[import(fs)]
    /// Documentation placeholder
    fn append(path: PathBuf, data: String) -> Result<(), E> {
        use std::fs::OpenOptions;
        use std::io::Write;
        let mut file = OpenOptions::new().append(true).open(path)?;
        Ok(writeln!(file, "{data}")?)
    }
    #[import(fs)]
    /// Returns `true` if this metadata is for a regular file. The
    /// result is mutually exclusive to the result of
    ///
    /// When the goal is simply to read from (or write to) the source, the most
    /// reliable way to test the source can be read (or written to) is to open
    /// it.
    fn is_file(path: PathBuf) -> Result<bool, E> {
        Ok(std::fs::metadata(path)?.is_file())
    }
    #[import(fs)]
    /// Returns `true` if this metadata is for a directory. The
    /// result is mutually exclusive to the result of
    /// `is_file`, and will be false for symlink metadata
    /// obtained
    fn is_dir(path: PathBuf) -> Result<bool, E> {
        Ok(std::fs::metadata(path)?.is_dir())
    }
    #[import(fs)]
    /// Documentation placeholder
    fn exists(path: PathBuf) -> Result<bool, E> {
        Ok(path.exists())
    }
    #[import(fs)]
    /// Returns `true` if the path points at an existing entity.
    ///
    /// Warning: this method may be error-prone, consider using [`try_exists()`] instead!
    /// It also has a risk of introducing time-of-check to time-of-use (TOCTOU) bugs.
    ///
    /// This function will traverse symbolic links to query information about the
    /// destination file.
    ///
    /// If you cannot access the metadata of the file, e.g. because of a
    /// permission error or broken symbolic links, this will return `false`.
    fn is_absolute(path: PathBuf) -> Result<bool, E> {
        Ok(path.is_absolute())
    }
    #[import(fs)]
    /// Returns `true` if the `Path` is relative, i.e., not absolute.
    fn is_relative(path: PathBuf) -> Result<bool, E> {
        Ok(path.is_relative())
    }
    #[import(fs)]
    /// Returns `true` if this metadata is for a symbolic link.
    fn is_symlink(path: PathBuf) -> Result<bool, E> {
        Ok(std::fs::metadata(path)?.is_symlink())
    }
    #[import(fs)]
    /// Returns the size of the file, in bytes, this metadata is for.
    fn file_size(path: PathBuf) -> Result<u64, E> {
        Ok(std::fs::metadata(path)?.len())
    }
    #[import(fs)]
    /// Documentation placeholder
    fn file_created_timestamp(path: PathBuf) -> Result<u128, E> {
        Ok(std::fs::metadata(path)?
            .created()?
            .duration_since(std::time::UNIX_EPOCH)?
            .as_millis())
    }
    #[import(fs)]
    /// Documentation placeholder
    fn file_modified_timestamp(path: PathBuf) -> Result<u128, E> {
        Ok(std::fs::metadata(path)?
            .modified()?
            .duration_since(std::time::UNIX_EPOCH)?
            .as_millis())
    }
    #[import(fs)]
    /// Documentation placeholder
    fn file_accessed_timestamp(path: PathBuf) -> Result<u128, E> {
        Ok(std::fs::metadata(path)?
            .accessed()?
            .duration_since(std::time::UNIX_EPOCH)?
            .as_millis())
    }
    #[import(fs)]
    /// Documentation placeholder
    fn path_join(paths: Vec<PathBuf>) -> Result<PathBuf, E> {
        let mut path = PathBuf::new();
        paths.iter().for_each(|part| {
            path.push(part);
        });
        Ok(path)
    }
    Ok(())
}
