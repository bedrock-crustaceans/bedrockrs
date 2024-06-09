use std::io::Cursor;

use byteorder::{LittleEndian, ReadBytesExt};
use nbt::{endian::little_endian::NbtLittleEndian, NbtTag};

#[derive(Debug)]
pub struct PalettedStorage {
    blocks: [i32; 4096],
    palette: Vec<NbtTag>,
}

impl PalettedStorage {
    // https://www.reddit.com/r/technicalminecraft/comments/x3pzb7/bedrock_leveldb_subchunk_format/
    // I have no idea how this actually works tbh
    pub fn decode(cur: &mut Cursor<&Vec<u8>>) -> PalettedStorage {
        let mut out = PalettedStorage {
            blocks: [0; 4096],
            palette: Vec::new(),
        };
        let palette_type = cur.read_u8().expect("Missing palette type");
        let bits_per_block = palette_type >> 1;
        println!("bpb: {}", bits_per_block);
        if bits_per_block == 0 {
            return out;
        }
        let blocks_per_word = 32 / bits_per_block;
        let num_words: i32 =
            (4096 + Into::<i32>::into(blocks_per_word) - 1) / Into::<i32>::into(blocks_per_word);
        let mask = (1 << bits_per_block) - 1; // assume 2s complement

        let mut pos = 0;
        for _ in 0..num_words {
            let mut word = cur.read_i32::<LittleEndian>().expect("Missing word");
            for _ in 0..blocks_per_word {
                let val = word & mask;
                if pos == 4096 {
                    break;
                }
                out.blocks[pos] = val;
                word = word >> bits_per_block;
                pos += 1;
            }
        }

        let palette_count = cur
            .read_i32::<LittleEndian>()
            .expect("Missing palette count");

        for _ in 0..palette_count {
            out.palette.push(
                NbtTag::nbt_deserialize::<NbtLittleEndian>(cur)
                    .expect("Bad NBT Tag in palette")
                    .1,
            );
        }

        return out;
    }
}
