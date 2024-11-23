use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub enum ResourcePackResponse {
    Cancel = 1,
    Downloading = 2,
    DownloadingFinished = 3,
    ResourcePackStackFinished = 4,
}