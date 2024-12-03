use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::enums::CommandPermissionLevel;

#[derive(ProtoCodec)]
struct EnumDataEntry {
    name: String,
    values: Vec<u32>, // TODO: weird varint. Needs custom proto
}

#[derive(ProtoCodec)]
struct SubCommandValues {
    #[endianness(le)]
    pub sub_command_first_value: u16,
    #[endianness(le)]
    pub sub_command_second_value: u16,
}

#[derive(ProtoCodec)]
struct ParameterDataEntry {
    pub name: String,
    #[endianness(le)]
    pub parse_symbol: u32,
    pub is_optional: bool,
    pub options: i8,
}

#[derive(ProtoCodec)]
struct OverloadsEntry {
    pub is_chaining: bool,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub parameter_data: Vec<ParameterDataEntry>,
}

#[derive(ProtoCodec)]
struct CommandsEntry {
    pub name: String,
    pub description: String,
    #[endianness(le)]
    pub flags: u16,
    pub permission_level: CommandPermissionLevel,
    #[endianness(le)]
    pub alias_enum: i32,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub chained_sub_command_indices: Vec<u16>, // TODO: custom proto impl
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub overloads: Vec<OverloadsEntry>,
}

#[derive(ProtoCodec)]
struct SoftEnumsEntry {
    pub enum_name: String,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub enum_options: Vec<String>,
}

#[derive(ProtoCodec)]
struct ConstraintsEntry {
    #[endianness(le)]
    pub enum_value_symbol: u32,
    #[endianness(le)]
    pub enum_symbol: u32,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub constraint_indices: Vec<i8>
}

#[gamepacket(id = 76)]
#[derive(ProtoCodec)]
pub struct AvailableCommandsPacket {
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub enum_values: Vec<String>,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub post_fixes: Vec<String>,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub enum_data: Vec<EnumDataEntry>,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub chained_sub_command_data: Vec<Vec<SubCommandValues>>,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub commands: Vec<Vec<SubCommandValues>>,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub soft_enums: Vec<SoftEnumsEntry>,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub constraints: Vec<ConstraintsEntry>,
}

// TODO: custom proto impl