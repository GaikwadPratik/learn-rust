#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to open repository")]
    ErrRepositoryOpen,
    #[error("Failed to read tag names")]
    ErrTagNamesRead,
}
