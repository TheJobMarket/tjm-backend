use thiserror::Error;
use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::operation::delete_object::DeleteObjectError;
use aws_sdk_s3::operation::get_object::GetObjectError;
use aws_sdk_s3::operation::put_object::PutObjectError;
use image::ImageError;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Error while deleting object: {0}")]
    DeleteObjectError(#[from] SdkError<DeleteObjectError>),
    #[error("Error while getting image: {0}")]
    GetObjectError(#[from] SdkError<GetObjectError>),
    #[error("Error while inserting image: {0}")]
    PutObjectError(#[from] SdkError<PutObjectError>),
    #[error("Error while manipulating image bytes: {0}")]
    ImageError(#[from] ImageError),
    // #[error("Error while getting data from multipart: {0}")]
    // Multipart(#[from] MultipartError),
    // #[error("IO error: {0}")]
    // IO(#[from] IoError),
    #[error("Body is empty")]
    EmptyBody, // the user tried to send an empty body while uploading
}