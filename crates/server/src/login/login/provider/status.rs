use crate::types::disconnect_reason::DisconnectReason;

pub enum LoginProviderStatus {
    ContinueLogin,
    AbortLogin {
        reason: String,
        disconnect_reason: Option<DisconnectReason>,
    },
}
