pub mod Social {
    use bedrockrs_macros::ProtoCodec;
    
    #[derive(ProtoCodec)]
    pub enum GamePublishSetting {
        NoMultiPlay = 0,
        InviteOnly = 1,
        FriendsOnly = 2,
        FriendsOfFriends = 3,
        Public = 4,
    }
}