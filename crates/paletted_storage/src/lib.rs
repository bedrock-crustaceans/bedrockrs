use std::io::Cursor;

use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Debug, Clone)]
pub struct PalettedStorage {
    pub blocks: [u32; 4096],
    pub palette: Vec<nbtx::Value>,
}

impl PalettedStorage {
    // https://www.reddit.com/r/technicalminecraft/comments/x3pzb7/bedrock_leveldb_subchunk_format/
    // I have no idea how this actually works tbh
    pub fn decode(cur: &mut Cursor<&[u8]>) -> PalettedStorage {
        let mut out = PalettedStorage {
            blocks: [0; 4096],
            palette: Vec::new(),
        };
        let palette_type = cur.read_u8().expect("Missing palette type");
        let network = palette_type & 1;
        let bits_per_block = palette_type >> 1;
        if bits_per_block == 0 {
            return out;
        }
        let blocks_per_word = 32 / bits_per_block;
        let num_words: i32 =
            (4096 + Into::<i32>::into(blocks_per_word) - 1) / Into::<i32>::into(blocks_per_word);
        let mask = (1 << bits_per_block) - 1; // assume 2s complement

        let mut pos = 0;
        for _ in 0..num_words {
            let mut word = cur.read_u32::<LittleEndian>().expect("Missing word");
            for _ in 0..blocks_per_word {
                let val = word & mask;
                if pos == 4096 {
                    break;
                }
                out.blocks[pos] = val;
                word >>= bits_per_block;
                pos += 1;
            }
        }

        let palette_count = cur
            .read_i32::<LittleEndian>()
            .expect("Missing palette count");

        for _ in 0..palette_count {
            match network {
                0 => {
                    out.palette.push(
                        todo!(), // NbtTag::nbt_deserialize::<NbtLittleEndian>(cur)
                                 //     .expect("Bad NBT Tag in palette")
                                 //     .1,
                    );
                }
                _ => {
                    out.palette.push(
                        todo!(), // NbtTag::nbt_deserialize::<NbtLittleEndianNetwork>(cur)
                                 //     .expect("Bad NBT Tag in palette")
                                 //     .1,
                    );
                }
            }
        }

        return out;
    }

    pub fn encode(&self, network: bool) -> Vec<u8> {
        let mut out = Vec::new();
        let palette_type: u8 = network.into();
        let bits_per_block = bits_needed_to_store(self.palette.len() as u32);

        let combined = ((bits_per_block << 1) as u8) + palette_type;

        out.push(combined);

        let mut current_word = 0u32;
        let mut bits_written = 0;

        for block in self.blocks {
            if bits_written + bits_per_block > 32 {
                out.extend_from_slice(&current_word.to_le_bytes());
                current_word = 0;
                bits_written = 0;
            }

            current_word = current_word + (block << bits_written);
            bits_written += bits_per_block;
        }

        if bits_written != 0 {
            out.extend_from_slice(&current_word.to_le_bytes());
        }

        out.extend((self.palette.len() as i32).to_le_bytes());

        for nbt in &self.palette {
            if network {
                nbtx::to_net_bytes_in(&mut out, &nbt).unwrap()
            } else {
                nbtx::to_le_bytes_in(&mut out, &nbt).unwrap()
            }
        }

        out
    }
}

fn bits_needed_to_store(n: u32) -> u32 {
    if n == 0 {
        return 1; // Edge case: 0 requires 1 bit to represent
    }
    (32 - n.leading_zeros()) as u32
}
