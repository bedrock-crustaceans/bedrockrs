use crate::connection::ConnectionShard;
use crate::error::LoginError;
use crate::gamepacket::GamePacket;
use crate::login::provider::{LoginProviderClient, LoginProviderServer, LoginProviderStatus};
use crate::login::provider::packs::LoginProviderPacks;
use crate::packets::resource_packs_info::ResourcePacksInfoPacket;
use crate::packets::resource_packs_stack::ResourcePacksStackPacket;
use crate::types::base_game_version::BaseGameVersion;
use crate::types::experiments::Experiments;

pub async fn packs(
    conn: &mut ConnectionShard,
    provider: &mut impl LoginProviderServer,
) -> Result<(), LoginError> {
    match provider.packs() {
        LoginProviderPacks::CDN {
            behavior_packs,
            resource_packs,
            cdn_urls,
        } => {
            //////////////////////////////////////
            // Resource Packs Info Packet
            //////////////////////////////////////

            // TODO impl this
            let mut resource_packs_info = ResourcePacksInfoPacket {
                resource_pack_required: false,
                has_addon_packs: false,
                has_scripts: false,
                force_server_packs_enabled: false,
                behavior_packs: vec![],
                resource_packs: vec![],
                cdn_urls: cdn_urls.clone(),
            };

            match provider.on_resource_packs_info_pk(&mut resource_packs_info) {
                LoginProviderStatus::ContinueLogin => {}
                LoginProviderStatus::AbortLogin { reason } => {
                    return Err(LoginError::Abort { reason });
                }
            };

            match conn
                .send(GamePacket::ResourcePacksInfo(resource_packs_info))
                .await
            {
                Ok(_) => {}
                Err(e) => return Err(LoginError::ConnectionError(e)),
            };

            match conn.flush().await {
                Ok(_) => {}
                Err(e) => return Err(LoginError::ConnectionError(e)),
            };

            //////////////////////////////////////
            // Resource Pack Client Response
            // (/Client Cache Status Packet)
            //////////////////////////////////////

            match conn.recv().await {
                Ok(GamePacket::ClientCacheStatus(mut client_cache_status)) => {
                    match provider.on_client_cache_status_pk(&mut client_cache_status) {
                        LoginProviderStatus::ContinueLogin => {}
                        LoginProviderStatus::AbortLogin { reason } => {
                            return Err(LoginError::Abort { reason });
                        }
                    };

                    if let Err(e) = conn.set_cache_supported(client_cache_status.cache_supported).await {
                        return Err(LoginError::ConnectionError(e));
                    }

                    match conn.recv().await {
                        Ok(GamePacket::ResourcePackClientResponse(mut resource_pack_client_response)) => {
                            match provider.on_resource_packs_response_pk(&mut resource_pack_client_response) {
                                LoginProviderStatus::ContinueLogin => {}
                                LoginProviderStatus::AbortLogin { reason } => {
                                    return Err(LoginError::Abort { reason });
                                }
                            };
                        }
                        Ok(other) => {
                            return Err(LoginError::FormatError(format!(
                                "Expected ClientCacheStatus or ResourcePackClientResponse packet, got: {other:?}"
                            )))
                        }
                        Err(e) => { return Err(LoginError::ConnectionError(e)) }
                    }
                }
                Ok(GamePacket::ResourcePackClientResponse(mut resource_pack_client_response)) => {
                    match provider.on_resource_packs_response_pk(&mut resource_pack_client_response) {
                        LoginProviderStatus::ContinueLogin => {}
                        LoginProviderStatus::AbortLogin { reason } => {
                            return Err(LoginError::Abort { reason });
                        }
                    };
                }
                Ok(other) => {
                    return Err(LoginError::FormatError(format!(
                        "Expected ClientCacheStatus or ResourcePackClientResponse packet, got: {other:?}"
                    )))
                }
                Err(e) => { return Err(LoginError::ConnectionError(e)) }
            }

            //////////////////////////////////////
            // Resource Packs Stack Packet
            //////////////////////////////////////

            // TODO impl this
            let mut resource_packs_stack = ResourcePacksStackPacket {
                texture_pack_required: false,
                addons: vec![],
                texture_packs: vec![],
                base_game_version: BaseGameVersion(String::from("1.0")),
                experiments: Experiments {
                    experiments: vec![],
                    ever_toggled: false,
                },
                include_editor_packs: false,
            };

            match provider.on_resource_packs_stack_pk(&mut resource_packs_stack) {
                LoginProviderStatus::ContinueLogin => {}
                LoginProviderStatus::AbortLogin { reason } => {
                    return Err(LoginError::Abort { reason });
                }
            };

            match conn
                .send(GamePacket::ResourcePackStack(resource_packs_stack))
                .await
            {
                Ok(_) => {}
                Err(e) => return Err(LoginError::ConnectionError(e)),
            };

            match conn.flush().await {
                Ok(_) => {}
                Err(e) => return Err(LoginError::ConnectionError(e)),
            };

            //////////////////////////////////////
            // Resource Pack Client Response
            //////////////////////////////////////

            match conn.recv().await {
                Ok(GamePacket::ResourcePackClientResponse(mut resource_pack_client_response)) => {
                    match provider.on_resource_packs_response_pk(&mut resource_pack_client_response)
                    {
                        LoginProviderStatus::ContinueLogin => {}
                        LoginProviderStatus::AbortLogin { reason } => {
                            return Err(LoginError::Abort { reason });
                        }
                    };
                }
                Ok(other) => {
                    return Err(LoginError::FormatError(format!(
                        "Expected ResourcePackClientResponse packet, got: {other:?}"
                    )))
                }
                Err(e) => return Err(LoginError::ConnectionError(e)),
            }
        }
        LoginProviderPacks::DirectNetworkTransfer { .. } => {
            todo!("impl LoginProviderbedrockrs_addons::DirectNetworkTransfer in login process")
        }
    };

    Ok(())
}
