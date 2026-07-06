use thiserror::Error;

#[derive(Debug, Error)]
pub enum FileError {
    #[error("no file uploaded")]
    MissingFile,

    #[error("invalid file type, only JPEG or PNG are accepted")]
    InvalidMimeType,

    #[error("the file is too large, the maximum allowed size is {max_size} bytes")]
    FileTooLarge { max_size: usize },

    #[error("failed to read the file")]
    FileReadError,
}
