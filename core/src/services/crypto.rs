use crate::{
    contracts::public::{CryptoConfigPayload, CryptoPublicKeyPayload},
    error::{AppError, Result},
};

pub fn crypto_config() -> CryptoConfigPayload {
    CryptoConfigPayload {
        transport_encryption: false,
    }
}

pub fn crypto_public_key() -> CryptoPublicKeyPayload {
    CryptoPublicKeyPayload {
        transport_encryption: false,
        public_key: String::new(),
    }
}

pub async fn crypto_decrypt() -> Result<CryptoConfigPayload> {
    Err(AppError::BadRequest(
        "transport encryption is disabled; encrypted decrypt endpoint is unavailable".into(),
    ))
}
