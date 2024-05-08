use crate::error::SerilizationError;

/// MCProtoSerialize is a trait that needs to be implemented
/// for all types that should be serializable into bytes for
/// the MCBE protocol.
/// ### Example:
/// ```
/// use byteorder::WriteBytesExt;
/// use serialize::ser::MCProtoSerialize;
///
/// struct myType {
///     my_data: i32,
/// }
///
/// impl MCProtoSerialize for myType {
///     fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), SerilizationError> where Self: Sized {
///         // use the byteorder to simplu write it to the buffer
///         // this error should be actually handled
///         buf.write_i32(*self).unwrap();
///         Ok(())
///     }
/// }
/// ```
///
pub trait MCProtoSerialize {
    /// proto_serialize is a trait function that has the own type and
    /// a buffer that the type should serialize itself into.
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), SerilizationError>
    where
        Self: Sized;
}
