use byteorder::ReadBytesExt;
use paletted_storage::PalettedStorage;
use std::io::Cursor;

#[derive(Debug)]
pub struct SubChunk {
    pub paletted_storage: Vec<PalettedStorage>
}



impl SubChunk {
    pub fn load(bytes: &Vec<u8>) -> SubChunk {
        let mut cur = Cursor::new(bytes);
        let ver = cur.read_u8().expect("Missing subchunk version");
        match ver {
            8 | 9 => {
                let mut out = SubChunk{paletted_storage: Vec::new()};
                let storage_layers = cur.read_u8().expect("Missing storage layers");
                // let mut y_index = None;
                if ver == 9 {
                    // idk if we need the y index or not yet
                    // y_index = Some(
                        cur.read_u8().expect("Missing Y index");
                    // );
                }
                
                for i in 0..storage_layers {
                    println!("layer: {}, count: {}", i, storage_layers);
                    out.paletted_storage.push(PalettedStorage::decode(&mut cur));
                }

                out
            },

            1 => {
                SubChunk{paletted_storage: vec![PalettedStorage::decode(&mut cur)]}
            }

            a => {panic!("Unsupported subchunk version {}", a);}
        }
    }
}