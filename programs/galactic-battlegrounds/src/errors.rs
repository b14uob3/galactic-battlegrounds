use crate::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Account already initialized")]
    AlreadyInitialized
}