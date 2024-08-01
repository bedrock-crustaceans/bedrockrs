use bedrockrs_core::int::LE;

#[derive(Debug, Clone)]
pub struct UseItemTransactionData {
    /// legacy_request_id is an ID that is only non-zero at times when sent by the client. The server should
    /// always send 0 for this. When this field is not 0, the LegacySetItemSlots slice below will have values
    /// in it.
    /// legacy_request_id ties in with the ItemStackResponse packet. If this field is non-0, the server should
    /// respond with an ItemStackResponse packet. Some inventory actions such as dropping an item out of the
    /// hotbar are still one using this packet, and the ItemStackResponse packet needs to tie in with it.
    legacy_request_id: LE<i32>
}
