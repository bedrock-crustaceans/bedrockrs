use bedrock_core::dimension::Dimension;

#[derive(Clone, Copy)]
pub enum RecordType {
    Data3D,
    Version,
    Data2D,
    Data2DLegacy,
    SubChunkPrefix{y: u8}, // -4 to 20 as of 1.18
    LegacyTerrain,
    BlockEntity,
    Entity,
    PendingTicks,
    LegacyBlockExtraData,
    BiomeState,
    FinalizedState,
    ConversionData,
    BorderBlocks,
    HardcodedSpawners,
    RandomTicks,
    Checksums,
    MetaDataHash,
    GeneratedPreCavesAndCliffsBlending,
    BlendingBiomeHeight,
    BlendingData,
    ActorDigestVersion,
    LegacyVersion
}

impl From<RecordType> for u8 {
    fn from(value: RecordType) -> Self {
        match value {
            RecordType::Data3D => 43,
            RecordType::Version => 44,
            RecordType::Data2D => 45,
            RecordType::Data2DLegacy => 46,
            RecordType::SubChunkPrefix {y: _} => 47,
            RecordType::LegacyTerrain => 48,
            RecordType::BlockEntity => 49,
            RecordType::Entity => 50,
            RecordType::PendingTicks => 51,
            RecordType::LegacyBlockExtraData => 52,
            RecordType::BiomeState => 53,
            RecordType::FinalizedState => 54,
            RecordType::ConversionData => 55,
            RecordType::BorderBlocks => 56,
            RecordType::HardcodedSpawners => 57,
            RecordType::RandomTicks => 58,
            RecordType::Checksums => 59,
            RecordType::MetaDataHash => 61,
            RecordType::GeneratedPreCavesAndCliffsBlending => 62,
            RecordType::BlendingBiomeHeight => 63,
            RecordType::BlendingData => 64,
            RecordType::ActorDigestVersion => 65,
            RecordType::LegacyVersion => 76
        }
    }
}

pub fn create_key(x: i32, z: i32, dimension: Dimension, record_type: RecordType) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    out.extend_from_slice(&x.to_le_bytes());
    out.extend_from_slice(&z.to_le_bytes());
    match dimension {
        Dimension::Overworld => {},
        dim => {
            out.extend_from_slice(&(dim as i32).to_le_bytes());
        }
    }

    out.push(Into::<u8>::into(record_type));

    match record_type {
        RecordType::SubChunkPrefix { y } => {
            out.push(y);
        }
        _ => {}
    }

    out
}