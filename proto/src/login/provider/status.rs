pub enum LoginProviderStatus {
    ContinueLogin,
    AbortLogin { reason: String },
}
