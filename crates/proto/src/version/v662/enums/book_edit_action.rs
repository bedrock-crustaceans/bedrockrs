use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub enum BookEditAction {
    ReplacePage = 0,
    AddPage = 1,
    DeletePage = 2,
    SwapPages = 3,
    Finalize = 4,
}