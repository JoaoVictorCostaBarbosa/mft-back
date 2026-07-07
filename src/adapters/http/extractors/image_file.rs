use crate::application::errors::AppError;
use axum::{
    async_trait,
    extract::{FromRequest, Request},
};
use axum_extra::extract::Multipart;

use crate::adapters::http::errors::HttpError;
use crate::application::errors::FileError;

pub const MAX_IMAGE_SIZE_BYTES: usize = 10_000_000;

pub struct ImageFile(pub Vec<u8>);

#[async_trait]
impl<S> FromRequest<S> for ImageFile
where
    S: Send + Sync,
{
    type Rejection = HttpError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let mut multipart = Multipart::from_request(req, state)
            .await
            .map_err(|_| HttpError(AppError::File(FileError::FileReadError)))?;

        while let Some(field) = multipart
            .next_field()
            .await
            .map_err(|_| HttpError(AppError::File(FileError::FileReadError)))?
        {
            if field.name() == Some("file") {
                let mime = field
                    .content_type()
                    .map(|m| m.to_string())
                    .ok_or_else(|| HttpError(AppError::File(FileError::InvalidMimeType)))?;

                if mime != "image/png" && mime != "image/jpeg" {
                    return Err(HttpError(AppError::File(FileError::InvalidMimeType)));
                }

                let bytes = field
                    .bytes()
                    .await
                    .map_err(|_| HttpError(AppError::File(FileError::FileReadError)))?;

                if bytes.len() > MAX_IMAGE_SIZE_BYTES {
                    return Err(HttpError(AppError::File(FileError::FileTooLarge {
                        max_size: MAX_IMAGE_SIZE_BYTES,
                    })));
                }

                return Ok(ImageFile(bytes.to_vec()));
            }
        }

        Err(HttpError(AppError::File(FileError::MissingFile)))
    }
}
