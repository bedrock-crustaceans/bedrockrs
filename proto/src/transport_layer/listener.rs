pub enum TransportLaterListener {
    RaknetUDP(rak_rs::Listener),
    NetherNet(/* TODO */)
}