#[derive(Copy, Clone, Debug)]
pub enum SerilizationError {
    WriteIOError,
    GenerateKeyError,
    JwtError,
}

#[derive(Copy, Clone, Debug)]
pub enum DeserilizationError {
    ReadIOError,
    NotEnoughRemainingError,
    ReadUtf8StringError,
    ReadJsonError,
    ReadJwtError,
    ReadEcdsaKey,
    Base64Error,
    InvalidGamepacketID,
    MissingField,
}
