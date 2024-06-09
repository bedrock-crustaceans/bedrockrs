use byteorder::ReadBytesExt;
use paletted_storage::PalettedStorage;
use std::io::Cursor;

pub struct SubChunk {
    pub paletted_storage: PalettedStorage
}



impl SubChunk {
    pub fn load(bytes: Vec<u8>) -> SubChunk {
        let mut cur = Cursor::new(bytes);
        let ver = cur.read_u8().expect("Missing subchunk version");
        match ver {
            8 | 9 => {
                let storage_layers = cur.read_u8().expect("Missing storage layers");
                let mut y_index = None;
                if ver == 9 {
                    y_index = Some(cur.read_u8().expect("Missing Y index"));
                }

                println!("storage_layers:{}", storage_layers);
                println!("y-index:{:?}", y_index);
                
                SubChunk{paletted_storage: PalettedStorage::decode(cur)}
            },

            1 => {
                todo!("Subchunk V1");
            }

            a => {panic!("Unsupported subchunk version {}", a);}
        }
    }
}