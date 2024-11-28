use std::io::Cursor;
use std::mem::size_of;
use uuid::Uuid;
use varint_rs::VarintWriter;
use bedrockrs_proto_core::error::ProtoCodecError;
use bedrockrs_proto_core::ProtoCodec;
use crate::version::v662::types::{NetworkItemInstanceDescriptor, RecipeIngredient};

pub struct ShapedRecipe {
    pub recipe_unique_id: String,
    pub ingredient_grid: Vec<Vec<RecipeIngredient>>,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub production_list: Vec<NetworkItemInstanceDescriptor>,
    pub recipe_id: Uuid,
    pub recipe_tag: String,
    #[endianness(var)]
    pub priority: i32,
}

impl ProtoCodec for ShapedRecipe {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        self.recipe_unique_id.proto_serialize(stream)?;
        
        let x_len: u32 = self.ingredient_grid.len().try_into()?;
        let y_len: u32 = self.ingredient_grid[0].len().try_into()?;
        stream.write_u32_varint(x_len)?;
        stream.write_u32_varint(y_len)?;
        for y in &self.ingredient_grid {
            for recipe in y {
                recipe.proto_serialize(stream)?;
            }
        }
        
        let p_len: u32 = self.production_list.len().try_into()?;
        stream.write_u32_varint(p_len)?;
        for p in &self.production_list {
            p.proto_serialize(stream)?;
        }
        
        self.recipe_id.proto_serialize(stream)?;
        self.recipe_tag.proto_serialize(stream)?;
        stream.write_i32_varint(self.priority)?;
        
        Ok(())
    }

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
        let recipe_unique_id = <String>::proto_deserialize(stream)?;
        
        let x_len = <u32 as ::bedrockrs_proto_core::ProtoCodecVAR>::proto_deserialize(stream)?;
        let y_len = <u32 as ::bedrockrs_proto_core::ProtoCodecVAR>::proto_deserialize(stream)?;
        let mut ingredient_grid = Vec::with_capacity(x_len.try_into()?);
        for _ in 0..x_len {
            let mut y_vec = Vec::with_capacity(y_len.try_into()?);
            for _ in 0..y_len {
                y_vec.push(<RecipeIngredient>::proto_deserialize(stream)?);
            }
            ingredient_grid.push(y_vec);
        }
        
        let p_len = <u32 as ::bedrockrs_proto_core::ProtoCodecVAR>::proto_deserialize(stream)?;
        let mut production_list = Vec::with_capacity(p_len.try_into()?);
        for _ in 0..p_len {
            production_list.push(<NetworkItemInstanceDescriptor>::proto_deserialize(stream)?);
        }
        
        let recipe_id = <Uuid>::proto_deserialize(stream)?;
        let recipe_tag = <String>::proto_deserialize(stream)?;
        let priority = <i32 as ::bedrockrs_proto_core::ProtoCodecVAR>::proto_deserialize(stream)?;
        
        Ok(Self {
            recipe_unique_id,
            ingredient_grid,
            production_list,
            recipe_id,
            recipe_tag,
            priority
        })
    }

    fn get_size_prediction(&self) -> usize {
        size_of::<String>()
        + size_of::<Vec<Vec<RecipeIngredient>>>()
        + size_of::<Vec<NetworkItemInstanceDescriptor>>()
        + size_of::<Uuid>()
        + size_of::<String>()
        + size_of::<i32>()
    } // TODO
}