use axum::{
    async_trait,
    extract::{FromRequest, Request},
};
use axum_extra::extract::Multipart;
use crate::domain::errors::{domain_error::DomainError, file_error::FileError};

pub struct ImageFile(pub Vec<u8>);

#[async_trait]
impl<S> FromRequest<S> for ImageFile
where
    S: Send + Sync,
{
    type Rejection = DomainError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let mut multipart = Multipart::from_request(req, state)
            .await
            .map_err(|_| DomainError::File(FileError::FileReadError))?;

        while let Some(field) = multipart
            .next_field()
            .await
            .map_err(|_| DomainError::File(FileError::FileReadError))?
        {
            if field.name() == Some("file") {
                let mime = field
                    .content_type()
                    .map(|m| m.to_string())
                    .ok_or(DomainError::File(FileError::InvalidMimeType))?;

                if mime != "image/png" && mime != "image/jpeg" {
                    return Err(DomainError::File(FileError::InvalidMimeType));
                }

                let bytes = field
                    .bytes()
                    .await
                    .map_err(|_| DomainError::File(FileError::FileReadError))?;

                if bytes.len() > 2_000_000 {
                    return Err(DomainError::File(FileError::FileTooLarge {
                        max_size: 2_000_000,
                    }));
                }

                return Ok(ImageFile(bytes.to_vec()));
            }
        }

        Err(DomainError::File(FileError::MissingFile))
    }
}
